import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

/**
 * Sidebars configuration for leetrs documentation.
 */
const sidebars: SidebarsConfig = {
  docsSidebar: [
    'intro',
    {
      type: 'category',
      label: '🚀 Getting Started',
      collapsed: false,
      items: [
        'getting-started/quickstart',
        'getting-started/installation',
        'getting-started/configuration',
        'getting-started/authentication',
      ],
    },
    {
      type: 'category',
      label: '💻 CLI Command Reference',
      collapsed: false,
      items: [
        'cli-reference/overview',
        'cli-reference/auth-and-status',
        'cli-reference/tui',
        'cli-reference/pick',
        'cli-reference/test-and-submit',
        'cli-reference/completion',
      ],
    },
    {
      type: 'category',
      label: '⌨️ TUI & Neovim Guide',
      collapsed: false,
      items: [
        'tui-guide/interactive-browser',
        'tui-guide/neovim-workflow',
      ],
    },
    {
      type: 'category',
      label: '🏗️ Architecture & Internals',
      collapsed: true,
      items: [
        'architecture/overview',
        'architecture/api-and-graphql',
        'architecture/caching-and-storage',
        'architecture/submission-engine',
      ],
    },
    {
      type: 'category',
      label: '🌐 Guides & FAQ',
      collapsed: true,
      items: [
        'guides/supported-languages',
        'guides/troubleshooting',
      ],
    },
    {
      type: 'category',
      label: '🤝 Contributing',
      collapsed: true,
      items: [
        'contributing/setup',
      ],
    },
  ],
};

export default sidebars;
