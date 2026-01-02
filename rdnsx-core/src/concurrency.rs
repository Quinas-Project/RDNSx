//! Concurrent processing utilities for high-performance DNS scanning

use std::sync::Arc;
use std::time::{Duration, Instant};

use futures::stream::{self, StreamExt};
use tokio::sync::Semaphore;
use tokio::time::timeout;
use tracing::{debug, warn, info};

use crate::error::{DnsxError, Result};
use crate::types::{DnsRecord, RecordType};

/// Configuration for concurrent processing
#[derive(Debug, Clone)]
pub struct ConcurrencyConfig {
    /// Maximum concurrent requests
    pub max_concurrent: usize,
    /// Batch size for processing
    pub batch_size: usize,
    /// Request timeout
    pub timeout: Duration,
    /// Rate limit (requests per second, 0 = unlimited)
    pub rate_limit: u64,
}

impl Default for ConcurrencyConfig {
    fn default() -> Self {
        Self {
            max_concurrent: 100,
            batch_size: 1000,
            timeout: Duration::from_secs(5),
            rate_limit: 0,
        }
    }
}

/// Performance metrics for concurrent processing
#[derive(Debug, Default)]
pub struct ProcessingMetrics {
    pub total_domains: usize,
    pub successful_queries: usize,
    pub failed_queries: usize,
    pub total_query_time: Duration,
    pub average_query_time: Duration,
    pub queries_per_second: f64,
}

/// Concurrent DNS query processor
pub struct ConcurrentProcessor<T, F> {
    config: ConcurrencyConfig,
    semaphore: Arc<Semaphore>,
    query_fn: Arc<F>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, F> ConcurrentProcessor<T, F>
where
    T: Send + 'static,
    F: Fn(T) -> futures::future::BoxFuture<'static, Result<Vec<DnsRecord>>> + Send + Sync + 'static,
{
    /// Create a new concurrent processor
    pub fn new(config: ConcurrencyConfig, query_fn: F) -> Self {
        let semaphore = Arc::new(Semaphore::new(config.max_concurrent));
        Self {
            config,
            semaphore,
            query_fn: Arc::new(query_fn),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Process items concurrently with streaming
    pub async fn process_stream<I>(
        &self,
        items: I,
    ) -> Result<(Vec<DnsRecord>, ProcessingMetrics)>
    where
        I: Iterator<Item = T> + Send,
        T: Send + 'static,
    {
        let start_time = Instant::now();
        let mut all_records = Vec::new();
        let mut metrics = ProcessingMetrics::default();

        // Create rate limiter if needed
        let rate_limiter = if self.config.rate_limit > 0 {
            Some(RateLimiter::new(self.config.rate_limit))
        } else {
            None
        };

        // Process items in batches to manage memory
        let items_vec: Vec<T> = items.collect();
        metrics.total_domains = items_vec.len();

        let chunks = items_vec.chunks(self.config.batch_size);

        for chunk in chunks {
            debug!("Processing batch of {} items", chunk.len());

            let batch_start = Instant::now();
            let batch_records = self.process_batch(chunk, &rate_limiter).await?;
            let batch_time = batch_start.elapsed();

            all_records.extend(batch_records);
            metrics.total_query_time += batch_time;

            debug!("Batch completed in {:.2}s", batch_time.as_secs_f64());
        }

        // Calculate final metrics
        let total_time = start_time.elapsed();
        if metrics.total_domains > 0 {
            metrics.average_query_time = metrics.total_query_time / metrics.total_domains as u32;
        }
        if total_time.as_secs_f64() > 0.0 {
            metrics.queries_per_second = metrics.total_domains as f64 / total_time.as_secs_f64();
        }

        info!(
            "Processed {} domains in {:.2}s ({:.1} qps, avg: {:.1}ms)",
            metrics.total_domains,
            total_time.as_secs_f64(),
            metrics.queries_per_second,
            metrics.average_query_time.as_millis()
        );

        Ok((all_records, metrics))
    }

    /// Process a batch of items concurrently
    async fn process_batch(
        &self,
        items: &[T],
        rate_limiter: &Option<RateLimiter>,
    ) -> Result<Vec<DnsRecord>>
    where
        T: Clone + Send + 'static,
    {
        let futures = stream::iter(items.iter().cloned())
            .map(|item| {
                let semaphore = Arc::clone(&self.semaphore);
                let query_fn = Arc::clone(&self.query_fn);
                let rate_limiter = rate_limiter.clone();

                async move {
                    let _permit = semaphore.acquire().await.unwrap();

                    // Apply rate limiting if configured
                    if let Some(ref limiter) = rate_limiter {
                        limiter.wait().await;
                    }

                    // Execute query with timeout
                    let result = timeout(self.config.timeout, query_fn(item)).await;

                    match result {
                        Ok(Ok(records)) => Ok(records),
                        Ok(Err(e)) => {
                            warn!("Query failed: {}", e);
                            Ok(Vec::new()) // Return empty vec for failed queries
                        }
                        Err(_) => {
                            warn!("Query timed out");
                            Ok(Vec::new())
                        }
                    }
                }
            })
            .buffer_unordered(self.config.max_concurrent);

        let results: Vec<Result<Vec<DnsRecord>>> = futures.collect().await;
        let mut all_records = Vec::new();

        for result in results {
            match result {
                Ok(records) => {
                    all_records.extend(records);
                }
                Err(e) => {
                    warn!("Batch processing error: {}", e);
                }
            }
        }

        Ok(all_records)
    }
}

/// Rate limiter for controlling request frequency
#[derive(Clone)]
pub struct RateLimiter {
    interval: Duration,
    last_request: std::sync::Arc<std::sync::Mutex<Instant>>,
}

impl RateLimiter {
    /// Create a new rate limiter
    pub fn new(requests_per_second: u64) -> Self {
        let interval = if requests_per_second > 0 {
            Duration::from_micros(1_000_000 / requests_per_second)
        } else {
            Duration::from_micros(0)
        };

        Self {
            interval,
            last_request: std::sync::Arc::new(std::sync::Mutex::new(Instant::now())),
        }
    }

    /// Wait until the next request can be made
    pub async fn wait(&self) {
        if self.interval.as_micros() == 0 {
            return;
        }

        let mut last_request = self.last_request.lock().unwrap();
        let now = Instant::now();
        let elapsed = now.duration_since(*last_request);

        if elapsed < self.interval {
            let sleep_duration = self.interval - elapsed;
            tokio::time::sleep(sleep_duration).await;
        }

        *last_request = Instant::now();
    }
}

/// Stream-based domain reader for memory-efficient processing
pub struct DomainStreamer<R> {
    reader: R,
    buffer_size: usize,
}

impl<R: std::io::BufRead> DomainStreamer<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buffer_size: 8192, // 8KB buffer
        }
    }

