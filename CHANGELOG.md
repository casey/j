Changelog
=========

[1.38.0](https://github.com/casey/just/releases/tag/1.38.0) - 2024-12-10
------------------------------------------------------------------------

### Added
- Add `[openbsd]` recipe attribute ([#2497](https://github.com/casey/just/pull/2497) by [vtamara](https://github.com/vtamara))
- Add `[working-directory]` recipe attribute ([#2438](https://github.com/casey/just/pull/2438) by [bcheidemann](https://github.com/bcheidemann))
- Add `--allow-missing` to ignore missing recipe and submodule errors ([#2460](https://github.com/casey/just/pull/2460) by [R3ZV](https://github.com/R3ZV))

### Changed
- Add snap package back to readme ([#2506](https://github.com/casey/just/pull/2506) by [casey](https://github.com/casey))
- Forbid duplicate non-repeatable attributes ([#2483](https://github.com/casey/just/pull/2483) by [casey](https://github.com/casey))

### Misc
- Publish docs to GitHub pages on release only ([#2516](https://github.com/casey/just/pull/2516) by [casey](https://github.com/casey))
- Note lack of support for string interpolation ([#2515](https://github.com/casey/just/pull/2515) by [casey](https://github.com/casey))
- Embolden help text errors ([#2502](https://github.com/casey/just/pull/2502) by [casey](https://github.com/casey))
- Style help text ([#2501](https://github.com/casey/just/pull/2501) by [casey](https://github.com/casey))
- Add `--request` subcommand for testing ([#2498](https://github.com/casey/just/pull/2498) by [casey](https://github.com/casey))
- [bin/forbid] Improve error message if ripgrep is missing ([#2493](https://github.com/casey/just/pull/2493) by [casey](https://github.com/casey))
- Fix Rust 1.83 clippy warnings ([#2487](https://github.com/casey/just/pull/2487) by [casey](https://github.com/casey))
- Refactor JSON tests ([#2484](https://github.com/casey/just/pull/2484) by [casey](https://github.com/casey))
- Get `Config` from `ExecutionContext` instead of passing separately ([#2481](https://github.com/casey/just/pull/2481) by [casey](https://github.com/casey))
- Don't write justfiles unchanged by formatting ([#2479](https://github.com/casey/just/pull/2479) by [casey](https://github.com/casey))

[1.37.0](https://github.com/casey/just/releases/tag/1.37.0) - 2024-11-20
------------------------------------------------------------------------

### Added
- Add `style()` function ([#2462](https://github.com/casey/just/pull/2462) by [casey](https://github.com/casey))
- Terminal escape sequence constants ([#2461](https://github.com/casey/just/pull/2461) by [casey](https://github.com/casey))
- Add `&&` and `||` operators ([#2444](https://github.com/casey/just/pull/2444) by [casey](https://github.com/casey))

### Changed
- Make recipe doc attribute override comment ([#2470](https://github.com/casey/just/pull/2470) by [casey](https://github.com/casey))
- Don't export constants ([#2449](https://github.com/casey/just/pull/2449) by [casey](https://github.com/casey))
- Allow duplicate imports ([#2437](https://github.com/casey/just/pull/2437) by [casey](https://github.com/casey))
- Publish single SHA256SUM file with releases ([#2417](https://github.com/casey/just/pull/2417) by [casey](https://github.com/casey))
- Mark recipes with private attribute as private in JSON dump ([#2415](https://github.com/casey/just/pull/2415) by [casey](https://github.com/casey))
- Forbid invalid attributes on assignments ([#2412](https://github.com/casey/just/pull/2412) by [casey](https://github.com/casey))

### Misc
- Update `softprops/action-gh-release` ([#2471](https://github.com/casey/just/pull/2471) by [app/dependabot](https://github.com/app/dependabot))
- Add `-g` to `rust-just` install instructions ([#2459](https://github.com/casey/just/pull/2459) by [gnpaone](https://github.com/gnpaone))
- Change doc backtick color to cyan ([#2469](https://github.com/casey/just/pull/2469) by [casey](https://github.com/casey))
- Note that `set shell` is not used for `[script]` recipes ([#2468](https://github.com/casey/just/pull/2468) by [iloveitaly](https://github.com/iloveitaly))
- Replace `derivative` with `derive-where` ([#2465](https://github.com/casey/just/pull/2465) by [laniakea64](https://github.com/laniakea64))
- Highlight backticks in docs when listing recipes ([#2423](https://github.com/casey/just/pull/2423) by [neunenak](https://github.com/neunenak))
- Update setup-just version in README ([#2456](https://github.com/casey/just/pull/2456) by [Julian](https://github.com/Julian))
- Fix shell function example in readme ([#2454](https://github.com/casey/just/pull/2454) by [casey](https://github.com/casey))
- Update softprops/action-gh-release ([#2450](https://github.com/casey/just/pull/2450) by [app/dependabot](https://github.com/app/dependabot))
- Use `justfile` instead of `mf` on invalid examples in readme ([#2447](https://github.com/casey/just/pull/2447) by [casey](https://github.com/casey))
- Add advice on printing complex strings ([#2446](https://github.com/casey/just/pull/2446) by [casey](https://github.com/casey))
- Document using functions in variable assignments ([#2431](https://github.com/casey/just/pull/2431) by [offby1](https://github.com/offby1))
- Use prettier string comparison in tests ([#2435](https://github.com/casey/just/pull/2435) by [neunenak](https://github.com/neunenak))
- Note `shell(…)` as an alternative to backticks ([#2430](https://github.com/casey/just/pull/2430) by [offby1](https://github.com/offby1))
- Update nix package links ([#2441](https://github.com/casey/just/pull/2441) by [yunusey](https://github.com/yunusey))
- Update README.中文.md ([#2424](https://github.com/casey/just/pull/2424) by [Jannchie](https://github.com/Jannchie))
- Add Recipe::subsequents ([#2428](https://github.com/casey/just/pull/2428) by [casey](https://github.com/casey))
- Add subsequents to grammar ([#2427](https://github.com/casey/just/pull/2427) by [casey](https://github.com/casey))
- Document checking releases hashes  ([#2418](https://github.com/casey/just/pull/2418) by [casey](https://github.com/casey))
- Show how to access positional arguments with powershell ([#2405](https://github.com/casey/just/pull/2405) by [casey](https://github.com/casey))
- Use `-CommandWithArgs` instead of `-cwa` ([#2404](https://github.com/casey/just/pull/2404) by [casey](https://github.com/casey))
- Document `-cwa` flag for PowerShell positional arguments ([#2403](https://github.com/casey/just/pull/2403) by [casey](https://github.com/casey))
- Use `unwrap_or` when creating relative path in loader ([#2400](https://github.com/casey/just/pull/2400) by [casey](https://github.com/casey))

[1.36.0](https://github.com/casey/just/releases/tag/1.36.0) - 2024-09-30
------------------------------------------------------------------------

### Changed
- Allow default values to use earlier recipe arguments ([#2382](https://github.com/casey/just/pull/2382) by [casey](https://github.com/casey))

### Added
- Add `--one` flag to forbid multiple recipes from being invoked on the command line ([#2374](https://github.com/casey/just/pull/2374) by [casey](https://github.com/casey))
- Allow including arbitrary characters in strings with `\u{…}` ([#2360](https://github.com/casey/just/pull/2360) by [laniakea64](https://github.com/laniakea64))
- Print recipe doc string when`--explain` flag  is passed ([#2319](https://github.com/casey/just/pull/2319) by [neunenak](https://github.com/neunenak))

### Misc
- Use unwrap_or_default() when getting default color and verbosity ([#2397](https://github.com/casey/just/pull/2397) by [casey](https://github.com/casey))
- De-duplicate suggestion methods ([#2392](https://github.com/casey/just/pull/2392) by [neunenak](https://github.com/neunenak))
- Refactor analyzer ([#2378](https://github.com/casey/just/pull/2378) by [neunenak](https://github.com/neunenak))
- Use `console` codeblocks in readme ([#2388](https://github.com/casey/just/pull/2388) by [casey](https://github.com/casey))
- Split packages table by platform ([#2385](https://github.com/casey/just/pull/2385) by [casey](https://github.com/casey))
- Document npm package ([#2384](https://github.com/casey/just/pull/2384) by [casey](https://github.com/casey))
- Add PyPI install instructions ([#2383](https://github.com/casey/just/pull/2383) by [casey](https://github.com/casey))
- Remove alias shadows recipe error ([#2375](https://github.com/casey/just/pull/2375) by [neunenak](https://github.com/neunenak))
- Name instead of number book chapter files ([#2372](https://github.com/casey/just/pull/2372) by [casey](https://github.com/casey))
- Add groups to project justfile ([#2351](https://github.com/casey/just/pull/2351) by [neunenak](https://github.com/neunenak))
- Document `\u{...}` ([#2371](https://github.com/casey/just/pull/2371) by [laniakea64](https://github.com/laniakea64))
- Remove old recipes from project justfile ([#2367](https://github.com/casey/just/pull/2367) by [casey](https://github.com/casey))
- Document `--dotenv-path` in readme ([#2366](https://github.com/casey/just/pull/2366) by [willie](https://github.com/willie))
- Remove ref-type crate ([#2364](https://github.com/casey/just/pull/2364) by [casey](https://github.com/casey))
- Fix type names in redefinition error message ([#2353](https://github.com/casey/just/pull/2353) by [marcaddeo](https://github.com/marcaddeo))
- Use relative in `.sha256sum` files ([#2358](https://github.com/casey/just/pull/2358) by [casey](https://github.com/casey))
- Link to readme in CONTRIBUTING.md ([#2348](https://github.com/casey/just/pull/2348) by [casey](https://github.com/casey))
- Fix clippy lints ([#2347](https://github.com/casey/just/pull/2347) by [casey](https://github.com/casey))
- Simplify `Subcommand::run` ([#2336](https://github.com/casey/just/pull/2336) by [neunenak](https://github.com/neunenak))
- Update module issue link in readme ([#2345](https://github.com/casey/just/pull/2345) by [casey](https://github.com/casey))
- Add blank line between CI workflow jobs ([#2343](https://github.com/casey/just/pull/2343) by [casey](https://github.com/casey))
- Color groups in `--list` output ([#2340](https://github.com/casey/just/pull/2340) by [casey](https://github.com/casey))
- Refactor and document subcommand and search ([#2335](https://github.com/casey/just/pull/2335) by [neunenak](https://github.com/neunenak))
- Document private variables ([#2331](https://github.com/casey/just/pull/2331) by [Jasha10](https://github.com/Jasha10))

[1.35.0](https://github.com/casey/just/releases/tag/1.35.0) - 2024-08-28
------------------------------------------------------------------------

### Changed
- Allow fallback with recipes in submodules ([#2329](https://github.com/casey/just/pull/2329) by [casey](https://github.com/casey))
- Allow `[private]` attribute on assignments ([#2300](https://github.com/casey/just/pull/2300) by [adsnaider](https://github.com/adsnaider))

### Misc
- Generate `.sha256sum` files for release artifacts ([#2323](https://github.com/casey/just/pull/2323) by [twm](https://github.com/twm))
- Clarify that subsequent dependencies run immediately after recipe ([#2326](https://github.com/casey/just/pull/2326) by [casey](https://github.com/casey))
- Fix readme typo ([#2321](https://github.com/casey/just/pull/2321) by [arminius-smh](https://github.com/arminius-smh))
- Remove Config::run ([#2320](https://github.com/casey/just/pull/2320) by [neunenak](https://github.com/neunenak))
- Bump MSRV to 1.74 ([#2306](https://github.com/casey/just/pull/2306) by [casey](https://github.com/casey))
- Remove logging ([#2305](https://github.com/casey/just/pull/2305) by [casey](https://github.com/casey))
- Group commands under dedicated heading in `--help` output ([#2302](https://github.com/casey/just/pull/2302) by [casey](https://github.com/casey))
- Fix readme typo ([#2297](https://github.com/casey/just/pull/2297) by [nyurik](https://github.com/nyurik))

[1.34.0](https://github.com/casey/just/releases/tag/1.34.0) - 2024-08-02
------------------------------------------------------------------------

### Fixed
- Make function paths relative to correct working directory ([#2294](https://github.com/casey/just/pull/2294) by [casey](https://github.com/casey))

### Changed
- Keep multi-line shebangs together ([#2276](https://github.com/casey/just/pull/2276) by [vkstrm](https://github.com/vkstrm))

### Misc
- Document `set working-directory` ([#2288](https://github.com/casey/just/pull/2288) by [nyurik](https://github.com/nyurik))
- Fix readme typos ([#2289](https://github.com/casey/just/pull/2289) by [casey](https://github.com/casey))

[1.33.0](https://github.com/casey/just/releases/tag/1.33.0) - 2024-07-30
------------------------------------------------------------------------

### Fixed
- Use correct backtick and `shell()` expression working directory in submodules ([#2285](https://github.com/casey/just/pull/2285) by [casey](https://github.com/casey))

### Added
- Add `working-directory` setting ([#2283](https://github.com/casey/just/pull/2283) by [nyurik](https://github.com/nyurik))
- Allow `[group]` attribute on submodules ([#2263](https://github.com/casey/just/pull/2263) by [jmwoliver](https://github.com/jmwoliver))
- Allow empty `[script]` attribute and add `set script-interpreter` ([#2264](https://github.com/casey/just/pull/2264) by [casey](https://github.com/casey))

### Misc
- Document which attributes apply to which items ([#2282](https://github.com/casey/just/pull/2282) by [casey](https://github.com/casey))
- Add missing productions ([#2280](https://github.com/casey/just/pull/2280) by [poliorcetics](https://github.com/poliorcetics))
- Fix Rust 1.80.0 warnings ([#2281](https://github.com/casey/just/pull/2281) by [casey](https://github.com/casey))
- Update softprops/action-gh-release ([#2269](https://github.com/casey/just/pull/2269) by [app/dependabot](https://github.com/app/dependabot))
- Remove `(no group)` header before ungrouped recipes ([#2268](https://github.com/casey/just/pull/2268) by [casey](https://github.com/casey))
- Document `script-interpreter` setting ([#2265](https://github.com/casey/just/pull/2265) by [casey](https://github.com/casey))
- `set dotenv-path` does not override `set dotenv-filename` ([#2262](https://github.com/casey/just/pull/2262) by [casey](https://github.com/casey))

[1.32.0](https://github.com/casey/just/releases/tag/1.32.0) - 2024-07-17
------------------------------------------------------------------------

### Added
- Add unstable `[script(…)]` attribute ([#2259](https://github.com/casey/just/pull/2259) by [casey](https://github.com/casey))
- Add `[extension: 'EXT']` attribute to set shebang recipe script file extension ([#2256](https://github.com/casey/just/pull/2256) by [casey](https://github.com/casey))
- Suppress mod doc comment with empty `[doc]` attribute ([#2254](https://github.com/casey/just/pull/2254) by [casey](https://github.com/casey))
- Allow `[doc]` annotation on modules ([#2247](https://github.com/casey/just/pull/2247) by [neunenak](https://github.com/neunenak))

[1.31.0](https://github.com/casey/just/releases/tag/1.31.0) - 2024-07-14
------------------------------------------------------------------------

### Stabilized
- Stabilize modules ([#2250](https://github.com/casey/just/pull/2250) by [casey](https://github.com/casey))

### Added
- Allow `mod` path to be directory containing module source ([#2238](https://github.com/casey/just/pull/2238) by [casey](https://github.com/casey))
- Allow enabling unstable features with `set unstable` ([#2237](https://github.com/casey/just/pull/2237) by [casey](https://github.com/casey))
- Allow abbreviating functions ending in `_directory` to `_dir` ([#2235](https://github.com/casey/just/pull/2235) by [casey](https://github.com/casey))

### Fixed
- Lexiclean search directory so `..` does not check the current directory ([#2236](https://github.com/casey/just/pull/2236) by [casey](https://github.com/casey))

### Misc
- Print space before submodules in `--list` with groups ([#2244](https://github.com/casey/just/pull/2244) by [casey](https://github.com/casey))

[1.30.1](https://github.com/casey/just/releases/tag/1.30.1) - 2024-07-06
------------------------------------------------------------------------

### Fixed
- Fix function argument count mismatch error message ([#2231](https://github.com/casey/just/pull/2231) by [casey](https://github.com/casey))

[1.30.0](https://github.com/casey/just/releases/tag/1.30.0) - 2024-07-06
------------------------------------------------------------------------

### Fixed
- Allow comments after `mod` statements ([#2201](https://github.com/casey/just/pull/2201) by [casey](https://github.com/casey))

### Changed
- Allow unstable features with `--summary` ([#2210](https://github.com/casey/just/pull/2210) by [casey](https://github.com/casey))
- Don't analyze comments when `ignore-comments` is set ([#2180](https://github.com/casey/just/pull/2180) by [casey](https://github.com/casey))
- List recipes by group in group justfile order with `just --list --unsorted` ([#2164](https://github.com/casey/just/pull/2164) by [casey](https://github.com/casey))
- List groups in source order with `just --groups --unsorted` ([#2160](https://github.com/casey/just/pull/2160) by [casey](https://github.com/casey))

### Added
- Avoid `install` and add 32-bit arm targets to `install.sh` ([#2214](https://github.com/casey/just/pull/2214) by [CramBL](https://github.com/CramBL))
- Give modules doc comments for `--list` ([#2199](https://github.com/casey/just/pull/2199) by [Spatenheinz](https://github.com/Spatenheinz))
- Add `datetime()` and `datetime_utc()` functions ([#2167](https://github.com/casey/just/pull/2167) by [casey](https://github.com/casey))
- Allow setting more command-line options with environment variables ([#2161](https://github.com/casey/just/pull/2161) by [casey](https://github.com/casey))

### Library
- Don't exit process in `run()` on argument parse error ([#2176](https://github.com/casey/just/pull/2176) by [casey](https://github.com/casey))
- Allow passing command-line arguments into `run()` ([#2173](https://github.com/casey/just/pull/2173) by [casey](https://github.com/casey))
- Ignore env_logger initialization errors ([#2170](https://github.com/casey/just/pull/2170) by [EnigmaCurry](https://github.com/EnigmaCurry))

### Misc
- Tweak readme ([#2227](https://github.com/casey/just/pull/2227) by [casey](https://github.com/casey))
- Add development guide to readme ([#2226](https://github.com/casey/just/pull/2226) by [casey](https://github.com/casey))
- Add shell-expanded string syntax to grammar ([#2223](https://github.com/casey/just/pull/2223) by [casey](https://github.com/casey))
- Add recipe for testing bash completion script ([#2221](https://github.com/casey/just/pull/2221) by [casey](https://github.com/casey))
- Fix use of `justfile_directory()` in readme ([#2219](https://github.com/casey/just/pull/2219) by [casey](https://github.com/casey))
- Use default values for `--list-heading` and `--list-prefix` ([#2213](https://github.com/casey/just/pull/2213) by [casey](https://github.com/casey))
- Use `clap::ValueParser` ([#2211](https://github.com/casey/just/pull/2211) by [neunenak](https://github.com/neunenak))
- Document module doc comments in readme ([#2208](https://github.com/casey/just/pull/2208) by [casey](https://github.com/casey))
- Use `-and` instead of `&&` in PowerShell completion script ([#2204](https://github.com/casey/just/pull/2204) by [casey](https://github.com/casey))
- Fix readme formatting ([#2203](https://github.com/casey/just/pull/2203) by [casey](https://github.com/casey))
- Link to justfiles on GitHub in readme ([#2198](https://github.com/casey/just/pull/2198) by [bukowa](https://github.com/bukowa))
- Link to modules when first introduced in readme ([#2193](https://github.com/casey/just/pull/2193) by [casey](https://github.com/casey))
- Update `softprops/action-gh-release` ([#2183](https://github.com/casey/just/pull/2183) by [app/dependabot](https://github.com/app/dependabot))
- Document remote justfile workaround ([#2175](https://github.com/casey/just/pull/2175) by [casey](https://github.com/casey))
- Document library interface ([#2174](https://github.com/casey/just/pull/2174) by [casey](https://github.com/casey))
- Remove dependency on cradle ([#2169](https://github.com/casey/just/pull/2169) by [nc7s](https://github.com/nc7s))
- Add note to readme about quoting paths on Windows ([#2166](https://github.com/casey/just/pull/2166) by [casey](https://github.com/casey))
- Add missing changelog credits ([#2163](https://github.com/casey/just/pull/2163) by [casey](https://github.com/casey))
- Credit myself in changelog ([#2162](https://github.com/casey/just/pull/2162) by [casey](https://github.com/casey))

[1.29.1](https://github.com/casey/just/releases/tag/1.29.1) - 2024-06-14
------------------------------------------------------------------------

### Fixed
- Fix unexport syntax conflicts ([#2158](https://github.com/casey/just/pull/2158) by [casey](https://github.com/casey))

[1.29.0](https://github.com/casey/just/releases/tag/1.29.0) - 2024-06-13
------------------------------------------------------------------------

### Added
- Add [positional-arguments] attribute ([#2151](https://github.com/casey/just/pull/2151) by [casey](https://github.com/casey))
- Use `--justfile` in Fish shell completions ([#2148](https://github.com/casey/just/pull/2148) by [rubot](https://github.com/rubot))
- Add `is_dependency()` function ([#2139](https://github.com/casey/just/pull/2139) by [neunenak](https://github.com/neunenak))
- Allow printing nu completion script with `just --completions nushell` ([#2140](https://github.com/casey/just/pull/2140) by [casey](https://github.com/casey))
- Add `[ATTRIBUTE: VALUE]` shorthand ([#2136](https://github.com/casey/just/pull/2136) by [neunenak](https://github.com/neunenak))
- Allow unexporting environment variables ([#2098](https://github.com/casey/just/pull/2098) by [neunenak](https://github.com/neunenak))

### Fixed
- Load environment file from dotenv-path relative to working directory ([#2152](https://github.com/casey/just/pull/2152) by [casey](https://github.com/casey))
- Fix `fzf` chooser preview with space-separated module paths ([#2141](https://github.com/casey/just/pull/2141) by [casey](https://github.com/casey))

### Misc
- Improve argument parsing and error handling for submodules ([#2154](https://github.com/casey/just/pull/2154) by [casey](https://github.com/casey))
- Document shell expanded string defaults ([#2153](https://github.com/casey/just/pull/2153) by [casey](https://github.com/casey))
- Test bare bash path in shebang on windows ([#2144](https://github.com/casey/just/pull/2144) by [casey](https://github.com/casey))
- Test shell not found error messages ([#2145](https://github.com/casey/just/pull/2145) by [casey](https://github.com/casey))
- Refactor evaluator ([#2138](https://github.com/casey/just/pull/2138) by [neunenak](https://github.com/neunenak))
- Fix man page generation in release workflow ([#2132](https://github.com/casey/just/pull/2132) by [casey](https://github.com/casey))

[1.28.0](https://github.com/casey/just/releases/tag/1.28.0) - 2024-06-05
------------------------------------------------------------------------

### Changed
- Write shebang recipes to $XDG_RUNTIME_DIR ([#2128](https://github.com/casey/just/pull/2128) by [casey](https://github.com/casey))
- Add `set dotenv-required` to require an environment file ([#2116](https://github.com/casey/just/pull/2116) by [casey](https://github.com/casey))
- Don't display submodule recipes in `--list` ([#2112](https://github.com/casey/just/pull/2112) by [casey](https://github.com/casey))

### Added
- Allow listing recipes in submodules with `--list-submodules` ([#2113](https://github.com/casey/just/pull/2113) by [casey](https://github.com/casey))
- Show recipes in submodules with `--show RECIPE::PATH` ([#2111](https://github.com/casey/just/pull/2111) by [casey](https://github.com/casey))
- Add `--timestamp-format` ([#2106](https://github.com/casey/just/pull/2106) by [neunenak](https://github.com/neunenak))
- Allow listing submodule recipes with `--list PATH` ([#2108](https://github.com/casey/just/pull/2108) by [casey](https://github.com/casey))
- Print recipe command timestamps with `--timestamps` ([#2084](https://github.com/casey/just/pull/2084) by [neunenak](https://github.com/neunenak))
- Add `module_file()` and `module_directory()` functions ([#2105](https://github.com/casey/just/pull/2105) by [casey](https://github.com/casey))

### Fixed
- Use space-separated recipe paths in `--choose` ([#2115](https://github.com/casey/just/pull/2115) by [casey](https://github.com/casey))
- Fix bash completion for aliases ([#2104](https://github.com/casey/just/pull/2104) by [laniakea64](https://github.com/laniakea64))

### Misc
- Don't check in manpage ([#2130](https://github.com/casey/just/pull/2130) by [casey](https://github.com/casey))
- Document default shell ([#2129](https://github.com/casey/just/pull/2129) by [casey](https://github.com/casey))
- Remove duplicate section in Chinese readme ([#2127](https://github.com/casey/just/pull/2127) by [potterxu](https://github.com/potterxu))
- Update Chinese readme ([#2124](https://github.com/casey/just/pull/2124) by [potterxu](https://github.com/potterxu))
- Fix typo in readme ([#2122](https://github.com/casey/just/pull/2122) by [potterxu](https://github.com/potterxu))
- Don't check in auto-generated completion scripts ([#2120](https://github.com/casey/just/pull/2120) by [casey](https://github.com/casey))
- Document when dependencies run in readme ([#2103](https://github.com/casey/just/pull/2103) by [casey](https://github.com/casey))
- Build aarch64-pc-windows-msvc release binaries ([#2100](https://github.com/casey/just/pull/2100) by [alshdavid](https://github.com/alshdavid))
- Clarify that `dotenv-path`-given env file is required ([#2099](https://github.com/casey/just/pull/2099) by [casey](https://github.com/casey))
- Print multi-line doc comments before recipe in `--list` ([#2090](https://github.com/casey/just/pull/2090) by [casey](https://github.com/casey))
- List unsorted imported recipes by import depth and offset ([#2092](https://github.com/casey/just/pull/2092) by [casey](https://github.com/casey))
- Update README.md ([#2091](https://github.com/casey/just/pull/2091) by [laniakea64](https://github.com/laniakea64))

[1.27.0](https://github.com/casey/just/releases/tag/1.27.0) - 2024-05-25
------------------------------------------------------------------------

### Changed
- Use cache dir for temporary files ([#2067](https://github.com/casey/just/pull/2067) by [casey](https://github.com/casey))

### Added
- Add `[doc]` attribute to set and suppress documentation comments ([#2050](https://github.com/casey/just/pull/2050) by [neunenak](https://github.com/neunenak))
- Add source_file() and source_directory() functions ([#2088](https://github.com/casey/just/pull/2088) by [casey](https://github.com/casey))
- Add recipe groups ([#1842](https://github.com/casey/just/pull/1842) by [neunenak](https://github.com/neunenak))
- Add shell() function for running external commands ([#2047](https://github.com/casey/just/pull/2047) by [gyreas](https://github.com/gyreas))
- Add `--global-justfile` flag ([#1846](https://github.com/casey/just/pull/1846) by [neunenak](https://github.com/neunenak))
- Add shell-expanded strings ([#2055](https://github.com/casey/just/pull/2055) by [casey](https://github.com/casey))
- Add `encode_uri_component` function ([#2052](https://github.com/casey/just/pull/2052) by [laniakea64](https://github.com/laniakea64))
- Add `choose` function for generating random strings ([#2049](https://github.com/casey/just/pull/2049) by [laniakea64](https://github.com/laniakea64))
- Add predefined constants ([#2054](https://github.com/casey/just/pull/2054) by [casey](https://github.com/casey))
- Allow setting some command-line options with environment variables ([#2044](https://github.com/casey/just/pull/2044) by [neunenak](https://github.com/neunenak))
- Add prepend() function ([#2045](https://github.com/casey/just/pull/2045) by [gyreas](https://github.com/gyreas))
- Add append() function ([#2046](https://github.com/casey/just/pull/2046) by [gyreas](https://github.com/gyreas))
- Add --man subcommand ([#2041](https://github.com/casey/just/pull/2041) by [casey](https://github.com/casey))
- Make `dotenv-path` relative to working directory ([#2040](https://github.com/casey/just/pull/2040) by [casey](https://github.com/casey))
- Add `assert` expression ([#1845](https://github.com/casey/just/pull/1845) by [de1iza](https://github.com/de1iza))
- Add 'allow-duplicate-variables' setting ([#1922](https://github.com/casey/just/pull/1922) by [Mijago](https://github.com/Mijago))

### Fixed
- List modules in source order with `--unsorted` ([#2085](https://github.com/casey/just/pull/2085) by [casey](https://github.com/casey))
- Show submodule recipes in --choose ([#2069](https://github.com/casey/just/pull/2069) by [casey](https://github.com/casey))
- Allow multiple imports of the same file in different modules ([#2065](https://github.com/casey/just/pull/2065) by [casey](https://github.com/casey))
- Fix submodule recipe listing indentation ([#2063](https://github.com/casey/just/pull/2063) by [casey](https://github.com/casey))
- Pass command as first argument to `shell` ([#2061](https://github.com/casey/just/pull/2061) by [casey](https://github.com/casey))
- Allow shell expanded strings in mod and import paths ([#2059](https://github.com/casey/just/pull/2059) by [casey](https://github.com/casey))
- Run imported recipes in root justfile with correct working directory ([#2056](https://github.com/casey/just/pull/2056) by [casey](https://github.com/casey))
- Fix output `\r\n` stripping ([#2035](https://github.com/casey/just/pull/2035) by [casey](https://github.com/casey))

### Misc
- Forbid whitespace in shell-expanded string prefixes ([#2083](https://github.com/casey/just/pull/2083) by [casey](https://github.com/casey))
- Add Debian and Ubuntu install instructions to readme ([#2072](https://github.com/casey/just/pull/2072) by [casey](https://github.com/casey))
- Remove snap installation instructions from readme ([#2070](https://github.com/casey/just/pull/2070) by [casey](https://github.com/casey))
- Fallback to wget in install script if curl isn't available([#1913](https://github.com/casey/just/pull/1913) by [tgross35](https://github.com/tgross35))
- Use std::io::IsTerminal instead of atty crate ([#2066](https://github.com/casey/just/pull/2066) by [casey](https://github.com/casey))
- Improve `shell()` documentation ([#2060](https://github.com/casey/just/pull/2060) by [laniakea64](https://github.com/laniakea64))
- Add bash completion for snap ([#2058](https://github.com/casey/just/pull/2058) by [albertodonato](https://github.com/albertodonato))
- Refactor list subcommand ([#2062](https://github.com/casey/just/pull/2062) by [casey](https://github.com/casey))
- Document working directory ([#2053](https://github.com/casey/just/pull/2053) by [casey](https://github.com/casey))
- Replace FunctionContext with Evaluator ([#2048](https://github.com/casey/just/pull/2048) by [casey](https://github.com/casey))
- Update clap to version 4 ([#1924](https://github.com/casey/just/pull/1924) by [poliorcetics](https://github.com/poliorcetics))
- Cleanup ([#2026](https://github.com/casey/just/pull/2026) by [adamnemecek](https://github.com/adamnemecek))
- Increase --list maximum alignable width from 30 to 50 ([#2039](https://github.com/casey/just/pull/2039) by [casey](https://github.com/casey))
- Document using `env -S` ([#2038](https://github.com/casey/just/pull/2038) by [casey](https://github.com/casey))
- Update line continuation documentation ([#1998](https://github.com/casey/just/pull/1998) by [laniakea64](https://github.com/laniakea64))
- Add example using GNU parallel to run tasks in concurrently ([#1915](https://github.com/casey/just/pull/1915) by [amarao](https://github.com/amarao))
- Placate clippy: use `clone_into` ([#2037](https://github.com/casey/just/pull/2037) by [casey](https://github.com/casey))
- Use --command-color when printing shebang recipe commands ([#1911](https://github.com/casey/just/pull/1911) by [avi-cenna](https://github.com/avi-cenna))
- Document how to use watchexec to re-run recipes when files change ([#2036](https://github.com/casey/just/pull/2036) by [casey](https://github.com/casey))
- Update VS Code extensions in readme ([#2034](https://github.com/casey/just/pull/2034) by [casey](https://github.com/casey))
- Add rust:just repology package table to readme ([#2032](https://github.com/casey/just/pull/2032) by [casey](https://github.com/casey))

[1.26.0](https://github.com/casey/just/releases/tag/1.26.0) - 2024-05-13
------------------------------------------------------------------------

### Added
- Add --no-aliases to hide aliases in --list ([#1961](https://github.com/casey/just/pull/1961) by [WJehee](https://github.com/WJehee))
- Add -E as alias for --dotenv-path ([#1910](https://github.com/casey/just/pull/1910) by [amarao](https://github.com/amarao))

### Misc
- Update softprops/action-gh-release ([#2029](https://github.com/casey/just/pull/2029) by [app/dependabot](https://github.com/app/dependabot))
- Update dependencies ([#1999](https://github.com/casey/just/pull/1999) by [neunenak](https://github.com/neunenak))
- Bump peaceiris/actions-gh-pages to version 4 ([#2005](https://github.com/casey/just/pull/2005) by [app/dependabot](https://github.com/app/dependabot))
- Clarify that janus operates on public justfiles only ([#2021](https://github.com/casey/just/pull/2021) by [casey](https://github.com/casey))
- Fix Error::TmpdirIo error message ([#1987](https://github.com/casey/just/pull/1987) by [casey](https://github.com/casey))
- Update softprops/action-gh-release ([#1973](https://github.com/casey/just/pull/1973) by [app/dependabot](https://github.com/app/dependabot))
- Rename `delete` example recipe to `delete-all` ([#1966](https://github.com/casey/just/pull/1966) by [aarmn](https://github.com/aarmn))
- Update softprops/action-gh-release ([#1954](https://github.com/casey/just/pull/1954) by [app/dependabot](https://github.com/app/dependabot))
- Fix function name typo ([#1953](https://github.com/casey/just/pull/1953) by [racerole](https://github.com/racerole))

[1.25.2](https://github.com/casey/just/releases/tag/1.25.2) - 2024-03-10
------------------------------------------------------------------------

- Unpin ctrlc ([#1951](https://github.com/casey/just/pull/1951) by [casey](https://github.com/casey))

[1.25.1](https://github.com/casey/just/releases/tag/1.25.1) - 2024-03-09
------------------------------------------------------------------------

### Misc
- Pin ctrlc to version 3.1.1 ([#1945](https://github.com/casey/just/pull/1945) by [casey](https://github.com/casey))
- Fix AArch64 release build error ([#1942](https://github.com/casey/just/pull/1942) by [casey](https://github.com/casey))

[1.25.0](https://github.com/casey/just/releases/tag/1.25.0) - 2024-03-07
------------------------------------------------------------------------

### Added
- Add `blake3` and `blake3_file` functions ([#1860](https://github.com/casey/just/pull/1860) by [tgross35](https://github.com/tgross35))

### Misc
- Fix readme typo ([#1936](https://github.com/casey/just/pull/1936) by [Justintime50](https://github.com/Justintime50))
- Use unwrap_or_default ([#1928](https://github.com/casey/just/pull/1928) by [casey](https://github.com/casey))
- Set codegen-units to 1 reduce release binary size ([#1920](https://github.com/casey/just/pull/1920) by [amarao](https://github.com/amarao))
- Document openSUSE package ([#1918](https://github.com/casey/just/pull/1918) by [sfalken](https://github.com/sfalken))
- Fix install.sh shellcheck warnings ([#1912](https://github.com/casey/just/pull/1912) by [tgross35](https://github.com/tgross35))

[1.24.0](https://github.com/casey/just/releases/tag/1.24.0) - 2024-02-11
------------------------------------------------------------------------

### Added
- Support recipe paths containing `::` in Bash completion script ([#1863](https://github.com/casey/just/pull/1863) by [crdx](https://github.com/crdx))
- Add function to canonicalize paths ([#1859](https://github.com/casey/just/pull/1859) by [casey](https://github.com/casey))

### Misc
- Document installing just on Github Actions in readme ([#1867](https://github.com/casey/just/pull/1867) by [cclauss](https://github.com/cclauss))
- Use unlikely-to-be-set variable name in env tests ([#1882](https://github.com/casey/just/pull/1882) by [casey](https://github.com/casey))
- Skip write_error test if running as root ([#1881](https://github.com/casey/just/pull/1881) by [casey](https://github.com/casey))
- Convert run_shebang into integration test ([#1880](https://github.com/casey/just/pull/1880) by [casey](https://github.com/casey))
- Install mdbook with cargo in CI workflow ([#1877](https://github.com/casey/just/pull/1877) by [casey](https://github.com/casey))
- Remove deprecated actions-rs/toolchain ([#1874](https://github.com/casey/just/pull/1874) by [cclauss](https://github.com/cclauss))
- Fix Gentoo package link ([#1875](https://github.com/casey/just/pull/1875) by [vozbu](https://github.com/vozbu))
- Fix typos found by codespell ([#1872](https://github.com/casey/just/pull/1872) by [cclauss](https://github.com/cclauss))
- Replace deprecated set-output command in Github Actions workflows ([#1869](https://github.com/casey/just/pull/1869) by [cclauss](https://github.com/cclauss))
- Update `actions/checkout` and `softprops/action-gh-release` ([#1871](https://github.com/casey/just/pull/1871) by [app/dependabot](https://github.com/app/dependabot))
- Keep GitHub Actions up to date with Dependabot ([#1868](https://github.com/casey/just/pull/1868) by [cclauss](https://github.com/cclauss))
- Add contrib directory ([#1870](https://github.com/casey/just/pull/1870) by [casey](https://github.com/casey))
- Fix install script ([#1844](https://github.com/casey/just/pull/1844) by [casey](https://github.com/casey))

[1.23.0](https://github.com/casey/just/releases/tag/1.23.0) - 2024-01-12
------------------------------------------------------------------------

### Added
- Allow setting custom confirm prompt ([#1834](https://github.com/casey/just/pull/1834) by [CramBL](https://github.com/CramBL))
- Add `set quiet` and `[no-quiet]` ([#1704](https://github.com/casey/just/pull/1704) by [dharrigan](https://github.com/dharrigan))
- Add `just_pid` function ([#1833](https://github.com/casey/just/pull/1833) by [Swordelf2](https://github.com/Swordelf2))
- Add functions to return XDG base directories ([#1822](https://github.com/casey/just/pull/1822) by [tgross35](https://github.com/tgross35))
- Add `--no-deps` to skip running recipe dependencies ([#1819](https://github.com/casey/just/pull/1819) by [ngharrington](https://github.com/ngharrington))

### Fixed
- Run imports in working directory of importer ([#1817](https://github.com/casey/just/pull/1817) by [casey](https://github.com/casey))

### Misc
- Include completion scripts in releases ([#1837](https://github.com/casey/just/pull/1837) by [casey](https://github.com/casey))
- Tweak readme table formatting ([#1836](https://github.com/casey/just/pull/1836) by [casey](https://github.com/casey))
- Don't abbreviate just in README ([#1831](https://github.com/casey/just/pull/1831) by [thled](https://github.com/thled))
- Ignore [private] recipes in just --list ([#1816](https://github.com/casey/just/pull/1816) by [crdx](https://github.com/crdx))
- Add a dash to tempdir prefix ([#1828](https://github.com/casey/just/pull/1828) by [casey](https://github.com/casey))

[1.22.1](https://github.com/casey/just/releases/tag/1.22.1) - 2024-01-08
------------------------------------------------------------------------

### Fixed
- Don't conflate recipes with the same name in different modules ([#1825](https://github.com/casey/just/pull/1825) by [casey](https://github.com/casey))

### Misc
- Clarify that UUID is version 4 ([#1821](https://github.com/casey/just/pull/1821) by [tgross35](https://github.com/tgross35))
- Make sigil stripping from recipe lines less incomprehensible ([#1812](https://github.com/casey/just/pull/1812) by [casey](https://github.com/casey))
- Refactor invalid path argument check ([#1811](https://github.com/casey/just/pull/1811) by [casey](https://github.com/casey))

[1.22.0](https://github.com/casey/just/releases/tag/1.22.0) - 2023-12-31
------------------------------------------------------------------------

### Added
- Recipes can be invoked with path syntax ([#1809](https://github.com/casey/just/pull/1809) by [casey](https://github.com/casey))
- Add `--format` and `--initialize` as aliases for `--fmt` and `--init` ([#1802](https://github.com/casey/just/pull/1802) by [casey](https://github.com/casey))

### Misc
- Move table of contents pointer to right ([#1806](https://github.com/casey/just/pull/1806) by [casey](https://github.com/casey))

[1.21.0](https://github.com/casey/just/releases/tag/1.21.0) - 2023-12-29
------------------------------------------------------------------------

### Added
- Optional modules and imports ([#1797](https://github.com/casey/just/pull/1797) by [casey](https://github.com/casey))
- Print submodule recipes in --summary ([#1794](https://github.com/casey/just/pull/1794) by [casey](https://github.com/casey))

### Misc
- Use box-drawing characters in error messages ([#1798](https://github.com/casey/just/pull/1798) by [casey](https://github.com/casey))
- Use Self ([#1795](https://github.com/casey/just/pull/1795) by [casey](https://github.com/casey))

[1.20.0](https://github.com/casey/just/releases/tag/1.20.0) - 2023-12-28
------------------------------------------------------------------------

### Added
- Allow mod statements with path to source file ([#1786](https://github.com/casey/just/pull/1786) by [casey](https://github.com/casey))

### Changed
- Expand tilde in import and module paths ([#1792](https://github.com/casey/just/pull/1792) by [casey](https://github.com/casey))
- Override imported recipes ([#1790](https://github.com/casey/just/pull/1790) by [casey](https://github.com/casey))
- Run recipes with working directory set to submodule directory ([#1788](https://github.com/casey/just/pull/1788) by [casey](https://github.com/casey))

### Misc
- Document import override behavior ([#1791](https://github.com/casey/just/pull/1791) by [casey](https://github.com/casey))
- Document submodule working directory ([#1789](https://github.com/casey/just/pull/1789) by [casey](https://github.com/casey))

[1.19.0](https://github.com/casey/just/releases/tag/1.19.0) - 2023-12-27
------------------------------------------------------------------------

### Added
- Add modules ([#1782](https://github.com/casey/just/pull/1782) by [casey](https://github.com/casey))

[1.18.1](https://github.com/casey/just/releases/tag/1.18.1) - 2023-12-24
------------------------------------------------------------------------

### Added
- Display a descriptive error for `!include` directives ([#1779](https://github.com/casey/just/pull/1779) by [casey](https://github.com/casey))

[1.18.0](https://github.com/casey/just/releases/tag/1.18.0) - 2023-12-24
------------------------------------------------------------------------

### Added
- Stabilize `!include path` as `import 'path'` ([#1771](https://github.com/casey/just/pull/1771) by [casey](https://github.com/casey))

### Misc
- Tweak readme ([#1775](https://github.com/casey/just/pull/1775) by [casey](https://github.com/casey))

[1.17.0](https://github.com/casey/just/releases/tag/1.17.0) - 2023-12-20
------------------------------------------------------------------------

### Added
- Add `[confirm]` attribute ([#1723](https://github.com/casey/just/pull/1723) by [Hwatwasthat](https://github.com/Hwatwasthat))

### Changed
- Don't default to included recipes ([#1740](https://github.com/casey/just/pull/1740) by [casey](https://github.com/casey))

### Fixed
- Pass justfile path to default chooser ([#1759](https://github.com/casey/just/pull/1759) by [Qeole](https://github.com/Qeole))
- Pass `--unstable` and `--color always` to default chooser ([#1758](https://github.com/casey/just/pull/1758) by [Qeole](https://github.com/Qeole))

### Misc
- Update Gentoo package repository ([#1757](https://github.com/casey/just/pull/1757) by [paul-jewell](https://github.com/paul-jewell))
- Fix readme header level ([#1752](https://github.com/casey/just/pull/1752) by [laniakea64](https://github.com/laniakea64))
- Document line continuations ([#1751](https://github.com/casey/just/pull/1751) by [laniakea64](https://github.com/laniakea64))
- List included recipes in load order ([#1745](https://github.com/casey/just/pull/1745) by [casey](https://github.com/casey))
- Fix build badge in zh readme ([#1743](https://github.com/casey/just/pull/1743) by [chenrui333](https://github.com/chenrui333))
- Rename Justfile::first → Justfile::default ([#1741](https://github.com/casey/just/pull/1741) by [casey](https://github.com/casey))
- Add file paths to error messages ([#1737](https://github.com/casey/just/pull/1737) by [casey](https://github.com/casey))
- Move !include processing into compiler ([#1618](https://github.com/casey/just/pull/1618) by [neunenak](https://github.com/neunenak))
- Update Arch Linux package URL in readme ([#1733](https://github.com/casey/just/pull/1733) by [felixonmars](https://github.com/felixonmars))
- Clarify that aliases can only be used on the command line ([#1726](https://github.com/casey/just/pull/1726) by [laniakea64](https://github.com/laniakea64))
- Remove VALID_ALIAS_ATTRIBUTES array ([#1731](https://github.com/casey/just/pull/1731) by [casey](https://github.com/casey))
- Fix justfile search link in Chinese docs ([#1730](https://github.com/casey/just/pull/1730) by [oluceps](https://github.com/oluceps))
- Add example of Windows shebang handling ([#1709](https://github.com/casey/just/pull/1709) by [pfmoore](https://github.com/pfmoore))
- Fix CI ([#1728](https://github.com/casey/just/pull/1728) by [casey](https://github.com/casey))

[1.16.0](https://github.com/casey/just/releases/tag/1.16.0) - 2023-11-08
------------------------------------------------------------------------

### Added
- Add ARMv6 release target ([#1715](https://github.com/casey/just/pull/1715) by [ragazenta](https://github.com/ragazenta))
- Add `semver_matches` function ([#1713](https://github.com/casey/just/pull/1713) by [t3hmrman](https://github.com/t3hmrman))
- Add `dotenv-filename` and `dotenv-path` settings ([#1692](https://github.com/casey/just/pull/1692) by [ltfourrier](https://github.com/ltfourrier))
- Allow setting echoed recipe line color ([#1670](https://github.com/casey/just/pull/1670) by [avi-cenna](https://github.com/avi-cenna))

### Fixed
- Fix Fish completion script ([#1710](https://github.com/casey/just/pull/1710) by [l4zygreed](https://github.com/l4zygreed))

### Misc
- Fix readme typo ([#1717](https://github.com/casey/just/pull/1717) by [barraponto](https://github.com/barraponto))
- Clean up error display ([#1699](https://github.com/casey/just/pull/1699) by [nyurik](https://github.com/nyurik))
- Misc fixes ([#1700](https://github.com/casey/just/pull/1700) by [nyurik](https://github.com/nyurik))
- Fix readme build badge ([#1697](https://github.com/casey/just/pull/1697) by [casey](https://github.com/casey))
- Fix set tempdir grammar ([#1695](https://github.com/casey/just/pull/1695) by [casey](https://github.com/casey))
- Add version to attributes ([#1694](https://github.com/casey/just/pull/1694) by [JoeyTeng](https://github.com/JoeyTeng))
- Update README.md ([#1691](https://github.com/casey/just/pull/1691) by [laniakea64](https://github.com/laniakea64))


[1.15.0](https://github.com/casey/just/releases/tag/1.15.0) - 2023-10-09
------------------------------------------------------------------------

### Added
- Add Nushell completion script ([#1571](https://github.com/casey/just/pull/1571) by [presidento](https://github.com/presidento))
- Allow unstable features to be enabled with environment variable ([#1588](https://github.com/casey/just/pull/1588) by [neunenak](https://github.com/neunenak))
- Add num_cpus() function ([#1568](https://github.com/casey/just/pull/1568) by [schultetwin1](https://github.com/schultetwin1))
- Allow escaping newlines ([#1551](https://github.com/casey/just/pull/1551) by [ids1024](https://github.com/ids1024))
- Stabilize JSON dump format ([#1633](https://github.com/casey/just/pull/1633) by [casey](https://github.com/casey))
- Add env() function ([#1613](https://github.com/casey/just/pull/1613) by [kykyi](https://github.com/kykyi))

### Changed
- Allow selecting multiple recipes with default chooser ([#1547](https://github.com/casey/just/pull/1547) by [fzdwx](https://github.com/fzdwx))

### Misc
- Don't recommend `vim-polyglot` in readme ([#1644](https://github.com/casey/just/pull/1644) by [laniakea64](https://github.com/laniakea64))
- Note Micro support in readme ([#1316](https://github.com/casey/just/pull/1316) by [tomodachi94](https://github.com/tomodachi94))
- Update Indentation Documentation ([#1600](https://github.com/casey/just/pull/1600) by [GinoMan](https://github.com/GinoMan))
- Fix triple-quoted string example in readme ([#1620](https://github.com/casey/just/pull/1620) by [avi-cenna](https://github.com/avi-cenna))
- README fix: the -d in `mktemp -d` is required to created folders. ([#1688](https://github.com/casey/just/pull/1688) by [gl-yziquel](https://github.com/gl-yziquel))
- Placate clippy ([#1689](https://github.com/casey/just/pull/1689) by [casey](https://github.com/casey))
- Fix README typos ([#1660](https://github.com/casey/just/pull/1660) by [akuhnregnier](https://github.com/akuhnregnier))
- Document Windows Package Manager install instructions ([#1656](https://github.com/casey/just/pull/1656) by [casey](https://github.com/casey))
- Test unpaired escaped carriage return error ([#1650](https://github.com/casey/just/pull/1650) by [casey](https://github.com/casey))
- Avoid grep aliases in bash completions ([#1622](https://github.com/casey/just/pull/1622) by [BojanStipic](https://github.com/BojanStipic))
- Clarify [unix] attribute in readme ([#1619](https://github.com/casey/just/pull/1619) by [neunenak](https://github.com/neunenak))
- Add descriptions to fish recipe completions ([#1578](https://github.com/casey/just/pull/1578) by [patricksjackson](https://github.com/patricksjackson))
- Add better documentation for --dump and --fmt ([#1603](https://github.com/casey/just/pull/1603) by [neunenak](https://github.com/neunenak))
- Cleanup ([#1566](https://github.com/casey/just/pull/1566) by [nyurik](https://github.com/nyurik))
- Document Helix editor support in readme ([#1604](https://github.com/casey/just/pull/1604) by [kenden](https://github.com/kenden))

[1.14.0](https://github.com/casey/just/releases/tag/1.14.0) - 2023-06-02
------------------------------------------------------------------------

### Changed
- Use `just --show` in default chooser ([#1539](https://github.com/casey/just/pull/1539) by [fzdwx](https://github.com/fzdwx))

### Misc
- Fix justfile search link ([#1607](https://github.com/casey/just/pull/1607) by [jbaber](https://github.com/jbaber))
- Ignore clippy::let_underscore_untyped ([#1609](https://github.com/casey/just/pull/1609) by [casey](https://github.com/casey))
- Link to private recipes section in readme ([#1542](https://github.com/casey/just/pull/1542) by [quad](https://github.com/quad))
- Update README to reflect new attribute syntax ([#1538](https://github.com/casey/just/pull/1538) by [neunenak](https://github.com/neunenak))
- Allow multiple attributes on one line ([#1537](https://github.com/casey/just/pull/1537) by [neunenak](https://github.com/neunenak))
- Analyze and Compiler tweaks ([#1534](https://github.com/casey/just/pull/1534) by [neunenak](https://github.com/neunenak))
- Downgrade to TLS 1.2 in install script ([#1536](https://github.com/casey/just/pull/1536) by [casey](https://github.com/casey))

[1.13.0](https://github.com/casey/just/releases/tag/1.13.0) - 2023-01-24
------------------------------------------------------------------------

### Added
- Add -n as a short flag for --for dry-run ([#1524](https://github.com/casey/just/pull/1524) by [maiha](https://github.com/maiha))
- Add invocation_directory_native() ([#1507](https://github.com/casey/just/pull/1507) by [casey](https://github.com/casey))

### Changed
- Ignore additional search path arguments ([#1528](https://github.com/casey/just/pull/1528) by [neunenak](https://github.com/neunenak))
- Only print fallback message when verbose ([#1510](https://github.com/casey/just/pull/1510) by [casey](https://github.com/casey))
- Print format diff to stdout ([#1506](https://github.com/casey/just/pull/1506) by [casey](https://github.com/casey))

### Fixed
- Test passing dot as argument between justfiles ([#1530](https://github.com/casey/just/pull/1530) by [casey](https://github.com/casey))
- Fix install script default directory ([#1525](https://github.com/casey/just/pull/1525) by [casey](https://github.com/casey))

### Misc
- Note that justfiles are order-insensitive ([#1529](https://github.com/casey/just/pull/1529) by [casey](https://github.com/casey))
- Borrow Ast in Analyser ([#1527](https://github.com/casey/just/pull/1527) by [neunenak](https://github.com/neunenak))
- Ignore chooser tests ([#1513](https://github.com/casey/just/pull/1513) by [casey](https://github.com/casey))
- Put default setting values in backticks ([#1512](https://github.com/casey/just/pull/1512) by [s1ck](https://github.com/s1ck))
- Use lowercase boolean literals in readme ([#1511](https://github.com/casey/just/pull/1511) by [s1ck](https://github.com/s1ck))
- Document invocation_directory_native() ([#1508](https://github.com/casey/just/pull/1508) by [casey](https://github.com/casey))
- Fix interrupt tests ([#1505](https://github.com/casey/just/pull/1505) by [casey](https://github.com/casey))

[1.12.0](https://github.com/casey/just/releases/tag/1.12.0) - 2023-01-12
------------------------------------------------------------------------

### Added
- Add `!include` directives ([#1470](https://github.com/casey/just/pull/1470) by [neunenak](https://github.com/neunenak))

### Changed
- Allow matching search path arguments ([#1475](https://github.com/casey/just/pull/1475) by [neunenak](https://github.com/neunenak))
- Allow recipe parameters to shadow variables ([#1480](https://github.com/casey/just/pull/1480) by [casey](https://github.com/casey))

### Misc
- Remove --unstable from fallback example in readme ([#1502](https://github.com/casey/just/pull/1502) by [casey](https://github.com/casey))
- Specify minimum rust version ([#1496](https://github.com/casey/just/pull/1496) by [benmoss](https://github.com/benmoss))
- Note that install.sh may fail on GitHub actions ([#1499](https://github.com/casey/just/pull/1499) by [casey](https://github.com/casey))
- Fix readme typo ([#1489](https://github.com/casey/just/pull/1489) by [auberisky](https://github.com/auberisky))
- Update install script and readmes to use tls v1.3 ([#1481](https://github.com/casey/just/pull/1481) by [casey](https://github.com/casey))
- Re-enable install.sh test on CI([#1478](https://github.com/casey/just/pull/1478) by [casey](https://github.com/casey))
- Don't test install.sh on CI ([#1477](https://github.com/casey/just/pull/1477) by [casey](https://github.com/casey))
- Update Chinese translation of readme ([#1476](https://github.com/casey/just/pull/1476) by [hustcer](https://github.com/hustcer))
- Fix install.sh for Windows ([#1474](https://github.com/casey/just/pull/1474) by [bloodearnest](https://github.com/bloodearnest))

[1.11.0](https://github.com/casey/just/releases/tag/1.11.0) - 2023-01-03
------------------------------------------------------------------------

### Added
- Stabilize fallback ([#1471](https://github.com/casey/just/pull/1471) by [casey](https://github.com/casey))

### Misc
- Update Sublime syntax instructions ([#1455](https://github.com/casey/just/pull/1455) by [nk9](https://github.com/nk9))

[1.10.0](https://github.com/casey/just/releases/tag/1.10.0) - 2023-01-01
------------------------------------------------------------------------

### Added
- Allow private attribute on aliases ([#1434](https://github.com/casey/just/pull/1434) by [neunenak](https://github.com/neunenak))

### Changed
- Suppress --fmt --check diff if --quiet is passed ([#1457](https://github.com/casey/just/pull/1457) by [casey](https://github.com/casey))

### Fixed
- Format exported variadic parameters correctly ([#1451](https://github.com/casey/just/pull/1451) by [casey](https://github.com/casey))

### Misc
- Fix section title grammar ([#1466](https://github.com/casey/just/pull/1466) by [brettcannon](https://github.com/brettcannon))
- Give pages job write permissions([#1464](https://github.com/casey/just/pull/1464) by [jsoref](https://github.com/jsoref))
- Fix spelling ([#1463](https://github.com/casey/just/pull/1463) by [jsoref](https://github.com/jsoref))
- Merge imports ([#1462](https://github.com/casey/just/pull/1462) by [casey](https://github.com/casey))
- Add instructions for taiki-e/install-action ([#1459](https://github.com/casey/just/pull/1459) by [azzamsa](https://github.com/azzamsa))
- Differentiate between shell and nushell example ([#1427](https://github.com/casey/just/pull/1427) by [Dialga](https://github.com/Dialga))
- Link regex docs in readme ([#1454](https://github.com/casey/just/pull/1454) by [casey](https://github.com/casey))
- Linkify changelog PRs and usernames ([#1440](https://github.com/casey/just/pull/1440) by [nk9](https://github.com/nk9))
- Eliminate lazy_static ([#1442](https://github.com/casey/just/pull/1442) by [camsteffen](https://github.com/camsteffen))
- Add attributes to sublime syntax file ([#1452](https://github.com/casey/just/pull/1452) by [crdx](https://github.com/crdx))
- Fix homepage style ([#1453](https://github.com/casey/just/pull/1453) by [casey](https://github.com/casey))
- Linkify homepage letters ([#1448](https://github.com/casey/just/pull/1448) by [nk9](https://github.com/nk9))
- Use `just` in readme codeblocks ([#1447](https://github.com/casey/just/pull/1447) by [nicochatzi](https://github.com/nicochatzi))
- Update MSRV in readme ([#1446](https://github.com/casey/just/pull/1446) by [casey](https://github.com/casey))
- Merge CI workflows ([#1444](https://github.com/casey/just/pull/1444) by [casey](https://github.com/casey))
- Use dotenvy instead of dotenv ([#1443](https://github.com/casey/just/pull/1443) by [mike-burns](https://github.com/mike-burns))
- Update Chinese translation of readme ([#1428](https://github.com/casey/just/pull/1428) by [hustcer](https://github.com/hustcer))

[1.9.0](https://github.com/casey/just/releases/tag/1.9.0) - 2022-11-25
----------------------------------------------------------------------

### Breaking Changes to Unstable Features
- Change `fallback` setting default to false ([#1425](https://github.com/casey/just/pull/1425) by [casey](https://github.com/casey))

### Added
- Hide recipes with `[private]` attribute ([#1422](https://github.com/casey/just/pull/1422) by [casey](https://github.com/casey))
- Add replace_regex function ([#1393](https://github.com/casey/just/pull/1393) by [miles170](https://github.com/miles170))
- Add [no-cd] attribute ([#1400](https://github.com/casey/just/pull/1400) by [casey](https://github.com/casey))

### Changed
- Omit shebang lines on Windows ([#1417](https://github.com/casey/just/pull/1417) by [casey](https://github.com/casey))

### Misc
- Placate clippy ([#1423](https://github.com/casey/just/pull/1423) by [casey](https://github.com/casey))
- Make include_shebang_line clearer ([#1418](https://github.com/casey/just/pull/1418) by [casey](https://github.com/casey))
- Use more secure cURL options in install.sh ([#1416](https://github.com/casey/just/pull/1416) by [casey](https://github.com/casey))
- Document how shebang recipes are executed ([#1412](https://github.com/casey/just/pull/1412) by [casey](https://github.com/casey))
- Fix typo: regec → regex ([#1409](https://github.com/casey/just/pull/1409) by [casey](https://github.com/casey))
- Use powershell.exe instead of pwsh.exe in readme ([#1394](https://github.com/casey/just/pull/1394) by [asdf8dfafjk](https://github.com/asdf8dfafjk))
- Expand alternatives and prior art in readme ([#1401](https://github.com/casey/just/pull/1401) by [casey](https://github.com/casey))
- Split up CI workflow ([#1399](https://github.com/casey/just/pull/1399) by [casey](https://github.com/casey))

[1.8.0](https://github.com/casey/just/releases/tag/1.8.0) - 2022-11-02
----------------------------------------------------------------------

### Added
- Add OS Configuration Attributes ([#1387](https://github.com/casey/just/pull/1387) by [casey](https://github.com/casey))

### Misc
- Link to sclu1034/vscode-just in readme ([#1396](https://github.com/casey/just/pull/1396) by [casey](https://github.com/casey))

[1.7.0](https://github.com/casey/just/releases/tag/1.7.0) - 2022-10-26
----------------------------------------------------------------------

### Breaking Changes to Unstable Features
- Make `fallback` setting default to true ([#1384](https://github.com/casey/just/pull/1384) by [casey](https://github.com/casey))

### Added
- Add more case-conversion functions ([#1383](https://github.com/casey/just/pull/1383) by [gVirtu](https://github.com/gVirtu))
- Add `tempdir` setting ([#1369](https://github.com/casey/just/pull/1369) by [dmatos2012](https://github.com/dmatos2012))
- Add [no-exit-message] recipe annotation ([#1354](https://github.com/casey/just/pull/1354) by [gokhanettin](https://github.com/gokhanettin))
- Add `capitalize(s)` function ([#1375](https://github.com/casey/just/pull/1375) by [femnad](https://github.com/femnad))

### Misc
- Credit contributors in changelog ([#1385](https://github.com/casey/just/pull/1385) by [casey](https://github.com/casey))
- Update asdf just plugin repository ([#1380](https://github.com/casey/just/pull/1380) by [kachick](https://github.com/kachick))
- Prepend commit messages with `- ` in changelog ([#1379](https://github.com/casey/just/pull/1379) by [casey](https://github.com/casey))
- Fail publish if `<sup>master</sup>` is found in README.md ([#1378](https://github.com/casey/just/pull/1378) by [casey](https://github.com/casey))
- Use for loop in capitalize implementation ([#1377](https://github.com/casey/just/pull/1377) by [casey](https://github.com/casey))

[1.6.0](https://github.com/casey/just/releases/tag/1.6.0) - 2022-10-19
----------------------------------------------------------------------

### Breaking Changes to Unstable Features
- Require `set fallback := true` to enable recipe fallback ([#1368](https://github.com/casey/just/pull/1368) by [casey](https://github.com/casey))

### Changed
- Allow fallback with search directory ([#1348](https://github.com/casey/just/pull/1348) by [casey](https://github.com/casey))

### Added
- Don't evaluate comments ([#1358](https://github.com/casey/just/pull/1358) by [casey](https://github.com/casey))
- Add skip-comments setting ([#1333](https://github.com/casey/just/pull/1333) by [neunenak](https://github.com/neunenak))
- Allow bash completion to complete tasks in other directories ([#1303](https://github.com/casey/just/pull/1303) by [jpbochi](https://github.com/jpbochi))

### Misc
- Restore www/CNAME ([#1364](https://github.com/casey/just/pull/1364) by [casey](https://github.com/casey))
- Improve book config ([#1363](https://github.com/casey/just/pull/1363) by [casey](https://github.com/casey))
- Add kitchen sink justfile to test syntax highlighting ([#1362](https://github.com/casey/just/pull/1362) by [nk9](https://github.com/nk9))
- Note version in which absolute path construction was added ([#1361](https://github.com/casey/just/pull/1361) by [casey](https://github.com/casey))
- Inline setup and cleanup functions in completion script test ([#1352](https://github.com/casey/just/pull/1352) by [casey](https://github.com/casey))

[1.5.0](https://github.com/casey/just/releases/tag/1.5.0) - 2022-9-11
---------------------------------------------------------------------

### Changed
- Allow constructing absolute paths with `/` operator ([#1320](https://github.com/casey/just/pull/1320) by [erikkrieg](https://github.com/erikkrieg))

### Misc
- Allow fewer lints ([#1340](https://github.com/casey/just/pull/1340) by [casey](https://github.com/casey))
- Fix issues reported by nightly clippy ([#1336](https://github.com/casey/just/pull/1336) by [neunenak](https://github.com/neunenak))
- Refactor run.rs ([#1335](https://github.com/casey/just/pull/1335) by [neunenak](https://github.com/neunenak))
- Allow comments on same line as settings ([#1339](https://github.com/casey/just/pull/1339) by [casey](https://github.com/casey))
- Fix justfile env shebang on Linux ([#1330](https://github.com/casey/just/pull/1330) by [casey](https://github.com/casey))
- Update Chinese translation of README.md ([#1325](https://github.com/casey/just/pull/1325) by [hustcer](https://github.com/hustcer))
- Add additional settings to grammar ([#1321](https://github.com/casey/just/pull/1321) by [psibi](https://github.com/psibi))
- Add an example of using a variable in a recipe parameter ([#1311](https://github.com/casey/just/pull/1311) by [papertigers](https://github.com/papertigers))

[1.4.0](https://github.com/casey/just/releases/tag/1.4.0) - 2022-8-08
---------------------------------------------------------------------

### Fixed
- Fix shell setting precedence ([#1306](https://github.com/casey/just/pull/1306) by [casey](https://github.com/casey))

### Misc
- Don't hardcode homebrew prefix ([#1295](https://github.com/casey/just/pull/1295) by [casey](https://github.com/casey))
- Exclude files from cargo package ([#1283](https://github.com/casey/just/pull/1283) by [casey](https://github.com/casey))
- Add usage note to default list recipe ([#1296](https://github.com/casey/just/pull/1296) by [jpbochi](https://github.com/jpbochi))
- Add MPR/Prebuilt-MPR installation instructions to README.md ([#1280](https://github.com/casey/just/pull/1280) by [hwittenborn](https://github.com/hwittenborn))
- Add make and makesure to readme ([#1299](https://github.com/casey/just/pull/1299) by [casey](https://github.com/casey))
- Document how to configure zsh completions on MacOS ([#1285](https://github.com/casey/just/pull/1285) by [nk9](https://github.com/nk9))
- Convert package table to HTML ([#1291](https://github.com/casey/just/pull/1291) by [casey](https://github.com/casey))

[1.3.0](https://github.com/casey/just/releases/tag/1.3.0) - 2022-7-25
---------------------------------------------------------------------

### Added
- Add `/` operator ([#1237](https://github.com/casey/just/pull/1237) by [casey](https://github.com/casey))

### Fixed
- Fix multibyte codepoint crash ([#1243](https://github.com/casey/just/pull/1243) by [casey](https://github.com/casey))

### Misc
- Update just-install reference on README.md ([#1275](https://github.com/casey/just/pull/1275) by [0xradical](https://github.com/0xradical))
- Split Recipe::run into Recipe::{run_shebang,run_linewise} ([#1270](https://github.com/casey/just/pull/1270) by [casey](https://github.com/casey))
- Add asdf package to readme([#1264](https://github.com/casey/just/pull/1264) by [jaacko-torus](https://github.com/jaacko-torus))
- Add mdbook deps for build-book recipe ([#1259](https://github.com/casey/just/pull/1259) by [TopherIsSwell](https://github.com/TopherIsSwell))
- Fix typo: argumant -> argument ([#1257](https://github.com/casey/just/pull/1257) by [kianmeng](https://github.com/kianmeng))
- Improve error message if `if` is missing the `else` ([#1252](https://github.com/casey/just/pull/1252) by [nk9](https://github.com/nk9))
- Explain how to pass arguments of a command to a dependency ([#1254](https://github.com/casey/just/pull/1254) by [heavelock](https://github.com/heavelock))
- Update Chinese translation of README.md ([#1253](https://github.com/casey/just/pull/1253) by [hustcer](https://github.com/hustcer))
- Improvements to Sublime syntax file ([#1250](https://github.com/casey/just/pull/1250) by [nk9](https://github.com/nk9))
- Prevent unbounded recursion when parsing expressions ([#1248](https://github.com/casey/just/pull/1248) by [evanrichter](https://github.com/evanrichter))
- Publish to snap store ([#1245](https://github.com/casey/just/pull/1245) by [casey](https://github.com/casey))
- Restore fuzz test harness ([#1246](https://github.com/casey/just/pull/1246) by [evanrichter](https://github.com/evanrichter))
- Add just-install to README file ([#1241](https://github.com/casey/just/pull/1241) by [brombal](https://github.com/brombal))
- Fix dead readme link ([#1240](https://github.com/casey/just/pull/1240) by [wdroz](https://github.com/wdroz))
- Do `use super::*;` instead of `use crate::common::*;` ([#1239](https://github.com/casey/just/pull/1239) by [casey](https://github.com/casey))
- Fix readme punctuation ([#1235](https://github.com/casey/just/pull/1235) by [casey](https://github.com/casey))
- Add argument splitting section to readme ([#1230](https://github.com/casey/just/pull/1230) by [casey](https://github.com/casey))
- Add notes about environment variables to readme ([#1229](https://github.com/casey/just/pull/1229) by [casey](https://github.com/casey))
- Fix book links ([#1227](https://github.com/casey/just/pull/1227) by [casey](https://github.com/casey))
- Add nushell README.md ([#1224](https://github.com/casey/just/pull/1224) by [hustcer](https://github.com/hustcer))
- Use absolute links in readme ([#1223](https://github.com/casey/just/pull/1223) by [casey](https://github.com/casey))
- Copy changelog into manual ([#1222](https://github.com/casey/just/pull/1222) by [casey](https://github.com/casey))
- Translate Chinese manual introduction and title ([#1220](https://github.com/casey/just/pull/1220) by [hustcer](https://github.com/hustcer))
- Build Chinese language user manual ([#1219](https://github.com/casey/just/pull/1219) by [casey](https://github.com/casey))
- Update Chinese translation of README.md ([#1218](https://github.com/casey/just/pull/1218) by [hustcer](https://github.com/hustcer))
- Translate all of README.md into Chinese ([#1217](https://github.com/casey/just/pull/1217) by [hustcer](https://github.com/hustcer))
- Translate all of features in README into Chinese ([#1215](https://github.com/casey/just/pull/1215) by [hustcer](https://github.com/hustcer))
- Make link to examples directory absolute ([#1213](https://github.com/casey/just/pull/1213) by [casey](https://github.com/casey))
- Translate part of features in README into Chinese ([#1211](https://github.com/casey/just/pull/1211) by [hustcer](https://github.com/hustcer))
- Add JetBrains IDE plugin to readme ([#1209](https://github.com/casey/just/pull/1209) by [linux-china](https://github.com/linux-china))
- Translate features chapter of readme to Chinese ([#1208](https://github.com/casey/just/pull/1208) by [hustcer](https://github.com/hustcer))

[1.2.0](https://github.com/casey/just/releases/tag/1.2.0) - 2022-5-31
---------------------------------------------------------------------

### Added
- Add `windows-shell` setting ([#1198](https://github.com/casey/just/pull/1198) by [casey](https://github.com/casey))
- SHA-256 and UUID functions ([#1170](https://github.com/casey/just/pull/1170) by [mbodmer](https://github.com/mbodmer))

### Misc
- Translate editor support and quick start to Chinese ([#1206](https://github.com/casey/just/pull/1206) by [hustcer](https://github.com/hustcer))
- Translate first section of readme into Chinese ([#1205](https://github.com/casey/just/pull/1205) by [hustcer](https://github.com/hustcer))
- Fix a bunch of typos ([#1204](https://github.com/casey/just/pull/1204) by [casey](https://github.com/casey))
- Remove cargo-limit usage from justfile ([#1199](https://github.com/casey/just/pull/1199) by [casey](https://github.com/casey))
- Add nix package manager install instructions ([#1194](https://github.com/casey/just/pull/1194) by [risingBirdSong](https://github.com/risingBirdSong))
- Fix broken link in readme ([#1183](https://github.com/casey/just/pull/1183) by [Vlad-Shcherbina](https://github.com/Vlad-Shcherbina))
- Add screenshot to manual ([#1181](https://github.com/casey/just/pull/1181) by [casey](https://github.com/casey))
- Style homepage ([#1180](https://github.com/casey/just/pull/1180) by [casey](https://github.com/casey))
- Center readme ([#1178](https://github.com/casey/just/pull/1178) by [casey](https://github.com/casey))
- Style and add links to homepage ([#1177](https://github.com/casey/just/pull/1177) by [casey](https://github.com/casey))
- Fix readme badge links ([#1176](https://github.com/casey/just/pull/1176) by [casey](https://github.com/casey))
- Generate book from readme ([#1155](https://github.com/casey/just/pull/1155) by [casey](https://github.com/casey))

[1.1.3](https://github.com/casey/just/releases/tag/1.1.3) - 2022-5-3
--------------------------------------------------------------------

### Fixed
- Skip duplicate recipe arguments ([#1174](https://github.com/casey/just/pull/1174) by [casey](https://github.com/casey))

### Misc
- Fix install script ([#1172](https://github.com/casey/just/pull/1172) by [casey](https://github.com/casey))
- Document that `invocation_directory()` returns an absolute path ([#1162](https://github.com/casey/just/pull/1162) by [casey](https://github.com/casey))
- Fix absolute_path documentation ([#1160](https://github.com/casey/just/pull/1160) by [casey](https://github.com/casey))
- Add cross-platform justfile example ([#1152](https://github.com/casey/just/pull/1152) by [presidento](https://github.com/presidento))

[1.1.2](https://github.com/casey/just/releases/tag/1.1.2) - 2022-3-30
---------------------------------------------------------------------

### Misc
- Document indentation rules ([#1142](https://github.com/casey/just/pull/1142) by [casey](https://github.com/casey))
- Remove stale link from readme ([#1141](https://github.com/casey/just/pull/1141) by [casey](https://github.com/casey))

### Unstable
- Search for missing recipes in parent directory justfiles ([#1149](https://github.com/casey/just/pull/1149) by [casey](https://github.com/casey))

[1.1.1](https://github.com/casey/just/releases/tag/1.1.1) - 2022-3-22
---------------------------------------------------------------------

### Misc
- Build MacOS ARM release binaries ([#1138](https://github.com/casey/just/pull/1138) by [casey](https://github.com/casey))
- Upgrade Windows Actions runners to windows-latest ([#1137](https://github.com/casey/just/pull/1137) by [casey](https://github.com/casey))

[1.1.0](https://github.com/casey/just/releases/tag/1.1.0) - 2022-3-10
---------------------------------------------------------------------

### Added
- Add `error()` function ([#1118](https://github.com/casey/just/pull/1118) by [chamons](https://github.com/chamons))
- Add `absolute_path` function ([#1121](https://github.com/casey/just/pull/1121) by [Laura7089](https://github.com/Laura7089))

[1.0.1](https://github.com/casey/just/releases/tag/1.0.1) - 2022-2-28
---------------------------------------------------------------------

### Fixed
- Make path_exists() relative to current directory ([#1122](https://github.com/casey/just/pull/1122) by [casey](https://github.com/casey))

### Misc
- Detail environment variable usage in readme ([#1086](https://github.com/casey/just/pull/1086) by [kenden](https://github.com/kenden))
- Format --init justfile ([#1116](https://github.com/casey/just/pull/1116) by [TheLocehiliosan](https://github.com/TheLocehiliosan))
- Add hint for Node.js script compatibility ([#1113](https://github.com/casey/just/pull/1113) by [casey](https://github.com/casey))

[1.0.0](https://github.com/casey/just/releases/tag/1.0.0) - 2022-2-22
---------------------------------------------------------------------

### Added
- Add path_exists() function ([#1106](https://github.com/casey/just/pull/1106) by [heavelock](https://github.com/heavelock))

### Misc
- Note that `pipefail` isn't normally set ([#1108](https://github.com/casey/just/pull/1108) by [casey](https://github.com/casey))

[0.11.2](https://github.com/casey/just/releases/tag/0.11.2) - 2022-2-15
-----------------------------------------------------------------------

### Misc
- Fix dotenv-load documentation ([#1104](https://github.com/casey/just/pull/1104) by [casey](https://github.com/casey))
- Fixup broken release package script ([#1100](https://github.com/casey/just/pull/1100) by [lutostag](https://github.com/lutostag))

[0.11.1](https://github.com/casey/just/releases/tag/0.11.1) - 2022-2-14
-----------------------------------------------------------------------

### Added
- Allow duplicate recipes ([#1095](https://github.com/casey/just/pull/1095) by [lutostag](https://github.com/lutostag))

### Misc
- Add arrow pointing to table of contents button ([#1096](https://github.com/casey/just/pull/1096) by [casey](https://github.com/casey))
- Improve readme ([#1093](https://github.com/casey/just/pull/1093) by [halostatue](https://github.com/halostatue))
- Remove asciidoc readme ([#1092](https://github.com/casey/just/pull/1092) by [casey](https://github.com/casey))
- Convert README.adoc to markdown ([#1091](https://github.com/casey/just/pull/1091) by [casey](https://github.com/casey))
- Add choco package to README ([#1090](https://github.com/casey/just/pull/1090) by [michidk](https://github.com/michidk))

[0.11.0](https://github.com/casey/just/releases/tag/0.11.0) - 2022-2-3
----------------------------------------------------------------------

### Breaking
- Change dotenv-load default to false ([#1082](https://github.com/casey/just/pull/1082) by [casey](https://github.com/casey))

[0.10.7](https://github.com/casey/just/releases/tag/0.10.7) - 2022-1-30
-----------------------------------------------------------------------

### Misc
- Don't run tests in release workflow ([#1080](https://github.com/casey/just/pull/1080) by [casey](https://github.com/casey))
- Fix windows chooser invocation error message test ([#1079](https://github.com/casey/just/pull/1079) by [casey](https://github.com/casey))
- Remove call to sed in justfile ([#1078](https://github.com/casey/just/pull/1078) by [casey](https://github.com/casey))

[0.10.6](https://github.com/casey/just/releases/tag/0.10.6) - 2022-1-29
-----------------------------------------------------------------------

### Added
- Add windows-powershell setting ([#1057](https://github.com/casey/just/pull/1057) by [michidk](https://github.com/michidk))

### Changed
- Allow using `-` and `@` in any order ([#1063](https://github.com/casey/just/pull/1063) by [casey](https://github.com/casey))

### Misc
- Use `Context` suffix for snafu error contexts ([#1068](https://github.com/casey/just/pull/1068) by [casey](https://github.com/casey))
- Upgrade snafu to 0.7 ([#1067](https://github.com/casey/just/pull/1067) by [shepmaster](https://github.com/shepmaster))
- Mention "$@" in the README ([#1064](https://github.com/casey/just/pull/1064) by [mpdude](https://github.com/mpdude))
- Note how to use PowerShell with CLI in readme ([#1056](https://github.com/casey/just/pull/1056) by [michidk](https://github.com/michidk))
- Link to cheatsheet from readme ([#1053](https://github.com/casey/just/pull/1053) by [casey](https://github.com/casey))
- Link to Homebrew installation docs in readme ([#1049](https://github.com/casey/just/pull/1049) by [michidk](https://github.com/michidk))
- Workflow tweaks ([#1045](https://github.com/casey/just/pull/1045) by [casey](https://github.com/casey))
- Push to correct origin in publish recipe ([#1044](https://github.com/casey/just/pull/1044) by [casey](https://github.com/casey))

[0.10.5](https://github.com/casey/just/releases/tag/0.10.5) - 2021-12-4
-----------------------------------------------------------------------

### Changed
- Use musl libc for ARM binaries ([#1037](https://github.com/casey/just/pull/1037) by [casey](https://github.com/casey))

### Misc
- Make completions work with Bash alias ([#1035](https://github.com/casey/just/pull/1035) by [kurtbuilds](https://github.com/kurtbuilds))
- Run tests on PRs ([#1040](https://github.com/casey/just/pull/1040) by [casey](https://github.com/casey))
- Improve GitHub Actions workflow triggers ([#1033](https://github.com/casey/just/pull/1033) by [casey](https://github.com/casey))
- Publish from GitHub master branch instead of local master ([#1032](https://github.com/casey/just/pull/1032) by [casey](https://github.com/casey))

[0.10.4](https://github.com/casey/just/releases/tag/0.10.4) - 2021-11-21
------------------------------------------------------------------------

### Added
- Add `--dump-format json` ([#992](https://github.com/casey/just/pull/992) by [casey](https://github.com/casey))
- Add `quote(s)` function for escaping strings ([#1022](https://github.com/casey/just/pull/1022) by [casey](https://github.com/casey))
- fmt: check formatting with `--check` ([#1001](https://github.com/casey/just/pull/1001) by [hdhoang](https://github.com/hdhoang))

### Misc
- Refactor github actions ([#1028](https://github.com/casey/just/pull/1028) by [casey](https://github.com/casey))
- Fix readme formatting ([#1030](https://github.com/casey/just/pull/1030) by [soenkehahn](https://github.com/soenkehahn))
- Use ps1 extension for pwsh shebangs ([#1027](https://github.com/casey/just/pull/1027) by [dmringo](https://github.com/dmringo))
- Ignore leading byte order mark in source files ([#1021](https://github.com/casey/just/pull/1021) by [casey](https://github.com/casey))
- Add color to `just --fmt --check` diff ([#1015](https://github.com/casey/just/pull/1015) by [casey](https://github.com/casey))

[0.10.3](https://github.com/casey/just/releases/tag/0.10.3) - 2021-10-30
------------------------------------------------------------------------

### Added
- Add `trim_end(s)` and `trim_start(s)` functions ([#999](https://github.com/casey/just/pull/999) by [casey](https://github.com/casey))
- Add more string manipulation functions ([#998](https://github.com/casey/just/pull/998) by [casey](https://github.com/casey))

### Changed
- Make `join` accept two or more arguments ([#1000](https://github.com/casey/just/pull/1000) by [casey](https://github.com/casey))

### Misc
- Add alternatives and prior art section to readme ([#1008](https://github.com/casey/just/pull/1008) by [casey](https://github.com/casey))
- Fix readme `make`'s not correctly displayed ([#1007](https://github.com/casey/just/pull/1007) by [peter50216](https://github.com/peter50216))
- Document the default recipe ([#1006](https://github.com/casey/just/pull/1006) by [casey](https://github.com/casey))
- Document creating user justfile recipe aliases ([#1005](https://github.com/casey/just/pull/1005) by [casey](https://github.com/casey))
- Fix readme typo ([#1004](https://github.com/casey/just/pull/1004) by [0xflotus](https://github.com/0xflotus))
- Add packaging status table to readme ([#1003](https://github.com/casey/just/pull/1003) by [casey](https://github.com/casey))
- Reword `sh` not found error messages ([#1002](https://github.com/casey/just/pull/1002) by [hdhoang](https://github.com/hdhoang))
- Only pass +crt-static to cargo build ([#997](https://github.com/casey/just/pull/997) by [casey](https://github.com/casey))
- Stop using tabs in justfile in editorconfig ([#996](https://github.com/casey/just/pull/996) by [casey](https://github.com/casey))
- Use consistent rustflags formatting ([#994](https://github.com/casey/just/pull/994) by [casey](https://github.com/casey))
- Use `cargo build` instead of `cargo rustc` ([#993](https://github.com/casey/just/pull/993) by [casey](https://github.com/casey))
- Don't skip variables in variable iterator ([#991](https://github.com/casey/just/pull/991) by [casey](https://github.com/casey))
- Remove deprecated equals error ([#985](https://github.com/casey/just/pull/985) by [casey](https://github.com/casey))

[0.10.2](https://github.com/casey/just/releases/tag/0.10.2) - 2021-9-26
-----------------------------------------------------------------------

### Added
- Implement regular expression match conditionals ([#970](https://github.com/casey/just/pull/970) by [casey](https://github.com/casey))

### Misc
- Add detailed instructions for installing prebuilt binaries ([#978](https://github.com/casey/just/pull/978) by [casey](https://github.com/casey))
- Improve readme package table formatting ([#977](https://github.com/casey/just/pull/977) by [casey](https://github.com/casey))
- Add conda package to README ([#976](https://github.com/casey/just/pull/976) by [kellpossible](https://github.com/kellpossible))
- Change MSRV to 1.46.0 ([#968](https://github.com/casey/just/pull/968) by [casey](https://github.com/casey))
- Use stable rustfmt instead of nightly ([#967](https://github.com/casey/just/pull/967) by [casey](https://github.com/casey))
- Fix readme typo: FOO → WORLD ([#964](https://github.com/casey/just/pull/964) by [casey](https://github.com/casey))
- Reword Emacs section in readme ([#962](https://github.com/casey/just/pull/962) by [casey](https://github.com/casey))
- Mention justl mode for Emacs ([#961](https://github.com/casey/just/pull/961) by [psibi](https://github.com/psibi))

[0.10.1](https://github.com/casey/just/releases/tag/0.10.1) - 2021-8-27
-----------------------------------------------------------------------

### Added
- Add flags for specifying name and path to environment file ([#941](https://github.com/casey/just/pull/941) by [Celeo](https://github.com/Celeo))

### Misc
- Fix error message tests for Alpine Linux ([#956](https://github.com/casey/just/pull/956) by [casey](https://github.com/casey))
- Bump `target` version to 2.0 ([#957](https://github.com/casey/just/pull/957) by [casey](https://github.com/casey))
- Mention `tree-sitter-just` in readme ([#951](https://github.com/casey/just/pull/951) by [casey](https://github.com/casey))
- Document release RSS feed in readme ([#950](https://github.com/casey/just/pull/950) by [casey](https://github.com/casey))
- Add installation instructions for Gentoo Linux ([#946](https://github.com/casey/just/pull/946) by [dm9pZCAq](https://github.com/dm9pZCAq))
- Make GitHub Actions instructions more prominent ([#944](https://github.com/casey/just/pull/944) by [casey](https://github.com/casey))
- Wrap `--help` text to terminal width ([#940](https://github.com/casey/just/pull/940) by [casey](https://github.com/casey))
- Add `.justfile` to sublime syntax file_extensions ([#938](https://github.com/casey/just/pull/938) by [casey](https://github.com/casey))
- Suggest using `~/.global.justfile` instead of `~/.justfile` ([#937](https://github.com/casey/just/pull/937) by [casey](https://github.com/casey))
- Update man page ([#935](https://github.com/casey/just/pull/935) by [casey](https://github.com/casey))

[0.10.0](https://github.com/casey/just/releases/tag/0.10.0) - 2021-8-2
----------------------------------------------------------------------

### Changed
- Warn if `.env` file is loaded in `dotenv-load` isn't explicitly set ([#925](https://github.com/casey/just/pull/925) by [casey](https://github.com/casey))

### Added
- Add `--changelog` subcommand ([#932](https://github.com/casey/just/pull/932) by [casey](https://github.com/casey))
- Support `.justfile` as an alternative to `justfile` ([#931](https://github.com/casey/just/pull/931) by [casey](https://github.com/casey))

### Misc
- Use cargo-limit for all recipes ([#928](https://github.com/casey/just/pull/928) by [casey](https://github.com/casey))
- Fix colors ([#927](https://github.com/casey/just/pull/927) by [casey](https://github.com/casey))
- Use ColorDisplay trait to print objects to the terminal ([#926](https://github.com/casey/just/pull/926) by [casey](https://github.com/casey))
- Deduplicate recipe parsing ([#923](https://github.com/casey/just/pull/923) by [casey](https://github.com/casey))
- Move subcommand functions into Subcommand ([#918](https://github.com/casey/just/pull/918) by [casey](https://github.com/casey))
- Check GitHub Actions workflow with actionlint ([#921](https://github.com/casey/just/pull/921) by [casey](https://github.com/casey))
- Add loader and refactor errors ([#917](https://github.com/casey/just/pull/917) by [casey](https://github.com/casey))
- Rename: Module → Ast ([#915](https://github.com/casey/just/pull/915) by [casey](https://github.com/casey))

[0.9.9](https://github.com/casey/just/releases/tag/0.9.9) - 2021-7-22
---------------------------------------------------------------------

### Added
- Add subsequent dependencies ([#820](https://github.com/casey/just/pull/820) by [casey](https://github.com/casey))
- Implement `else if` chaining ([#910](https://github.com/casey/just/pull/910) by [casey](https://github.com/casey))

### Fixed
- Fix circular variable dependency error message ([#909](https://github.com/casey/just/pull/909) by [casey](https://github.com/casey))

### Misc
- Improve readme ([#904](https://github.com/casey/just/pull/904) by [mtsknn](https://github.com/mtsknn))
- Add screenshot to readme ([#911](https://github.com/casey/just/pull/911) by [casey](https://github.com/casey))
- Add install instructions for Fedora Linux ([#898](https://github.com/casey/just/pull/898) by [olivierlemasle](https://github.com/olivierlemasle))
- Fix readme typos ([#903](https://github.com/casey/just/pull/903) by [rokf](https://github.com/rokf))
- Actually fix release tagging and publish changelog with releases ([#901](https://github.com/casey/just/pull/901) by [casey](https://github.com/casey))
- Fix broken prerelease tagging ([#900](https://github.com/casey/just/pull/900) by [casey](https://github.com/casey))
- Use string value for ref-type check ([#897](https://github.com/casey/just/pull/897) by [casey](https://github.com/casey))

[0.9.8](https://github.com/casey/just/releases/tag/0.9.8) - 2021-7-3
--------------------------------------------------------------------

### Misc
- Fix changelog formatting ([#894](https://github.com/casey/just/pull/894) by [casey](https://github.com/casey))
- Only run install script on CI for non-releases ([#895](https://github.com/casey/just/pull/895) by [casey](https://github.com/casey))

[0.9.7](https://github.com/casey/just/releases/tag/0.9.7) - 2021-7-3
--------------------------------------------------------------------

### Added
- Add string manipulation functions ([#888](https://github.com/casey/just/pull/888) by [terror](https://github.com/terror))

### Misc
- Remove test-utilities crate ([#892](https://github.com/casey/just/pull/892) by [casey](https://github.com/casey))
- Remove outdated note in `Cargo.toml` ([#891](https://github.com/casey/just/pull/891) by [casey](https://github.com/casey))
- Link to GitHub release pages in changelog ([#886](https://github.com/casey/just/pull/886) by [casey](https://github.com/casey))

[0.9.6](https://github.com/casey/just/releases/tag/0.9.6) - 2021-6-24
---------------------------------------------------------------------

### Added
- Add `clean` function for simplifying paths ([#883](https://github.com/casey/just/pull/883) by [casey](https://github.com/casey))
- Add `join` function for joining paths ([#882](https://github.com/casey/just/pull/882) by [casey](https://github.com/casey))
- Add path manipulation functions ([#872](https://github.com/casey/just/pull/872) by [TonioGela](https://github.com/TonioGela))

### Misc
- Add `file_extensions` to Sublime syntax file ([#878](https://github.com/casey/just/pull/878) by [Frederick888](https://github.com/Frederick888))
- Document path manipulation functions in readme ([#877](https://github.com/casey/just/pull/877) by [casey](https://github.com/casey))

[0.9.5](https://github.com/casey/just/releases/tag/0.9.5) - 2021-6-12
---------------------------------------------------------------------

### Added
- Add `--unstable` flag ([#869](https://github.com/casey/just/pull/869) by [casey](https://github.com/casey))
- Add Sublime Text syntax file ([#864](https://github.com/casey/just/pull/864) by [casey](https://github.com/casey))
- Add `--fmt` subcommand ([#837](https://github.com/casey/just/pull/837) by [vglfr](https://github.com/vglfr))

### Misc
- Mention doniogela.dev/just/ in readme ([#866](https://github.com/casey/just/pull/866) by [casey](https://github.com/casey))
- Mention that vim-just is now available from vim-polyglot ([#865](https://github.com/casey/just/pull/865) by [casey](https://github.com/casey))
- Mention `--list-heading` newline behavior ([#860](https://github.com/casey/just/pull/860) by [casey](https://github.com/casey))
- Check for `rg` in `bin/forbid` ([#859](https://github.com/casey/just/pull/859) by [casey](https://github.com/casey))
- Document that variables are not exported to backticks in the same scope ([#856](https://github.com/casey/just/pull/856) by [casey](https://github.com/casey))
- Remove `dotenv_load` from tests ([#853](https://github.com/casey/just/pull/853) by [casey](https://github.com/casey))
- Remove `v` prefix from version ([#850](https://github.com/casey/just/pull/850) by [casey](https://github.com/casey))
- Improve install script ([#847](https://github.com/casey/just/pull/847) by [casey](https://github.com/casey))
- Move pages assets back to `docs` ([#846](https://github.com/casey/just/pull/846) by [casey](https://github.com/casey))
- Move pages assets to `www` ([#845](https://github.com/casey/just/pull/845) by [casey](https://github.com/casey))

[0.9.4](https://github.com/casey/just/releases/tag/v0.9.4) - 2021-5-27
----------------------------------------------------------------------

### Misc
- Release `aarch64-unknown-linux-gnu` binaries ([#843](https://github.com/casey/just/pull/843) by [casey](https://github.com/casey))
- Add `$` to non-default parameter grammar ([#839](https://github.com/casey/just/pull/839) by [casey](https://github.com/casey))
- Add `$` to parameter grammar ([#838](https://github.com/casey/just/pull/838) by [NoahTheDuke](https://github.com/NoahTheDuke))
- Fix readme links ([#836](https://github.com/casey/just/pull/836) by [casey](https://github.com/casey))
- Add `vim-just` installation instructions to readme ([#835](https://github.com/casey/just/pull/835) by [casey](https://github.com/casey))
- Refactor shebang handling ([#833](https://github.com/casey/just/pull/833) by [casey](https://github.com/casey))

[0.9.3](https://github.com/casey/just/releases/tag/v0.9.3) - 2021-5-16
----------------------------------------------------------------------

### Added
- Add shebang support for 'cmd.exe' ([#828](https://github.com/casey/just/pull/828) by [pansila](https://github.com/pansila))
- Add `.exe` to powershell scripts ([#826](https://github.com/casey/just/pull/826) by [sigoden](https://github.com/sigoden))
- Add the `--command` subcommand ([#824](https://github.com/casey/just/pull/824) by [casey](https://github.com/casey))

### Fixed
- Fix bang lexing and placate clippy ([#821](https://github.com/casey/just/pull/821) by [casey](https://github.com/casey))

### Misc
- Fixed missing close apostrophe in GRAMMAR.md ([#830](https://github.com/casey/just/pull/830) by [SOF3](https://github.com/SOF3))
- Make 'else' keyword in grammar ([#829](https://github.com/casey/just/pull/829) by [SOF3](https://github.com/SOF3))
- Add forbid script ([#827](https://github.com/casey/just/pull/827) by [casey](https://github.com/casey))
- Remove `summary` feature ([#823](https://github.com/casey/just/pull/823) by [casey](https://github.com/casey))
- Document that just is now in Arch official repo ([#814](https://github.com/casey/just/pull/814) by [svenstaro](https://github.com/svenstaro))
- Fix changelog years ([#813](https://github.com/casey/just/pull/813) by [casey](https://github.com/casey))

[0.9.2](https://github.com/casey/just/releases/tag/v0.9.2) - 2021-5-02
----------------------------------------------------------------------

### Fixed
- Pass evaluated arguments as positional arguments ([#810](https://github.com/casey/just/pull/810) by [casey](https://github.com/casey))

[0.9.1](https://github.com/casey/just/releases/tag/v0.9.1) - 2021-4-24
----------------------------------------------------------------------

### Added
- Change `--eval` to print variable value only ([#806](https://github.com/casey/just/pull/806) by [casey](https://github.com/casey))
- Add `positional-arguments` setting ([#804](https://github.com/casey/just/pull/804) by [casey](https://github.com/casey))
- Allow filtering variables to evaluate ([#795](https://github.com/casey/just/pull/795) by [casey](https://github.com/casey))

### Changed
- Reform and improve string literals ([#793](https://github.com/casey/just/pull/793) by [casey](https://github.com/casey))
- Allow evaluating justfiles with no recipes ([#794](https://github.com/casey/just/pull/794) by [casey](https://github.com/casey))
- Unify string lexing ([#790](https://github.com/casey/just/pull/790) by [casey](https://github.com/casey))

### Misc
- Test multi-line strings in interpolation ([#789](https://github.com/casey/just/pull/789) by [casey](https://github.com/casey))
- Add shell setting examples to README ([#787](https://github.com/casey/just/pull/787) by [casey](https://github.com/casey))
- Disable .env warning for now ([#786](https://github.com/casey/just/pull/786) by [casey](https://github.com/casey))
- Warn if `.env` file loaded and `dotenv-load` unset ([#784](https://github.com/casey/just/pull/784) by [casey](https://github.com/casey))

[0.9.0](https://github.com/casey/just/releases/tag/v0.9.0) - 2021-3-28
----------------------------------------------------------------------

### Changed
- Turn `=` deprecation warning into a hard error ([#780](https://github.com/casey/just/pull/780) by [casey](https://github.com/casey))

[0.8.7](https://github.com/casey/just/releases/tag/v0.8.7) - 2021-3-28
----------------------------------------------------------------------

### Added
- Add `dotenv-load` setting ([#778](https://github.com/casey/just/pull/778) by [casey](https://github.com/casey))

### Misc
- Change publish recipe to use stable rust ([#777](https://github.com/casey/just/pull/777) by [casey](https://github.com/casey))

[0.8.6](https://github.com/casey/just/releases/tag/v0.8.6) - 2021-3-28
----------------------------------------------------------------------

### Added
- Add just_executable() function ([#775](https://github.com/casey/just/pull/775) by [bew](https://github.com/bew))
- Prefix parameters with `$` to export to environment ([#773](https://github.com/casey/just/pull/773) by [casey](https://github.com/casey))
- Add `set export` to export all variables as environment variables ([#767](https://github.com/casey/just/pull/767) by [casey](https://github.com/casey))

### Changed
- Suppress all output to stderr when `--quiet` ([#771](https://github.com/casey/just/pull/771) by [casey](https://github.com/casey))

### Misc
- Improve chooser invocation error message ([#772](https://github.com/casey/just/pull/772) by [casey](https://github.com/casey))
- De-emphasize cmd.exe in readme ([#768](https://github.com/casey/just/pull/768) by [casey](https://github.com/casey))
- Fix warnings ([#770](https://github.com/casey/just/pull/770) by [casey](https://github.com/casey))

[0.8.5](https://github.com/casey/just/releases/tag/v0.8.5) - 2021-3-24
----------------------------------------------------------------------

### Added
- Allow escaping double braces with `{{{{` ([#765](https://github.com/casey/just/pull/765) by [casey](https://github.com/casey))

### Misc
- Reorganize readme to highlight editor support ([#764](https://github.com/casey/just/pull/764) by [casey](https://github.com/casey))
- Add categories and keywords to Cargo manifest ([#763](https://github.com/casey/just/pull/763) by [casey](https://github.com/casey))
- Fix command output in readme ([#760](https://github.com/casey/just/pull/760) by [vvv](https://github.com/vvv))
- Note Emacs package `just-mode` in readme ([#759](https://github.com/casey/just/pull/759) by [leon-barrett](https://github.com/leon-barrett))
- Note shebang line splitting inconsistency in readme ([#757](https://github.com/casey/just/pull/757) by [casey](https://github.com/casey))

[0.8.4](https://github.com/casey/just/releases/tag/v0.8.4) - 2021-2-9
---------------------------------------------------------------------

### Added
- Add options to control list formatting ([#753](https://github.com/casey/just/pull/753) by [casey](https://github.com/casey))

### Misc
- Document how to change the working directory in a recipe ([#752](https://github.com/casey/just/pull/752) by [casey](https://github.com/casey))
- Implement `Default` for `Table` ([#748](https://github.com/casey/just/pull/748) by [casey](https://github.com/casey))
- Add Alpine Linux package to readme ([#736](https://github.com/casey/just/pull/736) by [jirutka](https://github.com/jirutka))
- Update to actions/cache@v2 ([#742](https://github.com/casey/just/pull/742) by [zyctree](https://github.com/zyctree))
- Add link in readme to GitHub Action ([#729](https://github.com/casey/just/pull/729) by [rossmacarthur](https://github.com/rossmacarthur))
- Add docs for justfile() and justfile_directory() ([#726](https://github.com/casey/just/pull/726) by [rminderhoud](https://github.com/rminderhoud))
- Fix CI ([#727](https://github.com/casey/just/pull/727) by [casey](https://github.com/casey))
- Improve readme ([#725](https://github.com/casey/just/pull/725) by [casey](https://github.com/casey))
- Replace saythanks.io link with malto: link ([#723](https://github.com/casey/just/pull/723) by [casey](https://github.com/casey))
- Update man page to v0.8.3 ([#720](https://github.com/casey/just/pull/720) by [casey](https://github.com/casey))

[0.8.3](https://github.com/casey/just/releases/tag/v0.8.3) - 2020-10-27
-----------------------------------------------------------------------

### Added
- Allow ignoring line endings inside delimiters ([#717](https://github.com/casey/just/pull/717) by [casey](https://github.com/casey))

[0.8.2](https://github.com/casey/just/releases/tag/v0.8.2) - 2020-10-26
-----------------------------------------------------------------------

### Added
- Add conditional expressions ([#714](https://github.com/casey/just/pull/714) by [casey](https://github.com/casey))

### Fixed
- Allow completing variables and recipes after `--set` in zsh completion script ([#697](https://github.com/casey/just/pull/697) by [heyrict](https://github.com/heyrict))

### Misc
- Add Parser::forbid ([#712](https://github.com/casey/just/pull/712) by [casey](https://github.com/casey))
- Automatically track expected tokens while parsing ([#711](https://github.com/casey/just/pull/711) by [casey](https://github.com/casey))
- Document feature flags in Cargo.toml ([#709](https://github.com/casey/just/pull/709) by [casey](https://github.com/casey))

[0.8.1](https://github.com/casey/just/releases/tag/v0.8.1) - 2020-10-15
-----------------------------------------------------------------------

### Changed
- Allow choosing multiple recipes to run ([#700](https://github.com/casey/just/pull/700) by [casey](https://github.com/casey))
- Complete recipes in bash completion script ([#685](https://github.com/casey/just/pull/685) by [vikesh-raj](https://github.com/vikesh-raj))
- Complete recipes names in PowerShell completion script ([#651](https://github.com/casey/just/pull/651) by [Insomniak47](https://github.com/Insomniak47))

### Misc
- Add FreeBSD port to readme ([#705](https://github.com/casey/just/pull/705) by [casey](https://github.com/casey))
- Placate clippy ([#698](https://github.com/casey/just/pull/698) by [casey](https://github.com/casey))
- Fix build fix ([#693](https://github.com/casey/just/pull/693) by [casey](https://github.com/casey))
- Fix readme documentation for ignoring errors ([#692](https://github.com/casey/just/pull/692) by [kenden](https://github.com/kenden))

[0.8.0](https://github.com/casey/just/releases/tag/v0.8.0) - 2020-10-3
----------------------------------------------------------------------

### Breaking
- Allow suppressing failures with `-` prefix ([#687](https://github.com/casey/just/pull/687) by [iwillspeak](https://github.com/iwillspeak))

### Misc
- Document how to ignore errors with `-` in readme ([#690](https://github.com/casey/just/pull/690) by [casey](https://github.com/casey))
- Install BSD Tar on GitHub Actions to fix CI errors ([#689](https://github.com/casey/just/pull/689) by [casey](https://github.com/casey))
- Move separate quiet config value to verbosity ([#686](https://github.com/casey/just/pull/686) by [Celeo](https://github.com/Celeo))

[0.7.3](https://github.com/casey/just/releases/tag/v0.7.3) - 2020-9-17
----------------------------------------------------------------------

### Added
- Add the `--choose` subcommand ([#680](https://github.com/casey/just/pull/680) by [casey](https://github.com/casey))

### Misc
- Combine integration tests into single binary ([#679](https://github.com/casey/just/pull/679) by [casey](https://github.com/casey))
- Document `--unsorted` flag in readme ([#672](https://github.com/casey/just/pull/672) by [casey](https://github.com/casey))

[0.7.2](https://github.com/casey/just/releases/tag/v0.7.2) - 2020-8-23
----------------------------------------------------------------------

### Added
- Add option to print recipes in source order ([#669](https://github.com/casey/just/pull/669) by [casey](https://github.com/casey))

### Misc
- Mention Linux, MacOS and Windows support in readme ([#666](https://github.com/casey/just/pull/666) by [casey](https://github.com/casey))
- Add list highlighting nice features to readme ([#664](https://github.com/casey/just/pull/664) by [casey](https://github.com/casey))

[0.7.1](https://github.com/casey/just/releases/tag/v0.7.1) - 2020-7-19
----------------------------------------------------------------------

### Fixed
- Search for `.env` file from working directory ([#661](https://github.com/casey/just/pull/661) by [casey](https://github.com/casey))

### Misc
- Move link-time optimization config into `Cargo.toml` ([#658](https://github.com/casey/just/pull/658) by [casey](https://github.com/casey))

[0.7.0](https://github.com/casey/just/releases/tag/v0.7.0) - 2020-7-16
----------------------------------------------------------------------

### Breaking
- Skip `.env` items which are set in environment ([#656](https://github.com/casey/just/pull/656) by [casey](https://github.com/casey))

### Misc
- Mark tags that start with `v` as releases ([#654](https://github.com/casey/just/pull/654) by [casey](https://github.com/casey))

[0.6.1](https://github.com/casey/just/releases/tag/v0.6.1) - 2020-6-28
----------------------------------------------------------------------

### Changed
- Only use `cygpath` on shebang if it contains `/` ([#652](https://github.com/casey/just/pull/652) by [casey](https://github.com/casey))

[0.6.0](https://github.com/casey/just/releases/tag/v0.6.0) - 2020-6-18
----------------------------------------------------------------------

### Changed
- Ignore '@' returned from interpolation evaluation ([#636](https://github.com/casey/just/pull/636) by [rjsberry](https://github.com/rjsberry))
- Strip leading spaces after line continuation ([#635](https://github.com/casey/just/pull/635) by [casey](https://github.com/casey))

### Added
- Add variadic parameters that accept zero or more arguments ([#645](https://github.com/casey/just/pull/645) by [rjsberry](https://github.com/rjsberry))

### Misc
- Clarify variadic parameter default values ([#646](https://github.com/casey/just/pull/646) by [rjsberry](https://github.com/rjsberry))
- Add keybase example justfile  ([#640](https://github.com/casey/just/pull/640) by [blaggacao](https://github.com/blaggacao))
- Strip trailing whitespace in `examples/pre-commit.just` ([#644](https://github.com/casey/just/pull/644) by [casey](https://github.com/casey))
- Test that example justfiles successfully parse ([#643](https://github.com/casey/just/pull/643) by [casey](https://github.com/casey))
- Link example justfiles in readme ([#641](https://github.com/casey/just/pull/641) by [casey](https://github.com/casey))
- Add example justfile ([#639](https://github.com/casey/just/pull/639) by [blaggacao](https://github.com/blaggacao))
- Document how to run recipes after another recipe ([#630](https://github.com/casey/just/pull/630) by [casey](https://github.com/casey))

[0.5.11](https://github.com/casey/just/releases/tag/v0.5.11) - 2020-5-23
------------------------------------------------------------------------

### Added
- Don't load `.env` file when `--no-dotenv` is passed ([#627](https://github.com/casey/just/pull/627) by [casey](https://github.com/casey))

### Changed
- Complete recipe names in fish completion script ([#625](https://github.com/casey/just/pull/625) by [tyehle](https://github.com/tyehle))
- Suggest aliases for unknown recipes ([#624](https://github.com/casey/just/pull/624) by [Celeo](https://github.com/Celeo))

[0.5.10](https://github.com/casey/just/releases/tag/v0.5.10) - 2020-3-18
------------------------------------------------------------------------

[0.5.9](https://github.com/casey/just/releases/tag/v0.5.9) - 2020-3-18
----------------------------------------------------------------------

### Added
- Update zsh completion file ([#606](https://github.com/casey/just/pull/606) by [heyrict](https://github.com/heyrict))
- Add `--variables` subcommand that prints variable names ([#608](https://github.com/casey/just/pull/608) by [casey](https://github.com/casey))
- Add github pages site with improved install script ([#597](https://github.com/casey/just/pull/597) by [casey](https://github.com/casey))

### Fixed
- Don't require justfile to print completions ([#596](https://github.com/casey/just/pull/596) by [casey](https://github.com/casey))

### Misc
- Only build for linux on docs.rs ([#611](https://github.com/casey/just/pull/611) by [casey](https://github.com/casey))
- Trim completions and ensure final newline ([#609](https://github.com/casey/just/pull/609) by [casey](https://github.com/casey))
- Trigger build on pushes and pull requests ([#607](https://github.com/casey/just/pull/607) by [casey](https://github.com/casey))
- Document behavior of `@` on shebang recipes ([#602](https://github.com/casey/just/pull/602) by [casey](https://github.com/casey))
- Add `.nojekyll` file to github pages site ([#599](https://github.com/casey/just/pull/599) by [casey](https://github.com/casey))
- Add `:` favicon ([#598](https://github.com/casey/just/pull/598) by [casey](https://github.com/casey))
- Delete old CI configuration and update build badge ([#595](https://github.com/casey/just/pull/595) by [casey](https://github.com/casey))
- Add download count badge to readme ([#594](https://github.com/casey/just/pull/594) by [casey](https://github.com/casey))
- Wrap comments at 80 characters ([#593](https://github.com/casey/just/pull/593) by [casey](https://github.com/casey))
- Use unstable rustfmt configuration options ([#592](https://github.com/casey/just/pull/592) by [casey](https://github.com/casey))

[0.5.8](https://github.com/casey/just/releases/tag/v0.5.8) - 2020-1-28
----------------------------------------------------------------------

### Changed
- Only use `cygpath` on windows if present ([#586](https://github.com/casey/just/pull/586) by [casey](https://github.com/casey))

### Misc
- Improve comments in justfile ([#588](https://github.com/casey/just/pull/588) by [casey](https://github.com/casey))
- Remove unused dependencies ([#587](https://github.com/casey/just/pull/587) by [casey](https://github.com/casey))

[0.5.7](https://github.com/casey/just/releases/tag/v0.5.7) - 2020-1-28
----------------------------------------------------------------------

### Misc
- Don't include directories in release archive ([#583](https://github.com/casey/just/pull/583) by [casey](https://github.com/casey))

[0.5.6](https://github.com/casey/just/releases/tag/v0.5.6) - 2020-1-28
----------------------------------------------------------------------

### Misc
- Build and upload release artifacts from GitHub Actions ([#581](https://github.com/casey/just/pull/581) by [casey](https://github.com/casey))
- List solus package in readme ([#579](https://github.com/casey/just/pull/579) by [casey](https://github.com/casey))
- Expand use of GitHub Actions ([#580](https://github.com/casey/just/pull/580) by [casey](https://github.com/casey))
- Fix readme typo: interpetation -> interpretation ([#578](https://github.com/casey/just/pull/578) by [Plommonsorbet](https://github.com/Plommonsorbet))

[0.5.5](https://github.com/casey/just/releases/tag/v0.5.5) - 2020-1-15
----------------------------------------------------------------------

### Added
- Generate shell completion scripts with `--completions` ([#572](https://github.com/casey/just/pull/572) by [casey](https://github.com/casey))

### Misc
- Check long lines and FIXME/TODO on CI ([#575](https://github.com/casey/just/pull/575) by [casey](https://github.com/casey))
- Add additional continuous integration checks ([#574](https://github.com/casey/just/pull/574) by [casey](https://github.com/casey))

[0.5.4](https://github.com/casey/just/releases/tag/v0.5.4) - 2019-12-25
-----------------------------------------------------------------------

### Added
- Add `justfile_directory()` and `justfile()` ([#569](https://github.com/casey/just/pull/569) by [casey](https://github.com/casey))

### Misc
- Add table of package managers that include just to readme ([#568](https://github.com/casey/just/pull/568) by [casey](https://github.com/casey))
- Remove yaourt AUR helper from readme ([#567](https://github.com/casey/just/pull/567) by [ky0n](https://github.com/ky0n))
- Fix regression in error message color printing ([#566](https://github.com/casey/just/pull/566) by [casey](https://github.com/casey))
- Reform indentation handling ([#565](https://github.com/casey/just/pull/565) by [casey](https://github.com/casey))
- Update Cargo.lock with new version ([#564](https://github.com/casey/just/pull/564) by [casey](https://github.com/casey))

[0.5.3](https://github.com/casey/just/releases/tag/v0.5.3) - 2019-12-11
-----------------------------------------------------------------------

### Misc
- Assert that lexer advances over entire input ([#560](https://github.com/casey/just/pull/560) by [casey](https://github.com/casey))
- Fix typo: `chracter` -> `character` ([#561](https://github.com/casey/just/pull/561) by [casey](https://github.com/casey))
- Improve pre-publish check ([#562](https://github.com/casey/just/pull/562) by [casey](https://github.com/casey))

[0.5.2](https://github.com/casey/just/releases/tag/v0.5.2) - 2019-12-7
----------------------------------------------------------------------

### Added
- Add flags to set and clear shell arguments ([#551](https://github.com/casey/just/pull/551) by [casey](https://github.com/casey))
- Allow passing arguments to dependencies ([#555](https://github.com/casey/just/pull/555) by [casey](https://github.com/casey))

### Misc
- Un-implement Deref for Table ([#546](https://github.com/casey/just/pull/546) by [casey](https://github.com/casey))
- Resolve recipe dependencies ([#547](https://github.com/casey/just/pull/547) by [casey](https://github.com/casey))
- Resolve alias targets ([#548](https://github.com/casey/just/pull/548) by [casey](https://github.com/casey))
- Remove unnecessary type argument to Alias ([#549](https://github.com/casey/just/pull/549) by [casey](https://github.com/casey))
- Resolve functions ([#550](https://github.com/casey/just/pull/550) by [casey](https://github.com/casey))
- Reform scope and binding ([#556](https://github.com/casey/just/pull/556) by [casey](https://github.com/casey))

[0.5.1](https://github.com/casey/just/releases/tag/v0.5.1) - 2019-11-20
-----------------------------------------------------------------------

### Added
- Add `--init` subcommand ([#541](https://github.com/casey/just/pull/541) by [casey](https://github.com/casey))

### Changed
- Avoid fs::canonicalize ([#539](https://github.com/casey/just/pull/539) by [casey](https://github.com/casey))

### Misc
- Mention `set shell` as alternative to installing `sh` ([#533](https://github.com/casey/just/pull/533) by [casey](https://github.com/casey))
- Refactor Compilation error to contain a Token ([#535](https://github.com/casey/just/pull/535) by [casey](https://github.com/casey))
- Move lexer comment ([#536](https://github.com/casey/just/pull/536) by [casey](https://github.com/casey))
- Add missing `--init` test ([#543](https://github.com/casey/just/pull/543) by [casey](https://github.com/casey))

[0.5.0](https://github.com/casey/just/releases/tag/v0.5.0) - 2019-11-12
-----------------------------------------------------------------------

### Added

- Add `set shell := [...]` to grammar ([#526](https://github.com/casey/just/pull/526) by [casey](https://github.com/casey))
- Add `shell` setting ([#525](https://github.com/casey/just/pull/525) by [casey](https://github.com/casey))
- Document settings in readme ([#527](https://github.com/casey/just/pull/527) by [casey](https://github.com/casey))

### Changed
- Reform positional argument parsing ([#523](https://github.com/casey/just/pull/523) by [casey](https://github.com/casey))
- Highlight echoed recipe lines in bold by default ([#512](https://github.com/casey/just/pull/512) by [casey](https://github.com/casey))

### Misc

- Gargantuan refactor ([#522](https://github.com/casey/just/pull/522) by [casey](https://github.com/casey))
- Move subcommand execution into Subcommand ([#514](https://github.com/casey/just/pull/514) by [casey](https://github.com/casey))
- Move `cd` out of Config::from_matches ([#513](https://github.com/casey/just/pull/513) by [casey](https://github.com/casey))
- Remove now-unnecessary borrow checker appeasement ([#511](https://github.com/casey/just/pull/511) by [casey](https://github.com/casey))
- Reform Parser ([#509](https://github.com/casey/just/pull/509) by [casey](https://github.com/casey))
- Note need to publish with nightly cargo ([#506](https://github.com/casey/just/pull/506) by [casey](https://github.com/casey))

[0.4.5](https://github.com/casey/just/releases/tag/v0.4.5) - 2019-10-31
-----------------------------------------------------------------------

### User-visible

### Changed
- Display alias with `--show NAME` if one exists ([#466](https://github.com/casey/just/pull/466) by [casey](https://github.com/casey))

### Documented
- Document multi-line constructs (for/if/while) ([#453](https://github.com/casey/just/pull/453) by [casey](https://github.com/casey))
- Generate man page with help2man ([#463](https://github.com/casey/just/pull/463) by [casey](https://github.com/casey))
- Add context to deprecation warnings ([#473](https://github.com/casey/just/pull/473) by [casey](https://github.com/casey))
- Improve messages for alias error messages ([#500](https://github.com/casey/just/pull/500) by [casey](https://github.com/casey))

### Misc

### Cleanup
- Update deprecated rust range patterns and clippy config ([#450](https://github.com/casey/just/pull/450) by [light4](https://github.com/light4))
- Make comments in common.rs lowercase ([#470](https://github.com/casey/just/pull/470) by [casey](https://github.com/casey))
- Use `pub(crate)` instead of `pub` ([#471](https://github.com/casey/just/pull/471) by [casey](https://github.com/casey))
- Hide summary functionality behind feature flag ([#472](https://github.com/casey/just/pull/472) by [casey](https://github.com/casey))
- Fix `summary` feature conditional compilation ([#475](https://github.com/casey/just/pull/475) by [casey](https://github.com/casey))
- Allow integration test cases to omit common values ([#480](https://github.com/casey/just/pull/480) by [casey](https://github.com/casey))
- Add `unindent()` for nicer integration test strings ([#481](https://github.com/casey/just/pull/481) by [casey](https://github.com/casey))
- Start pulling argument parsing out of run::run() ([#483](https://github.com/casey/just/pull/483) by [casey](https://github.com/casey))
- Add explicit `Subcommand` enum ([#484](https://github.com/casey/just/pull/484) by [casey](https://github.com/casey))
- Avoid using error code `1` in integration tests ([#486](https://github.com/casey/just/pull/486) by [casey](https://github.com/casey))
- Use more indented strings in integration tests ([#489](https://github.com/casey/just/pull/489) by [casey](https://github.com/casey))
- Refactor `run::run` and Config ([#490](https://github.com/casey/just/pull/490) by [casey](https://github.com/casey))
- Remove `misc.rs` ([#491](https://github.com/casey/just/pull/491) by [casey](https://github.com/casey))
- Remove unused `use` statements ([#497](https://github.com/casey/just/pull/497) by [casey](https://github.com/casey))
- Refactor lexer tests ([#498](https://github.com/casey/just/pull/498) by [casey](https://github.com/casey))
- Use constants instead of literals in arg parser ([#504](https://github.com/casey/just/pull/504) by [casey](https://github.com/casey))

### Infrastructure
- Add repository attribute to Cargo.toml ([#493](https://github.com/casey/just/pull/493) by [SOF3](https://github.com/SOF3))
- Check minimal version compatibility before publishing ([#487](https://github.com/casey/just/pull/487) by [casey](https://github.com/casey))

### Continuous Integration
- Disable FreeBSD builds ([#474](https://github.com/casey/just/pull/474) by [casey](https://github.com/casey))
- Use `bash` as shell for all integration tests ([#479](https://github.com/casey/just/pull/479) by [casey](https://github.com/casey))
- Don't install `dash` on Travis ([#482](https://github.com/casey/just/pull/482) by [casey](https://github.com/casey))

### Dependencies
- Use `tempfile` crate instead of `tempdir` ([#455](https://github.com/casey/just/pull/455) by [NickeZ](https://github.com/NickeZ))
- Bump clap dependency to 2.33.0 ([#458](https://github.com/casey/just/pull/458) by [NickeZ](https://github.com/NickeZ))
- Minimize dependency version requirements ([#461](https://github.com/casey/just/pull/461) by [casey](https://github.com/casey))
- Remove dependency on brev ([#462](https://github.com/casey/just/pull/462) by [casey](https://github.com/casey))
- Update dependencies ([#501](https://github.com/casey/just/pull/501) by [casey](https://github.com/casey))

[0.4.4](https://github.com/casey/just/releases/tag/v0.4.4) - 2019-06-02
-----------------------------------------------------------------------

### Changed
- Ignore file name case while searching for justfile ([#436](https://github.com/casey/just/pull/436) by [shevtsiv](https://github.com/shevtsiv))

### Added
- Display alias target with `--show` ([#443](https://github.com/casey/just/pull/443) by [casey](https://github.com/casey))

[0.4.3](https://github.com/casey/just/releases/tag/v0.4.3) - 2019-05-07
-----------------------------------------------------------------------

### Changed
- Deprecate `=` in assignments, aliases, and exports in favor of `:=` ([#413](https://github.com/casey/just/pull/413) by [casey](https://github.com/casey))

### Added
- Pass stdin handle to backtick process ([#409](https://github.com/casey/just/pull/409) by [casey](https://github.com/casey))

### Documented
- Fix readme command line ([#411](https://github.com/casey/just/pull/411) by [casey](https://github.com/casey))
- Typo: "command equivelant" -> "command equivalent" ([#418](https://github.com/casey/just/pull/418) by [casey](https://github.com/casey))
- Mention Make’s “phony target” workaround in the comparison ([#421](https://github.com/casey/just/pull/421) by [roryokane](https://github.com/roryokane))
- Add Void Linux install instructions to readme ([#423](https://github.com/casey/just/pull/423) by [casey](https://github.com/casey))

### Cleaned up or Refactored
- Remove stray source files ([#408](https://github.com/casey/just/pull/408) by [casey](https://github.com/casey))
- Replace some calls to brev crate ([#410](https://github.com/casey/just/pull/410) by [casey](https://github.com/casey))
- Lexer code deduplication and refactoring ([#414](https://github.com/casey/just/pull/414) by [casey](https://github.com/casey))
- Refactor and rename test macros ([#415](https://github.com/casey/just/pull/415) by [casey](https://github.com/casey))
- Move CompilationErrorKind into separate module ([#416](https://github.com/casey/just/pull/416) by [casey](https://github.com/casey))
- Remove `write_token_error_context` ([#417](https://github.com/casey/just/pull/417) by [casey](https://github.com/casey))

[0.4.2](https://github.com/casey/just/releases/tag/v0.4.2) - 2019-04-12
-----------------------------------------------------------------------

### Changed
- Regex-based lexer replaced with much nicer character-at-a-time lexer ([#406](https://github.com/casey/just/pull/406) by [casey](https://github.com/casey))

[0.4.1](https://github.com/casey/just/releases/tag/v0.4.1) - 2019-04-12
-----------------------------------------------------------------------

### Changed
- Make summary function non-generic ([#404](https://github.com/casey/just/pull/404) by [casey](https://github.com/casey))

[0.4.0](https://github.com/casey/just/releases/tag/v0.4.0) - 2019-04-12
-----------------------------------------------------------------------

### Added
- Add recipe aliases ([#390](https://github.com/casey/just/pull/390) by [ryloric](https://github.com/ryloric))
- Allow arbitrary expressions as default arguments ([#400](https://github.com/casey/just/pull/400) by [casey](https://github.com/casey))
- Add justfile summaries ([#399](https://github.com/casey/just/pull/399) by [casey](https://github.com/casey))
- Allow outer shebang lines so justfiles can be used as scripts ([#393](https://github.com/casey/just/pull/393) by [casey](https://github.com/casey))
- Allow `--justfile` without `--working-directory` ([#392](https://github.com/casey/just/pull/392) by [smonami](https://github.com/smonami))
- Add link to Chinese translation of readme by chinanf-boy ([#377](https://github.com/casey/just/pull/377) by [casey](https://github.com/casey))

### Changed
- Upgrade to Rust 2018 ([#394](https://github.com/casey/just/pull/394) by [casey](https://github.com/casey))
- Format the codebase with rustfmt ([#346](https://github.com/casey/just/pull/346) by [casey](https://github.com/casey))

[0.3.13](https://github.com/casey/just/releases/tag/v0.3.13) - 2018-11-06
-------------------------------------------------------------------------

### Added
- Print recipe signature if missing arguments ([#369](https://github.com/casey/just/pull/369) by [ladysamantha](https://github.com/ladysamantha))
- Add grandiloquent verbosity level that echos shebang recipes ([#348](https://github.com/casey/just/pull/348) by [casey](https://github.com/casey))
- Wait for child processes to finish ([#345](https://github.com/casey/just/pull/345) by [casey](https://github.com/casey))
- Improve invalid escape sequence error messages ([#328](https://github.com/casey/just/pull/328) by [casey](https://github.com/casey))

### Fixed
- Use PutBackN instead of PutBack in parser ([#364](https://github.com/casey/just/pull/364) by [casey](https://github.com/casey))

[0.3.12](https://github.com/casey/just/releases/tag/v0.3.12) - 2018-06-19
-------------------------------------------------------------------------

### Added
- Implemented invocation_directory function ([#323](https://github.com/casey/just/pull/323) by [casey](https://github.com/casey))

[0.3.11](https://github.com/casey/just/releases/tag/v0.3.11) - 2018-05-6
------------------------------------------------------------------------

### Fixed
- Fixed colors on windows ([#317](https://github.com/casey/just/pull/317) by [casey](https://github.com/casey))

[0.3.10](https://github.com/casey/just/releases/tag/v0.3.10) - 2018-3-19
------------------------------------------------------------------------

### Added
- Make .env vars available in env_var functions ([#310](https://github.com/casey/just/pull/310) by [casey](https://github.com/casey))

[0.3.8](https://github.com/casey/just/releases/tag/v0.3.8) - 2018-3-5
---------------------------------------------------------------------

### Added
- Add dotenv integration ([#306](https://github.com/casey/just/pull/306) by [casey](https://github.com/casey))

[0.3.7](https://github.com/casey/just/releases/tag/v0.3.7) - 2017-12-11
-----------------------------------------------------------------------

### Fixed
- Fix error if ! appears in comment ([#296](https://github.com/casey/just/pull/296) by [casey](https://github.com/casey))

[0.3.6](https://github.com/casey/just/releases/tag/v0.3.6) - 2017-12-11
-----------------------------------------------------------------------

### Fixed
- Lex CRLF line endings properly ([#292](https://github.com/casey/just/pull/292) by [casey](https://github.com/casey))

[0.3.5](https://github.com/casey/just/releases/tag/v0.3.5) - 2017-12-11
-----------------------------------------------------------------------

### Added
- Align doc-comments in `--list` output ([#273](https://github.com/casey/just/pull/273) by [casey](https://github.com/casey))
- Add `arch()`, `os()`, and `os_family()` functions ([#277](https://github.com/casey/just/pull/277) by [casey](https://github.com/casey))
- Add `env_var(key)` and `env_var_or_default(key, default)` functions ([#280](https://github.com/casey/just/pull/280) by [casey](https://github.com/casey))

[0.3.4](https://github.com/casey/just/releases/tag/v0.3.4) - 2017-10-06
-----------------------------------------------------------------------

### Added
- Do not evaluate backticks in assignments during dry runs ([#253](https://github.com/casey/just/pull/253) by [aoeu](https://github.com/aoeu))

### Changed
- Change license to CC0 going forward ([#270](https://github.com/casey/just/pull/270) by [casey](https://github.com/casey))

[0.3.1](https://github.com/casey/just/releases/tag/v0.3.1) - 2017-10-06
-----------------------------------------------------------------------

### Added
- Started keeping a changelog in CHANGELOG.md ([#220](https://github.com/casey/just/pull/220) by [casey](https://github.com/casey))
- Recipes whose names begin with an underscore will not appear in `--list` or `--summary` ([#229](https://github.com/casey/just/pull/229) by [casey](https://github.com/casey))
