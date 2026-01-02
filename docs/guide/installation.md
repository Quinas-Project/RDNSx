# Installation Guide

## Prerequisites

### Rust Installation

RDNSx requires Rust 1.70.0 or later. Install Rust using [rustup](https://rustup.rs/):

**Windows:**
```powershell
# Download and run rustup-init.exe
# Or use winget:
winget install --id Rustlang.Rustup
```

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Verify Installation:**
```bash
rustc --version
cargo --version
```

### System Dependencies

#### Windows
Install Visual Studio Build Tools with C++ support:
```powershell
# Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
# Or use winget:
winget install Microsoft.VisualStudio.2022.BuildTools --override "--passive --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

#### Linux (Ubuntu/Debian)
```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

#### macOS
```bash
# Xcode Command Line Tools
xcode-select --install

# Or using Homebrew:
brew install openssl pkg-config
```

## Installation Methods

### Method 1: Cargo Install (Recommended)

```bash
cargo install rdnsx
```

This installs the latest stable version from crates.io.

### Method 2: Build from Source

```bash
# Clone the repository
git clone https://github.com/Quinas-Project/RDNSx.git
cd RDNSx

# Build in release mode
cargo build --release

# The binary will be available at: target/release/rdnsx
```

### Method 3: Development Setup

```bash
# Clone with submodules
git clone https://github.com/Quinas-Project/RDNSx.git
cd RDNSx

# Run tests
cargo test

# Run development build
cargo build

# Run with example
cargo run -- query example.com
```

## Verification

After installation, verify RDNSx is working:

```bash
# Check version
rdnsx --version

# Test basic functionality
rdnsx query example.com --a

# View help
rdnsx --help
```

## Configuration

### Environment Variables

RDNSx supports configuration via environment variables:

```bash
# Set default resolvers
export RDNSX_RESOLVERS="8.8.8.8,1.1.1.1"

# Set concurrency level
export RDNSX_CONCURRENCY=100

# Set timeout
export RDNSX_TIMEOUT=5
```

### Configuration File

Create a configuration file at `~/.rdnsx/config.toml`:

```toml
[default]
resolvers = ["8.8.8.8", "1.1.1.1"]
concurrency = 100
timeout = 5

[export]
batch_size = 1000
```

## Troubleshooting

### Common Issues

#### "cargo install failed"
- Ensure you have a stable internet connection
- Try `cargo install --verbose rdnsx` for detailed error messages

#### "linking with link.exe failed" (Windows)
- Install Visual Studio Build Tools with C++ workload
- Restart your terminal/command prompt after installation

#### "Permission denied" (Linux/macOS)
- Try running with `sudo` or check file permissions
- For system-wide installation: `sudo cargo install rdnsx`

#### "SSL/TLS errors"
- Ensure OpenSSL development libraries are installed
- On Ubuntu: `sudo apt install libssl-dev`

### Getting Help

If you encounter issues:

1. Check the [troubleshooting guide](./troubleshooting.md)
2. Search existing [GitHub issues](https://github.com/Quinas-Project/RDNSx/issues)
3. Create a new issue with:
   - Your operating system and version
   - Rust version (`rustc --version`)
   - Complete error output
   - Steps to reproduce

## Next Steps

Once installed, proceed to the [Quick Start Guide](./quick-start.md) to learn basic usage.