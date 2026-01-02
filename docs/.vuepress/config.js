module.exports = {
  title: 'RDNSx',
  description: 'Fast and multi-purpose DNS toolkit',
  base: '/RDNSx/',
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/guide/' },
      { text: 'API', link: '/api/' },
      { text: 'GitHub', link: 'https://github.com/Quinas-Project/RDNSx' }
    ],
    sidebar: {
      '/guide/': [
        '',
        'installation',
        'quick-start',
        'dns-records',
        'querying',
        'exports',
        'advanced-usage',
        'troubleshooting'
      ],
      '/api/': [
        '',
        'cli-reference',
        'library-api',
        'examples'
      ]
    },
    lastUpdated: 'Last Updated'
  },
  plugins: [
    ['@vuepress/back-to-top', true],
    ['@vuepress/medium-zoom', true],
    ['@vuepress/google-analytics', {
      ga: 'UA-XXXXXXXXX-X' // Replace with actual GA ID if needed
    }]
  ]
}