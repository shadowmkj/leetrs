## [unreleased](https://github.com/shadowmkj/leetrs)

### 🚀 Features

- *(cookie-extract)* Introduce cross-platform cookie extraction crate - ([9b7bb10](https://github.com/shadowmkj/leetrs/commit/9b7bb10768f8f6ccc91e0f1abf3d20d7e7453df1)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(cookie-extract)* Enhance linux encryption key extraction and profile discovery - ([5a1cbea](https://github.com/shadowmkj/leetrs/commit/5a1cbeacd5da4dca0ce18c5da11c533d97673db3)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(cookie-extract)* Expand firefox profile discovery for snaps, flatpaks, and forks - ([1daf0a0](https://github.com/shadowmkj/leetrs/commit/1daf0a05c1d91cbeb47402ce813c7419e4a65358)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(cookie-extract)* Support multiple linux keyring providers and key candidates - ([67ef989](https://github.com/shadowmkj/leetrs/commit/67ef98971f012c88a72b10fc6dbbcdfbca42f797)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(logging)* Add verbose diagnostic logging with env_logger - ([e7452c3](https://github.com/shadowmkj/leetrs/commit/e7452c366badda298c546d35f934279b439dd2d2)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(auth)* Add debug logging for extracted cookies - ([57a8d17](https://github.com/shadowmkj/leetrs/commit/57a8d17a0d96e193620f1dea96ffe6151f078f85)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(cookie-extract)* Improve Windows support and sqlite wal handling - ([370440d](https://github.com/shadowmkj/leetrs/commit/370440d89f47a6ffe59526751ba7fa02bc39b9d7)) (from [`@shadowmkj`](https://github.com/shadowmkj))

### 🐛 Bug Fixes

- *(client)* Use questionFrontendId for problem metadata & optimize topic attachment ([#32](https://github.com/shadowmkj/leetrs/pull/32)) - ([f3f5635](https://github.com/shadowmkj/leetrs/commit/f3f56357f37d7aba75f27af462cc090387f8c6e4)) (from [`@shadowmkj`](https://github.com/shadowmkj))

### ⚙️ Misc

- *(release)* Generate changelog and update cliff.toml - ([ca8969c](https://github.com/shadowmkj/leetrs/commit/ca8969c51776592f6952afba3f9ec7c09b4063e5)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(changelog)* Update changelog and git-cliff configuration - ([36f4fda](https://github.com/shadowmkj/leetrs/commit/36f4fdacaad0aa335d69c1e51795d4edf5dd15ca)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(changelog)* Update git-cliff configuration and regenerate changelog - ([5502a96](https://github.com/shadowmkj/leetrs/commit/5502a96a6a2f6c5e24f7267cf52f856d4cca1da4)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Update contributing guide and pull request template for xtask ([#33](https://github.com/shadowmkj/leetrs/pull/33)) - ([9f73922](https://github.com/shadowmkj/leetrs/commit/9f739224ef19cf460ab63beff77629571cff8f4a)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(cookie-extract,xtask)* Simplify nested conditions and add cross-target clippy support - ([364cb46](https://github.com/shadowmkj/leetrs/commit/364cb462f98ec50f537b7cf01fc048f05544c659)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(cookie-extract)* Add deterministic v10 decryption round-trip test - ([f35dec1](https://github.com/shadowmkj/leetrs/commit/f35dec18047ef391b4b4889ecce909a37d547f3c)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Update authentication documentation for expanded browser support and manual cURL parsing - ([e2549d0](https://github.com/shadowmkj/leetrs/commit/e2549d010a6737c323549da3272430d77fef1555)) (from [`@shadowmkj`](https://github.com/shadowmkj))
## [v1.1.0](https://github.com/shadowmkj/leetrs/releases/tag/v1.1.0) - 2026-08-19

### 🚀 Features

- Add config toml file - ([790a767](https://github.com/shadowmkj/leetrs/commit/790a767c325ba0c97270a91bf6893f408bd1b3bc)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Topic wise filtering ([#8](https://github.com/shadowmkj/leetrs/pull/8)) - ([3eb3435](https://github.com/shadowmkj/leetrs/commit/3eb3435d50102d9524593c1bed270e5ed71216df)) (from [`@deepansh11`](https://github.com/deepansh11))
- *(config)* Auto-create default config.toml if missing - ([538132a](https://github.com/shadowmkj/leetrs/commit/538132a18cf47e756e24a7cb958d8816023508cb)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(docs)* Initialize Docusaurus site structure ([#18](https://github.com/shadowmkj/leetrs/pull/18)) - ([23275a1](https://github.com/shadowmkj/leetrs/commit/23275a17855fd47a9afbafec393f6036119abfa8)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(tui)* Add Ctrl+d and Ctrl+u keybindings in topic filter mode - ([18aafcb](https://github.com/shadowmkj/leetrs/commit/18aafcbae0b010fdadaa9220f16bb364b795a8b8)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(tui)* Add topic search with modal input modes - ([62bc2b6](https://github.com/shadowmkj/leetrs/commit/62bc2b6ea7829c3769d1c6a0e5a09388e27b9eb9)) (from [`@shadowmkj`](https://github.com/shadowmkj))

### 🐛 Bug Fixes

- *(picker)* Return network error when offline instead of panicking - ([d0f249e](https://github.com/shadowmkj/leetrs/commit/d0f249e124ad090be776db48ae6fe0561161b9b1)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(tui)* Clear screen on enter and route pick errors to TUI alert popup - ([d2e6ee9](https://github.com/shadowmkj/leetrs/commit/d2e6ee93a8b5fcd442f4edf163591175c58861a0)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(docs)* Update logo image path and format homepage source - ([644609c](https://github.com/shadowmkj/leetrs/commit/644609c21a704e83db8a32475f47f7bd545e184b)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(ci)* Add environment setting for production in workflow - ([588db12](https://github.com/shadowmkj/leetrs/commit/588db12e708b9fb412bc44bf3d93f6adc1a395be)) (from [`@shadowmkj`](https://github.com/shadowmkj))

### ⚡️ Performance

- *(tui)* Load topic filter list from embedded topics.txt - ([1dd179c](https://github.com/shadowmkj/leetrs/commit/1dd179c04337c7da558fb940382fe32e3318a498)) (from [`@shadowmkj`](https://github.com/shadowmkj))

### 👷 CI

- *(ci)* Add code coverage workflow and git-cliff configuration - ([de7dad0](https://github.com/shadowmkj/leetrs/commit/de7dad094785659784fa4a8e0a9037eb252e4e93)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(ci)* Add cargo xtask automation and github actions workflows - ([e603a0c](https://github.com/shadowmkj/leetrs/commit/e603a0c519ac8beb0193d367b6c2108ece6dcfe2)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(ci)* Add dependabot configuration for automated updates - ([572dee3](https://github.com/shadowmkj/leetrs/commit/572dee3cb45b123a852f2d61d72769fab31909a0)) (from [`@shadowmkj`](https://github.com/shadowmkj))

### ⚙️ Misc

- Added documentation for the entire codebase - ([2d87c17](https://github.com/shadowmkj/leetrs/commit/2d87c1712c9f60893cd116a852ba836a0e7472e0)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Dynamic editor support, let-chains, and performance cleanups - ([7f147b0](https://github.com/shadowmkj/leetrs/commit/7f147b0e46a77564f9abb7bbbd49b1f58708ebd3)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(models)* Split models.rs into submodules and extract language helpers - ([9d72686](https://github.com/shadowmkj/leetrs/commit/9d72686c74d3fcd2ca58bf1c931909eb4e5fa92f)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(cache)* Introduce CacheService and consolidate project directory lookups - ([2df014e](https://github.com/shadowmkj/leetrs/commit/2df014e3db29518c75ee3fd2854d4b22057bb807)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(client)* Implement LeetCodeApi trait and generic status polling - ([2f75015](https://github.com/shadowmkj/leetrs/commit/2f7501515509e0ee5b5fc3fa6b006eb27ba6d541)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(cli)* Extract command handlers from main to commands module - ([b3c784b](https://github.com/shadowmkj/leetrs/commit/b3c784b276a8552c92c3dfd785fde607886b4916)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(services)* Introduce SubmissionService and format result output - ([37afdd3](https://github.com/shadowmkj/leetrs/commit/37afdd35047cc180c9447d9ba0430619b7cb09b7)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(tui)* Decompose selection screen into sub-widgets and add RAII terminal guard - ([e7f1869](https://github.com/shadowmkj/leetrs/commit/e7f18696e2d7db5ee085e586f877b9c035612d2b)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(config)* Register new library modules and update gitignore - ([c523bd1](https://github.com/shadowmkj/leetrs/commit/c523bd1d89e051fd7373e83a4ee878215c07c2b0)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Update README with TUI features and quick start guidelines - ([df8ff29](https://github.com/shadowmkj/leetrs/commit/df8ff296b07a02c97dda605337e90393656e1a3c)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(readme)* Expand documentation, add table of contents and typography refinements - ([b3d3b42](https://github.com/shadowmkj/leetrs/commit/b3d3b42b5a98c02e37ef6e05c86049df64e3805e)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Add a warning (in development) to readme - ([6413374](https://github.com/shadowmkj/leetrs/commit/6413374a9f3f907ccbc837654bf09ab55b07e33b)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Update documentation, add architectural decision records, and project assets - ([d9a5a79](https://github.com/shadowmkj/leetrs/commit/d9a5a795dc3ecbe109eda186d5c35635dd5b7217)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Address clippy lints and clean up code - ([e9c34b1](https://github.com/shadowmkj/leetrs/commit/e9c34b1844fdfa0e592974bd34fb5441839f9a1a)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Fix markdown formatting and syntax across documentation - ([d64c594](https://github.com/shadowmkj/leetrs/commit/d64c59431659c43870c4e1e59b475ba1cd310f13)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(readme)* Add built with ratatui badge - ([a7c875b](https://github.com/shadowmkj/leetrs/commit/a7c875b5fb10f0817d4b0cfd37bd563464cd4aa7)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(refactor)* Clean up and remove redundant unit tests - ([b3519b3](https://github.com/shadowmkj/leetrs/commit/b3519b3bdddf143edda6cecf9160b9417ca16150)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(dependabot)* Group patch updates and tui crates - ([9963a57](https://github.com/shadowmkj/leetrs/commit/9963a57f6c4e7fab79efa10a828846b84462fb90)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(cargo)* Format TOML configuration files - ([c0956c8](https://github.com/shadowmkj/leetrs/commit/c0956c84c765317f543c6170e28b6bb55a9b5074)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(release)* Bump version to 1.1.0 - ([0aea802](https://github.com/shadowmkj/leetrs/commit/0aea8028c8401ea915dc268d861f4f63afb22339)) (from [`@shadowmkj`](https://github.com/shadowmkj))

### 👥 New Contributors

- [`@shadowmkj`](https://github.com/shadowmkj) made their first contribution

- [`@dependabot[bot]`](https://github.com/dependabot[bot]) made their first contribution in [#31](https://github.com/shadowmkj/leetrs/pull/31)

- [`@deepansh11`](https://github.com/deepansh11) made their first contribution in [#8](https://github.com/shadowmkj/leetrs/pull/8)

- [`@SykikXO`](https://github.com/SykikXO) made their first contribution in [#5](https://github.com/shadowmkj/leetrs/pull/5)

- [`@spencer-hann`](https://github.com/spencer-hann) made their first contribution in [#4](https://github.com/shadowmkj/leetrs/pull/4)

- [`@meghakj4`](https://github.com/meghakj4) made their first contribution in [#3](https://github.com/shadowmkj/leetrs/pull/3)

