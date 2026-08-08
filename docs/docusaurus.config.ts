import { themes as prismThemes } from 'prism-react-renderer';
import type { Config } from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';

const config: Config = {
    title: 'leetrs',
    tagline: 'A blazing-fast, Rust-powered CLI engine and TUI for solving LeetCode problems natively in Neovim.',
    favicon: 'img/leetrs-new.png',

    // Future flags
    future: {
        v4: true,
    },

    // Set the production url of your site here
    url: 'https://shadowmkj.github.io',
    // Set the /<baseUrl>/ pathname under which your site is served
    baseUrl: '/leetrs/',

    // GitHub pages deployment config.
    organizationName: 'shadowmkj',
    projectName: 'leetrs',
    trailingSlash: false,

    onBrokenLinks: 'throw',

    i18n: {
        defaultLocale: 'en',
        locales: ['en'],
    },

    presets: [
        [
            'classic',
            {
                docs: {
                    sidebarPath: './sidebars.ts',
                    editUrl: 'https://github.com/shadowmkj/leetrs/tree/main/docs/',
                },
                blog: false,
                theme: {
                    customCss: './src/css/custom.css',
                },
            } satisfies Preset.Options,
        ],
    ],

    plugins: [
        [
            require.resolve('@easyops-cn/docusaurus-search-local'),
            {
                hashed: true,
                language: ['en'],
                highlightSearchTermsOnTargetPage: true,
                explicitSearchResultPath: true,
                docsRouteBasePath: '/docs',
            },
        ],
    ],

    themeConfig: {
        image: 'img/leetrs-new.png',
        colorMode: {
            defaultMode: 'dark',
            respectPrefersColorScheme: true,
        },
        navbar: {
            title: 'leetrs',
            logo: {
                alt: 'leetrs logo',
                src: 'img/leetrs-new.png',
            },
            items: [
                {
                    type: 'docSidebar',
                    sidebarId: 'docsSidebar',
                    position: 'left',
                    label: 'Documentation',
                },
                {
                    to: '/docs/cli-reference/overview',
                    label: 'CLI Reference',
                    position: 'left',
                },
                {
                    to: '/docs/architecture/overview',
                    label: 'Architecture',
                    position: 'left',
                },
                {
                    href: 'https://github.com/shadowmkj/leetrs',
                    label: 'GitHub',
                    position: 'right',
                },
            ],
        },
        footer: {
            style: 'dark',
            links: [
                {
                    title: 'Documentation',
                    items: [
                        {
                            label: 'Overview',
                            to: '/docs/',
                        },
                        {
                            label: 'Quickstart Guide',
                            to: '/docs/getting-started/quickstart',
                        },
                        {
                            label: 'CLI Reference',
                            to: '/docs/cli-reference/overview',
                        },
                    ],
                },
                {
                    title: 'Deep Dive',
                    items: [
                        {
                            label: 'TUI & Keybindings',
                            to: '/docs/tui-guide/interactive-browser',
                        },
                        {
                            label: 'Neovim Workflow',
                            to: '/docs/tui-guide/neovim-workflow',
                        },
                        {
                            label: 'Architecture',
                            to: '/docs/architecture/overview',
                        },
                    ],
                },
                {
                    title: 'Community & Code',
                    items: [
                        {
                            label: 'GitHub Repository',
                            href: 'https://github.com/shadowmkj/leetrs',
                        },
                        {
                            label: 'Crates.io Package',
                            href: 'https://crates.io/crates/leetrs',
                        },
                        {
                            label: 'Issue Tracker',
                            href: 'https://github.com/shadowmkj/leetrs/issues',
                        },
                    ],
                },
            ],
            copyright: `Copyright © ${new Date().getFullYear()} shadowmkj. Built with Docusaurus.`,
        },
        prism: {
            theme: prismThemes.github,
            darkTheme: prismThemes.dracula,
            additionalLanguages: ['rust', 'python', 'sql', 'toml', 'bash', 'json', 'markdown'],
        },
    } satisfies Preset.ThemeConfig,
};

export default config;
