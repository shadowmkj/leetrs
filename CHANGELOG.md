## [v1.2.2](https://github.com/shadowmkj/leetrs/releases/tag/v1.2.2) - 2026-11-09

### 🚀 Features

- *(cli)* Add custom test case input to leetrs test ([#66](https://github.com/shadowmkj/leetrs/pull/66)) - ([d649659](https://github.com/shadowmkj/leetrs/commit/d649659a6f76bdd813b1202dbf28aa476ea9e56c)) (from [`@costajohnt`](https://github.com/costajohnt))
- *(cli)* Add leetrs daily command for the Problem of the Day ([#69](https://github.com/shadowmkj/leetrs/pull/69)) - ([d0f957e](https://github.com/shadowmkj/leetrs/commit/d0f957ee497c7067c0d01809af1649129259ced5)) (from [`@costajohnt`](https://github.com/costajohnt))
- *(build)* Embed git commit metadata into build version - ([e6f55f0](https://github.com/shadowmkj/leetrs/commit/e6f55f05919f788bb97b7f560f6976b6ca0c492b)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(cli)* Add browser flag to open problems in web browser - ([f0c7d5a](https://github.com/shadowmkj/leetrs/commit/f0c7d5a40a61c9339a7bb0d3a4bc82166ea4fb77)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(config)* Support solutions_dir in config.toml ([#68](https://github.com/shadowmkj/leetrs/pull/68)) - ([6dcb560](https://github.com/shadowmkj/leetrs/commit/6dcb560102161490c9ada9d550019f1e43867939)) (from [`@costajohnt`](https://github.com/costajohnt))
- *(cli)* Add clean and reset commands to purge local cache and credentials - ([2114e6d](https://github.com/shadowmkj/leetrs/commit/2114e6d1ee74f6a22f4c2443a3203f6a3866a9e5)) (from [`@shadowmkj`](https://github.com/shadowmkj))

### 🐛 Bug Fixes

- *(completion)* Generate Elvish and PowerShell scripts instead of panicking ([#67](https://github.com/shadowmkj/leetrs/pull/67)) - ([8284786](https://github.com/shadowmkj/leetrs/commit/82847868b15e067eb3b710802610c1395585fa09)) (from [`@saimskywalker`](https://github.com/saimskywalker))
- *(client)* Add retry limits and terminal failure detection to polling loop ([#75](https://github.com/shadowmkj/leetrs/pull/75)) - ([9159cce](https://github.com/shadowmkj/leetrs/commit/9159cce4bee4b4af6cb9271bf5323e667008ed7b)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(picker)* Do not overwrite existing solution files when picking by ID or slug - ([261f884](https://github.com/shadowmkj/leetrs/commit/261f88444fc343b97ba4f1ce1728feaa900a61c9)) (from [`@shadowmkj`](https://github.com/shadowmkj))

### ⚡️ Performance

- *(cache)* Stream JSON in CacheService read and write ([#70](https://github.com/shadowmkj/leetrs/pull/70)) - ([f383684](https://github.com/shadowmkj/leetrs/commit/f383684a5f27b039653fb5e1a7bcbcb4017ac5cb)) (from [`@costajohnt`](https://github.com/costajohnt))

### ⚙️ Misc

- Update cli reference documentation and README - ([73c722d](https://github.com/shadowmkj/leetrs/commit/73c722d0001aa3f919b913aa8dd54dc3a310a02e)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Update git-cliff configuration for PR numbers and filtering - ([225fcd2](https://github.com/shadowmkj/leetrs/commit/225fcd2c612a74e6eee6376c8a95aa0daae36730)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(release)* Bump version to 1.2.2 - ([e24d7fb](https://github.com/shadowmkj/leetrs/commit/e24d7fb5a812caa46e1211518c7f1f64d4be6d9e)) (by Milan Pramod)

### 👥 New Contributors

- [`@costajohnt`](https://github.com/costajohnt) made their first contribution in [#66](https://github.com/shadowmkj/leetrs/pull/66)

- [`@saimskywalker`](https://github.com/saimskywalker) made their first contribution in [#67](https://github.com/shadowmkj/leetrs/pull/67)

## [v1.2.1](https://github.com/shadowmkj/leetrs/releases/tag/v1.2.1) - 2026-10-28

### 🚀 Features

- *(submission)* Add support for capturing and formatting stdout ([#58](https://github.com/shadowmkj/leetrs/pull/58)) - ([acf7a55](https://github.com/shadowmkj/leetrs/commit/acf7a55a8607981605a238bf30a9bfca58407767)) (from [`@shadowmkj`](https://github.com/shadowmkj))

### ⚙️ Misc

- Update submodule (cookie-extract) for crates.io - ([6f1e1a3](https://github.com/shadowmkj/leetrs/commit/6f1e1a32714f38b2b258b30cd78ed757295d41a9)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Remove work in progress warning from readme - ([c755e68](https://github.com/shadowmkj/leetrs/commit/c755e68f80d1f7f80c79fa46eed0bc34fdc8cdf6)) (from [`@shadowmkj`](https://github.com/shadowmkj))
## [v1.2.0](https://github.com/shadowmkj/leetrs/releases/tag/v1.2.0) - 2026-10-16

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
- *(changelog)* Update changelog and git-cliff configuration - ([cdd1e9d](https://github.com/shadowmkj/leetrs/commit/cdd1e9d7ba966718bc9439759e255a1874bd5a36)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- *(changelog)* Simplify author and contributor templates in cliff.toml - ([55cf87b](https://github.com/shadowmkj/leetrs/commit/55cf87bc4387fe02e89ff2fc692ee27b10c610fb)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Merge branch 'release/1.2.0-beta.1' - ([de435b3](https://github.com/shadowmkj/leetrs/commit/de435b3cecafbf78970f8866d6c32781e1d5b724)) (from [`@shadowmkj`](https://github.com/shadowmkj))
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

- Initial commit - ([73d73d0](https://github.com/shadowmkj/leetrs/commit/73d73d081c1452101781c03bbdb8913cfc31ffe3)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Initial GraphQL setup + Auth setup - ([063d703](https://github.com/shadowmkj/leetrs/commit/063d703d5f82d8ada3e8a94973d21f00fcfc67fc)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Basic code submission - ([b477ea5](https://github.com/shadowmkj/leetrs/commit/b477ea5e6287a4085301e1c696dc2837d8ae481b)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Support for python and rust snippets. Better submission and file naming. - ([00fbc17](https://github.com/shadowmkj/leetrs/commit/00fbc1775d947b3c7f1277499be8fced1b5ae4b5)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Add README - ([5d33ee9](https://github.com/shadowmkj/leetrs/commit/5d33ee94c3a44b43aebcf3edd0bd0e8b7cd3d56d)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Add autocompletions - ([d9fe423](https://github.com/shadowmkj/leetrs/commit/d9fe4237103270849b68ba66b4ab78880a96a7ac)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Better submissions + Code refactoring - ([8dd2d05](https://github.com/shadowmkj/leetrs/commit/8dd2d0585b26bd0b85c8208dda0ea3877201cf79)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Implement basic TUI using ratatui - ([69d89d4](https://github.com/shadowmkj/leetrs/commit/69d89d421ca01d541bd4a29dbce2df3221eae1f9)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Better error handling + Default language to python - ([a40fa27](https://github.com/shadowmkj/leetrs/commit/a40fa273d50c70218596df7f8203debd63a845d0)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Caching problems in TUI (made it 10x faster) - ([af78249](https://github.com/shadowmkj/leetrs/commit/af782496e71be6d302f518d5ea468861b635a52a)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Add runtime error handling - ([e963724](https://github.com/shadowmkj/leetrs/commit/e9637243eb81de4f20d34ae2bf8064836110edad)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Added search + basic difficulty filtering - ([27ad968](https://github.com/shadowmkj/leetrs/commit/27ad968e9d994f777e0ba3eaae7a09d0a72ec889)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Implemented continuity with TUI - ([8a67333](https://github.com/shadowmkj/leetrs/commit/8a67333bc0acfb1bb63733b1e921db8dd86fd62e)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Default behaviour of leetrs to TUI - ([94ec655](https://github.com/shadowmkj/leetrs/commit/94ec655993a2ef5d8482bf5329f46ffa4d908347)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Add simple instruction + minor UI updates - ([388e3c4](https://github.com/shadowmkj/leetrs/commit/388e3c4408c3710d15cc61bc59bbfa69ce4a6340)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Major Refactor + Add a help screen - ([025210c](https://github.com/shadowmkj/leetrs/commit/025210cada4fc8e63b11097bbd9a5091d2421afe)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Add a hint in selection screen for "?" - ([57ed465](https://github.com/shadowmkj/leetrs/commit/57ed465a612dce402fa5e89896e672ca4958d0d5)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Added test submission feature - ([2b2f62f](https://github.com/shadowmkj/leetrs/commit/2b2f62ffdfd0d9e04a78824ef984462cf2b5d024)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Add a completed mark for problems - ([a0d5fad](https://github.com/shadowmkj/leetrs/commit/a0d5fad078e14543db723cc641aeffd3c012dee1)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Performance Optimizations.

Lesser memory allocations (replaced clone() with Reference Counting)
Also use indices for filtering instead of cloning data. - ([1348cb9](https://github.com/shadowmkj/leetrs/commit/1348cb95ef27c9d687e23d680fbc0042f51134bd)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Added state persistance in TUI

After closing Neovim, the TUI state is not lost.
The search, filter states are preserved. - ([87a947b](https://github.com/shadowmkj/leetrs/commit/87a947b64794576423d465327f83b45bbe6b93d8)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Add support for SQL problems - ([854dbb5](https://github.com/shadowmkj/leetrs/commit/854dbb53ca5d1f007bdc87c807b0770f46190a1a)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Added topics to problems. TUI display also

It displays first topic in the TUI now - ([a7c96e2](https://github.com/shadowmkj/leetrs/commit/a7c96e2ebdcb919dce9df9a2ff722dcca07d91c2)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Fix Database problem submission - ([ec070fe](https://github.com/shadowmkj/leetrs/commit/ec070fe8663f01518c33886684d8bb6901ea9d3f)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Add user data + handle leetcode premium questions - ([0ce5cec](https://github.com/shadowmkj/leetrs/commit/0ce5cec42dca7e709a97bea56a00e02af154c482)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Add a keybind 'o' to open the problem in browser - ([b34af2b](https://github.com/shadowmkj/leetrs/commit/b34af2baa842fa9dfa028dd550951cefbb3a48d7)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Fix some errors - ([e78b5d9](https://github.com/shadowmkj/leetrs/commit/e78b5d9d5a25a47b4a1aa54da414a3d22dc845b1)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Add contributing guidelines to CONTRIBUTING.md

Added guidelines for contributing to the project, including ways to contribute, opening issues, and making contributions. - ([0ca676e](https://github.com/shadowmkj/leetrs/commit/0ca676eab120d0f137321ccc81a179f223bc3ba5)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Added documentation for the entire codebase - ([2d87c17](https://github.com/shadowmkj/leetrs/commit/2d87c1712c9f60893cd116a852ba836a0e7472e0)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Add a video demo - ([e65e8c7](https://github.com/shadowmkj/leetrs/commit/e65e8c7bc59005dd5c91362f1852b3f46262bb3c)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Merge pull request #3 from meghakj4/optimization

updated cargo.toml with recommended optimization - ([d6b5d0a](https://github.com/shadowmkj/leetrs/commit/d6b5d0acf3f2dbdbaaf71c0c3e8f86cc5578ecc3)) (from [`@meghakj4`](https://github.com/meghakj4))
- Allow language selection in tui command ([#4](https://github.com/shadowmkj/leetrs/pull/4)) - ([5541606](https://github.com/shadowmkj/leetrs/commit/5541606c7ffc6f1cc255d2645645900e7aa2685a)) (from [`@spencer-hann`](https://github.com/spencer-hann))
- Update gh workflow: only run release after pushing tag - ([3d05ee5](https://github.com/shadowmkj/leetrs/commit/3d05ee58576c089046b4354c4437a58c2be826ad)) (from [`@shadowmkj`](https://github.com/shadowmkj))
- Remove duplicate Esc binding entry in help screen ([#5](https://github.com/shadowmkj/leetrs/pull/5)) - ([aba52e9](https://github.com/shadowmkj/leetrs/commit/aba52e9f39e1185c6e9734741e426ad9334456b3)) (from [`@SykikXO`](https://github.com/SykikXO))
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