    pub fn with_buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = size;
        self
    }

    /// Stream domains one by one without loading everything into memory
    pub fn stream_domains(self) -> impl Iterator<Item = Result<String>> {
        let mut lines = self.reader.lines();

        std::iter::from_fn(move || {
            match lines.next() {
                Some(Ok(line)) => {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with('#') {
                        // Skip empty lines and comments, continue to next
                        Some(Ok(String::new())) // Will be filtered out
                    } else {
                        Some(Ok(trimmed.to_string()))
                    }
                }
                Some(Err(e)) => Some(Err(DnsxError::Io(e))),
                None => None,
            }
        }).filter_map(|result| {
            match result {
                Ok(s) if s.is_empty() => None, // Filter out empty lines
                other => Some(other),
            }
        })
    }
}

/// Adaptive batch sizer based on performance metrics
pub struct AdaptiveBatchSizer {
    current_size: usize,
    min_size: usize,
    max_size: usize,
    target_qps: f64,
    adjustment_factor: f64,
}

impl AdaptiveBatchSizer {
    pub fn new(initial_size: usize, min_size: usize, max_size: usize) -> Self {
        Self {
            current_size: initial_size,
            min_size,
            max_size,
            target_qps: 1000.0, // Target 1000 queries per second
            adjustment_factor: 1.2, // Adjust by 20%
        }
    }

    /// Adjust batch size based on current performance
    pub fn adjust(&mut self, current_qps: f64) {
        if current_qps > self.target_qps * 1.1 {
            // Too fast, increase batch size
            self.current_size = ((self.current_size as f64 * self.adjustment_factor) as usize)
                .min(self.max_size);
        } else if current_qps < self.target_qps * 0.9 {
            // Too slow, decrease batch size
            self.current_size = ((self.current_size as f64 / self.adjustment_factor) as usize)
                .max(self.min_size);
        }
    }

    pub fn current_size(&self) -> usize {
        self.current_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = RateLimiter::new(10); // 10 requests per second
        let start = Instant::now();

        for _ in 0..5 {
            limiter.wait().await;
        }

        let elapsed = start.elapsed();
        // Should take at least 0.4 seconds for 5 requests at 10/s
        assert!(elapsed >= Duration::from_millis(400));
    }

    #[test]
    fn test_adaptive_batch_sizer() {
        let mut sizer = AdaptiveBatchSizer::new(100, 10, 1000);

        // Test increasing batch size (high QPS)
        sizer.adjust(1200.0);
        assert!(sizer.current_size() > 100);

        // Test decreasing batch size (low QPS)
        sizer.adjust(800.0);
        assert!(sizer.current_size() < sizer.current_size);
    }
}