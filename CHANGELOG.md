Changelog
=========

[1.25.2](https://github.com/casey/just/releases/tag/1.25.2) - 2024-03-10
------------------------------------------------------------------------

- Unpin ctrlc ([#1951](https://github.com/casey/just/pull/1951))

[1.25.1](https://github.com/casey/just/releases/tag/1.25.1) - 2024-03-09
------------------------------------------------------------------------

### Misc
- Pin ctrlc to version 3.1.1 ([#1945](https://github.com/casey/just/pull/1945))
- Fix AArch64 release build error ([#1942](https://github.com/casey/just/pull/1942))

[1.25.0](https://github.com/casey/just/releases/tag/1.25.0) - 2024-03-07
------------------------------------------------------------------------

### Added
- Add `blake3` and `blake3_file` functions ([#1860](https://github.com/casey/just/pull/1860) by [tgross35](https://github.com/tgross35))

### Misc
- Fix readme typo ([#1936](https://github.com/casey/just/pull/1936) by [Justintime50](https://github.com/Justintime50))
- Use unwrap_or_default ([#1928](https://github.com/casey/just/pull/1928))
- Set codegen-units to 1 reduce release binary size ([#1920](https://github.com/casey/just/pull/1920) by [amarao](https://github.com/amarao))
- Document openSUSE package ([#1918](https://github.com/casey/just/pull/1918) by [sfalken](https://github.com/sfalken))
- Fix install.sh shellcheck warnings ([#1912](https://github.com/casey/just/pull/1912) by [tgross35](https://github.com/tgross35))

[1.24.0](https://github.com/casey/just/releases/tag/1.24.0) - 2024-02-11
------------------------------------------------------------------------

### Added
- Support recipe paths containing `::` in Bash completion script ([#1863](https://github.com/casey/just/pull/1863) by [crdx](https://github.com/crdx))
- Add function to canonicalize paths ([#1859](https://github.com/casey/just/pull/1859))

### Misc
- Document installing just on Github Actions in readme ([#1867](https://github.com/casey/just/pull/1867) by [cclauss](https://github.com/cclauss))
- Use unlikely-to-be-set variable name in env tests ([#1882](https://github.com/casey/just/pull/1882))
- Skip write_error test if running as root ([#1881](https://github.com/casey/just/pull/1881))
- Convert run_shebang into integration test ([#1880](https://github.com/casey/just/pull/1880))
- Install mdbook with cargo in CI workflow ([#1877](https://github.com/casey/just/pull/1877))
- Remove deprecated actions-rs/toolchain ([#1874](https://github.com/casey/just/pull/1874) by [cclauss](https://github.com/cclauss))
- Fix Gentoo package link ([#1875](https://github.com/casey/just/pull/1875) by [vozbu](https://github.com/vozbu))
- Fix typos found by codespell ([#1872](https://github.com/casey/just/pull/1872) by [cclauss](https://github.com/cclauss))
- Replace deprecated set-output command in Github Actions workflows ([#1869](https://github.com/casey/just/pull/1869) by [cclauss](https://github.com/cclauss))
- Update `actions/checkout` and `softprops/action-gh-release` ([#1871](https://github.com/casey/just/pull/1871) by [app/dependabot](https://github.com/app/dependabot))
- Keep GitHub Actions up to date with Dependabot ([#1868](https://github.com/casey/just/pull/1868) by [cclauss](https://github.com/cclauss))
- Add contrib directory ([#1870](https://github.com/casey/just/pull/1870))
- Fix install script ([#1844](https://github.com/casey/just/pull/1844))

[1.23.0](https://github.com/casey/just/releases/tag/1.23.0) - 2024-01-12
------------------------------------------------------------------------

### Added
- Allow setting custom confirm prompt ([#1834](https://github.com/casey/just/pull/1834) by [CramBL](https://github.com/CramBL))
- Add `set quiet` and `[no-quiet]` ([#1704](https://github.com/casey/just/pull/1704) by [dharrigan](https://github.com/dharrigan))
- Add `just_pid` function ([#1833](https://github.com/casey/just/pull/1833) by [Swordelf2](https://github.com/Swordelf2))
- Add functions to return XDG base directories ([#1822](https://github.com/casey/just/pull/1822) by [tgross35](https://github.com/tgross35))
- Add `--no-deps` to skip running recipe dependencies ([#1819](https://github.com/casey/just/pull/1819) by [ngharrington](https://github.com/ngharrington))

### Fixed
- Run imports in working directory of importer ([#1817](https://github.com/casey/just/pull/1817))

### Misc
- Include completion scripts in releases ([#1837](https://github.com/casey/just/pull/1837))
- Tweak readme table formatting ([#1836](https://github.com/casey/just/pull/1836))
- Don't abbreviate just in README ([#1831](https://github.com/casey/just/pull/1831) by [thled](https://github.com/thled))
- Ignore [private] recipes in just --list ([#1816](https://github.com/casey/just/pull/1816) by [crdx](https://github.com/crdx))
- Add a dash to tempdir prefix ([#1828](https://github.com/casey/just/pull/1828))

[1.22.1](https://github.com/casey/just/releases/tag/1.22.1) - 2024-01-08
------------------------------------------------------------------------

### Fixed
- Don't conflate recipes with the same name in different modules ([#1825](https://github.com/casey/just/pull/1825))

### Misc
- Clarify that UUID is version 4 ([#1821](https://github.com/casey/just/pull/1821) by [tgross35](https://github.com/tgross35))
- Make sigil stripping from recipe lines less incomprehensible ([#1812](https://github.com/casey/just/pull/1812))
- Refactor invalid path argument check ([#1811](https://github.com/casey/just/pull/1811))

[1.22.0](https://github.com/casey/just/releases/tag/1.22.0) - 2023-12-31
------------------------------------------------------------------------

### Added
- Recipes can be invoked with path syntax ([#1809](https://github.com/casey/just/pull/1809))
- Add `--format` and `--initialize` as aliases for `--fmt` and `--init` ([#1802](https://github.com/casey/just/pull/1802))

### Misc
- Move table of contents pointer to right ([#1806](https://github.com/casey/just/pull/1806))

[1.21.0](https://github.com/casey/just/releases/tag/1.21.0) - 2023-12-29
------------------------------------------------------------------------

### Added
- Optional modules and imports ([#1797](https://github.com/casey/just/pull/1797))
- Print submodule recipes in --summary ([#1794](https://github.com/casey/just/pull/1794))

### Misc
- Use box-drawing characters in error messages ([#1798](https://github.com/casey/just/pull/1798))
- Use Self ([#1795](https://github.com/casey/just/pull/1795))

[1.20.0](https://github.com/casey/just/releases/tag/1.20.0) - 2023-12-28
------------------------------------------------------------------------

### Added
- Allow mod statements with path to source file ([#1786](https://github.com/casey/just/pull/1786))

### Changed
- Expand tilde in import and module paths ([#1792](https://github.com/casey/just/pull/1792))
- Override imported recipes ([#1790](https://github.com/casey/just/pull/1790))
- Run recipes with working directory set to submodule directory ([#1788](https://github.com/casey/just/pull/1788))

### Misc
- Document import override behavior ([#1791](https://github.com/casey/just/pull/1791))
- Document submodule working directory ([#1789](https://github.com/casey/just/pull/1789))

[1.19.0](https://github.com/casey/just/releases/tag/1.19.0) - 2023-12-27
------------------------------------------------------------------------

### Added
- Add modules ([#1782](https://github.com/casey/just/pull/1782))

[1.18.1](https://github.com/casey/just/releases/tag/1.18.1) - 2023-12-24
------------------------------------------------------------------------

### Added
- Display a descriptive error for `!include` directives ([#1779](https://github.com/casey/just/pull/1779))

[1.18.0](https://github.com/casey/just/releases/tag/1.18.0) - 2023-12-24
------------------------------------------------------------------------

### Added
- Stabilize `!include path` as `import 'path'` ([#1771](https://github.com/casey/just/pull/1771))

### Misc
- Tweak readme ([#1775](https://github.com/casey/just/pull/1775))

[1.17.0](https://github.com/casey/just/releases/tag/1.17.0) - 2023-12-20
------------------------------------------------------------------------

### Added
- Add `[confirm]` attribute ([#1723](https://github.com/casey/just/pull/1723) by [Hwatwasthat](https://github.com/Hwatwasthat))

### Changed
- Don't default to included recipes ([#1740](https://github.com/casey/just/pull/1740))

### Fixed
- Pass justfile path to default chooser ([#1759](https://github.com/casey/just/pull/1759) by [Qeole](https://github.com/Qeole))
- Pass `--unstable` and `--color always` to default chooser ([#1758](https://github.com/casey/just/pull/1758) by [Qeole](https://github.com/Qeole))

### Misc
- Update Gentoo package repository ([#1757](https://github.com/casey/just/pull/1757) by [paul-jewell](https://github.com/paul-jewell))
- Fix readme header level ([#1752](https://github.com/casey/just/pull/1752) by [laniakea64](https://github.com/laniakea64))
- Document line continuations ([#1751](https://github.com/casey/just/pull/1751) by [laniakea64](https://github.com/laniakea64))
- List included recipes in load order ([#1745](https://github.com/casey/just/pull/1745))
- Fix build badge in zh readme ([#1743](https://github.com/casey/just/pull/1743) by [chenrui333](https://github.com/chenrui333))
- Rename Justfile::first → Justfile::default ([#1741](https://github.com/casey/just/pull/1741))
- Add file paths to error messages ([#1737](https://github.com/casey/just/pull/1737))
- Move !include processing into compiler ([#1618](https://github.com/casey/just/pull/1618) by [neunenak](https://github.com/neunenak))
- Update Arch Linux package URL in readme ([#1733](https://github.com/casey/just/pull/1733) by [felixonmars](https://github.com/felixonmars))
- Clarify that aliases can only be used on the command line ([#1726](https://github.com/casey/just/pull/1726) by [laniakea64](https://github.com/laniakea64))
- Remove VALID_ALIAS_ATTRIBUTES array ([#1731](https://github.com/casey/just/pull/1731))
- Fix justfile search link in Chinese docs ([#1730](https://github.com/casey/just/pull/1730) by [oluceps](https://github.com/oluceps))
- Add example of Windows shebang handling ([#1709](https://github.com/casey/just/pull/1709) by [pfmoore](https://github.com/pfmoore))
- Fix CI ([#1728](https://github.com/casey/just/pull/1728))

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
- Fix readme build badge ([#1697](https://github.com/casey/just/pull/1697))
- Fix set tempdir grammar ([#1695](https://github.com/casey/just/pull/1695))
- Add version to attributes ([#1694](https://github.com/casey/just/pull/1694) by [JoeyTeng](https://github.com/JoeyTeng))
- Update README.md ([#1691](https://github.com/casey/just/pull/1691) by [laniakea64](https://github.com/laniakea64))


[1.15.0](https://github.com/casey/just/releases/tag/1.15.0) - 2023-10-09
------------------------------------------------------------------------

### Added
- Add Nushell completion script ([#1571](https://github.com/casey/just/pull/1571) by [presidento](https://github.com/presidento))
- Allow unstable features to be enabled with environment variable ([#1588](https://github.com/casey/just/pull/1588) by [neunenak](https://github.com/neunenak))
- Add num_cpus() function ([#1568](https://github.com/casey/just/pull/1568) by [schultetwin1](https://github.com/schultetwin1))
- Allow escaping newlines ([#1551](https://github.com/casey/just/pull/1551) by [ids1024](https://github.com/ids1024))
- Stabilize JSON dump format ([#1633](https://github.com/casey/just/pull/1633))
- Add env() function ([#1613](https://github.com/casey/just/pull/1613) by [kykyi](https://github.com/kykyi))

### Changed
- Allow selecting multiple recipes with default chooser ([#1547](https://github.com/casey/just/pull/1547) by [fzdwx](https://github.com/fzdwx))

### Misc
- Don't recommend `vim-polyglot` in readme ([#1644](https://github.com/casey/just/pull/1644) by [laniakea64](https://github.com/laniakea64))
- Note Micro support in readme ([#1316](https://github.com/casey/just/pull/1316) by [tomodachi94](https://github.com/tomodachi94))
- Update Indentation Documentation ([#1600](https://github.com/casey/just/pull/1600) by [GinoMan](https://github.com/GinoMan))
- Fix triple-quoted string example in readme ([#1620](https://github.com/casey/just/pull/1620) by [avi-cenna](https://github.com/avi-cenna))
- README fix: the -d in `mktemp -d` is required to created folders. ([#1688](https://github.com/casey/just/pull/1688) by [gl-yziquel](https://github.com/gl-yziquel))
- Placate clippy ([#1689](https://github.com/casey/just/pull/1689))
- Fix README typos ([#1660](https://github.com/casey/just/pull/1660) by [akuhnregnier](https://github.com/akuhnregnier))
- Document Windows Package Manager install instructions ([#1656](https://github.com/casey/just/pull/1656))
- Test unpaired escaped carriage return error ([#1650](https://github.com/casey/just/pull/1650))
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
- Ignore clippy::let_underscore_untyped ([#1609](https://github.com/casey/just/pull/1609))
- Link to private recipes section in readme ([#1542](https://github.com/casey/just/pull/1542) by [quad](https://github.com/quad))
- Update README to reflect new attribute syntax ([#1538](https://github.com/casey/just/pull/1538) by [neunenak](https://github.com/neunenak))
- Allow multiple attributes on one line ([#1537](https://github.com/casey/just/pull/1537) by [neunenak](https://github.com/neunenak))
- Analyze and Compiler tweaks ([#1534](https://github.com/casey/just/pull/1534) by [neunenak](https://github.com/neunenak))
- Downgrade to TLS 1.2 in install script ([#1536](https://github.com/casey/just/pull/1536))

[1.13.0](https://github.com/casey/just/releases/tag/1.13.0) - 2023-01-24
------------------------------------------------------------------------

### Added
- Add -n as a short flag for --for dry-run ([#1524](https://github.com/casey/just/pull/1524) by [maiha](https://github.com/maiha))
- Add invocation_directory_native() ([#1507](https://github.com/casey/just/pull/1507))

### Changed
- Ignore additional search path arguments ([#1528](https://github.com/casey/just/pull/1528) by [neunenak](https://github.com/neunenak))
- Only print fallback message when verbose ([#1510](https://github.com/casey/just/pull/1510))
- Print format diff to stdout ([#1506](https://github.com/casey/just/pull/1506))

### Fixed
- Test passing dot as argument between justfiles ([#1530](https://github.com/casey/just/pull/1530))
- Fix install script default directory ([#1525](https://github.com/casey/just/pull/1525))

### Misc
- Note that justfiles are order-insensitive ([#1529](https://github.com/casey/just/pull/1529))
- Borrow Ast in Analyser ([#1527](https://github.com/casey/just/pull/1527) by [neunenak](https://github.com/neunenak))
- Ignore chooser tests ([#1513](https://github.com/casey/just/pull/1513))
- Put default setting values in backticks ([#1512](https://github.com/casey/just/pull/1512) by [s1ck](https://github.com/s1ck))
- Use lowercase boolean literals in readme ([#1511](https://github.com/casey/just/pull/1511) by [s1ck](https://github.com/s1ck))
- Document invocation_directory_native() ([#1508](https://github.com/casey/just/pull/1508))
- Fix interrupt tests ([#1505](https://github.com/casey/just/pull/1505))

[1.12.0](https://github.com/casey/just/releases/tag/1.12.0) - 2023-01-12
------------------------------------------------------------------------

### Added
- Add `!include` directives ([#1470](https://github.com/casey/just/pull/1470) by [neunenak](https://github.com/neunenak))

### Changed
- Allow matching search path arguments ([#1475](https://github.com/casey/just/pull/1475) by [neunenak](https://github.com/neunenak))
- Allow recipe parameters to shadow variables ([#1480](https://github.com/casey/just/pull/1480))

### Misc
- Remove --unstable from fallback example in readme ([#1502](https://github.com/casey/just/pull/1502))
- Specify minimum rust version ([#1496](https://github.com/casey/just/pull/1496) by [benmoss](https://github.com/benmoss))
- Note that install.sh may fail on GitHub actions ([#1499](https://github.com/casey/just/pull/1499))
- Fix readme typo ([#1489](https://github.com/casey/just/pull/1489) by [auberisky](https://github.com/auberisky))
- Update install script and readmes to use tls v1.3 ([#1481](https://github.com/casey/just/pull/1481))
- Re-enable install.sh test on CI([#1478](https://github.com/casey/just/pull/1478))
- Don't test install.sh on CI ([#1477](https://github.com/casey/just/pull/1477))
- Update Chinese translation of readme ([#1476](https://github.com/casey/just/pull/1476) by [hustcer](https://github.com/hustcer))
- Fix install.sh for Windows ([#1474](https://github.com/casey/just/pull/1474) by [bloodearnest](https://github.com/bloodearnest))

[1.11.0](https://github.com/casey/just/releases/tag/1.11.0) - 2023-01-03
------------------------------------------------------------------------

### Added
- Stabilize fallback ([#1471](https://github.com/casey/just/pull/1471))

### Misc
- Update Sublime syntax instructions ([#1455](https://github.com/casey/just/pull/1455) by [nk9](https://github.com/nk9))

[1.10.0](https://github.com/casey/just/releases/tag/1.10.0) - 2023-01-01
------------------------------------------------------------------------

### Added
- Allow private attribute on aliases ([#1434](https://github.com/casey/just/pull/1434) by [neunenak](https://github.com/neunenak))

### Changed
- Suppress --fmt --check diff if --quiet is passed ([#1457](https://github.com/casey/just/pull/1457))

### Fixed
- Format exported variadic parameters correctly ([#1451](https://github.com/casey/just/pull/1451))

### Misc
- Fix section title grammar ([#1466](https://github.com/casey/just/pull/1466) by [brettcannon](https://github.com/brettcannon))
- Give pages job write permissions([#1464](https://github.com/casey/just/pull/1464) by [jsoref](https://github.com/jsoref))
- Fix spelling ([#1463](https://github.com/casey/just/pull/1463) by [jsoref](https://github.com/jsoref))
- Merge imports ([#1462](https://github.com/casey/just/pull/1462))
- Add instructions for taiki-e/install-action ([#1459](https://github.com/casey/just/pull/1459) by [azzamsa](https://github.com/azzamsa))
- Differentiate between shell and nushell example ([#1427](https://github.com/casey/just/pull/1427) by [Dialga](https://github.com/Dialga))
- Link regex docs in readme ([#1454](https://github.com/casey/just/pull/1454))
- Linkify changelog PRs and usernames ([#1440](https://github.com/casey/just/pull/1440) by [nk9](https://github.com/nk9))
- Eliminate lazy_static ([#1442](https://github.com/casey/just/pull/1442) by [camsteffen](https://github.com/camsteffen))
- Add attributes to sublime syntax file ([#1452](https://github.com/casey/just/pull/1452) by [crdx](https://github.com/crdx))
- Fix homepage style ([#1453](https://github.com/casey/just/pull/1453))
- Linkify homepage letters ([#1448](https://github.com/casey/just/pull/1448) by [nk9](https://github.com/nk9))
- Use `just` in readme codeblocks ([#1447](https://github.com/casey/just/pull/1447) by [nicochatzi](https://github.com/nicochatzi))
- Update MSRV in readme ([#1446](https://github.com/casey/just/pull/1446))
- Merge CI workflows ([#1444](https://github.com/casey/just/pull/1444))
- Use dotenvy instead of dotenv ([#1443](https://github.com/casey/just/pull/1443) by [mike-burns](https://github.com/mike-burns))
- Update Chinese translation of readme ([#1428](https://github.com/casey/just/pull/1428) by [hustcer](https://github.com/hustcer))

[1.9.0](https://github.com/casey/just/releases/tag/1.9.0) - 2022-11-25
----------------------------------------------------------------------

### Breaking Changes to Unstable Features
- Change `fallback` setting default to false ([#1425](https://github.com/casey/just/pull/1425))

### Added
- Hide recipes with `[private]` attribute ([#1422](https://github.com/casey/just/pull/1422))
- Add replace_regex function ([#1393](https://github.com/casey/just/pull/1393) by [miles170](https://github.com/miles170))
- Add [no-cd] attribute ([#1400](https://github.com/casey/just/pull/1400))

### Changed
- Omit shebang lines on Windows ([#1417](https://github.com/casey/just/pull/1417))

### Misc
- Placate clippy ([#1423](https://github.com/casey/just/pull/1423))
- Make include_shebang_line clearer ([#1418](https://github.com/casey/just/pull/1418))
- Use more secure cURL options in install.sh ([#1416](https://github.com/casey/just/pull/1416))
- Document how shebang recipes are executed ([#1412](https://github.com/casey/just/pull/1412))
- Fix typo: regec → regex ([#1409](https://github.com/casey/just/pull/1409))
- Use powershell.exe instead of pwsh.exe in readme ([#1394](https://github.com/casey/just/pull/1394) by [asdf8dfafjk](https://github.com/asdf8dfafjk))
- Expand alternatives and prior art in readme ([#1401](https://github.com/casey/just/pull/1401))
- Split up CI workflow ([#1399](https://github.com/casey/just/pull/1399))

[1.8.0](https://github.com/casey/just/releases/tag/1.8.0) - 2022-11-02
----------------------------------------------------------------------

### Added
- Add OS Configuration Attributes ([#1387](https://github.com/casey/just/pull/1387))

### Misc
- Link to sclu1034/vscode-just in readme ([#1396](https://github.com/casey/just/pull/1396))

[1.7.0](https://github.com/casey/just/releases/tag/1.7.0) - 2022-10-26
----------------------------------------------------------------------

### Breaking Changes to Unstable Features
- Make `fallback` setting default to true ([#1384](https://github.com/casey/just/pull/1384))

### Added
- Add more case-conversion functions ([#1383](https://github.com/casey/just/pull/1383) by [gVirtu](https://github.com/gVirtu))
- Add `tempdir` setting ([#1369](https://github.com/casey/just/pull/1369) by [dmatos2012](https://github.com/dmatos2012))
- Add [no-exit-message] recipe annotation ([#1354](https://github.com/casey/just/pull/1354) by [gokhanettin](https://github.com/gokhanettin))
- Add `capitalize(s)` function ([#1375](https://github.com/casey/just/pull/1375) by [femnad](https://github.com/femnad))

### Misc
- Credit contributors in changelog ([#1385](https://github.com/casey/just/pull/1385))
- Update asdf just plugin repository ([#1380](https://github.com/casey/just/pull/1380) by [kachick](https://github.com/kachick))
- Prepend commit messages with `- ` in changelog ([#1379](https://github.com/casey/just/pull/1379))
- Fail publish if `<sup>master</sup>` is found in README.md ([#1378](https://github.com/casey/just/pull/1378))
- Use for loop in capitalize implementation ([#1377](https://github.com/casey/just/pull/1377))

[1.6.0](https://github.com/casey/just/releases/tag/1.6.0) - 2022-10-19
----------------------------------------------------------------------

### Breaking Changes to Unstable Features
- Require `set fallback := true` to enable recipe fallback ([#1368](https://github.com/casey/just/pull/1368))

### Changed
- Allow fallback with search directory ([#1348](https://github.com/casey/just/pull/1348))

### Added
- Don't evaluate comments ([#1358](https://github.com/casey/just/pull/1358))
- Add skip-comments setting ([#1333](https://github.com/casey/just/pull/1333) by [neunenak](https://github.com/neunenak))
- Allow bash completion to complete tasks in other directories ([#1303](https://github.com/casey/just/pull/1303) by [jpbochi](https://github.com/jpbochi))

### Misc
- Restore www/CNAME ([#1364](https://github.com/casey/just/pull/1364))
- Improve book config ([#1363](https://github.com/casey/just/pull/1363))
- Add kitchen sink justfile to test syntax highlighting ([#1362](https://github.com/casey/just/pull/1362) by [nk9](https://github.com/nk9))
- Note version in which absolute path construction was added ([#1361](https://github.com/casey/just/pull/1361))
- Inline setup and cleanup functions in completion script test ([#1352](https://github.com/casey/just/pull/1352))

[1.5.0](https://github.com/casey/just/releases/tag/1.5.0) - 2022-9-11
---------------------------------------------------------------------

### Changed
- Allow constructing absolute paths with `/` operator ([#1320](https://github.com/casey/just/pull/1320) by [erikkrieg](https://github.com/erikkrieg))

### Misc
- Allow fewer lints ([#1340](https://github.com/casey/just/pull/1340))
- Fix issues reported by nightly clippy ([#1336](https://github.com/casey/just/pull/1336) by [neunenak](https://github.com/neunenak))
- Refactor run.rs ([#1335](https://github.com/casey/just/pull/1335) by [neunenak](https://github.com/neunenak))
- Allow comments on same line as settings ([#1339](https://github.com/casey/just/pull/1339))
- Fix justfile env shebang on Linux ([#1330](https://github.com/casey/just/pull/1330))
- Update Chinese translation of README.md ([#1325](https://github.com/casey/just/pull/1325) by [hustcer](https://github.com/hustcer))
- Add additional settings to grammar
- Add an example of using a variable in a recipe parameter ([#1311](https://github.com/casey/just/pull/1311) by [papertigers](https://github.com/papertigers))

[1.4.0](https://github.com/casey/just/releases/tag/1.4.0) - 2022-8-08
---------------------------------------------------------------------

### Fixed
- Fix shell setting precedence ([#1306](https://github.com/casey/just/pull/1306))

### Misc
- Don't hardcode homebrew prefix ([#1295](https://github.com/casey/just/pull/1295))
- Exclude files from cargo package ([#1283](https://github.com/casey/just/pull/1283))
- Add usage note to default list recipe ([#1296](https://github.com/casey/just/pull/1296) by [jpbochi](https://github.com/jpbochi))
- Add MPR/Prebuilt-MPR installation instructions to README.md ([#1280](https://github.com/casey/just/pull/1280) by [hwittenborn](https://github.com/hwittenborn))
- Add make and makesure to readme ([#1299](https://github.com/casey/just/pull/1299))
- Document how to configure zsh completions on MacOS ([#1285](https://github.com/casey/just/pull/1285) by [nk9](https://github.com/nk9))
- Convert package table to HTML ([#1291](https://github.com/casey/just/pull/1291))

[1.3.0](https://github.com/casey/just/releases/tag/1.3.0) - 2022-7-25
---------------------------------------------------------------------

### Added
- Add `/` operator ([#1237](https://github.com/casey/just/pull/1237))

### Fixed
- Fix multibyte codepoint crash ([#1243](https://github.com/casey/just/pull/1243))

### Misc
- Update just-install reference on README.md ([#1275](https://github.com/casey/just/pull/1275) by [0xradical](https://github.com/0xradical))
- Split Recipe::run into Recipe::{run_shebang,run_linewise} ([#1270](https://github.com/casey/just/pull/1270))
- Add asdf package to readme([#1264](https://github.com/casey/just/pull/1264) by [jaacko-torus](https://github.com/jaacko-torus))
- Add mdbook deps for build-book recipe ([#1259](https://github.com/casey/just/pull/1259) by [TopherIsSwell](https://github.com/TopherIsSwell))
- Fix typo: argumant -> argument ([#1257](https://github.com/casey/just/pull/1257) by [kianmeng](https://github.com/kianmeng))
- Improve error message if `if` is missing the `else` ([#1252](https://github.com/casey/just/pull/1252) by [nk9](https://github.com/nk9))
- Explain how to pass arguments of a command to a dependency ([#1254](https://github.com/casey/just/pull/1254) by [heavelock](https://github.com/heavelock))
- Update Chinese translation of README.md ([#1253](https://github.com/casey/just/pull/1253) by [hustcer](https://github.com/hustcer))
- Improvements to Sublime syntax file ([#1250](https://github.com/casey/just/pull/1250) by [nk9](https://github.com/nk9))
- Prevent unbounded recursion when parsing expressions ([#1248](https://github.com/casey/just/pull/1248) by [evanrichter](https://github.com/evanrichter))
- Publish to snap store ([#1245](https://github.com/casey/just/pull/1245))
- Restore fuzz test harness ([#1246](https://github.com/casey/just/pull/1246) by [evanrichter](https://github.com/evanrichter))
- Add just-install to README file ([#1241](https://github.com/casey/just/pull/1241) by [brombal](https://github.com/brombal))
- Fix dead readme link ([#1240](https://github.com/casey/just/pull/1240) by [wdroz](https://github.com/wdroz))
- Do `use super::*;` instead of `use crate::common::*;` ([#1239](https://github.com/casey/just/pull/1239))
- Fix readme punctuation ([#1235](https://github.com/casey/just/pull/1235))
- Add argument splitting section to readme ([#1230](https://github.com/casey/just/pull/1230))
- Add notes about environment variables to readme ([#1229](https://github.com/casey/just/pull/1229))
- Fix book links ([#1227](https://github.com/casey/just/pull/1227))
- Add nushell README.md ([#1224](https://github.com/casey/just/pull/1224) by [hustcer](https://github.com/hustcer))
- Use absolute links in readme ([#1223](https://github.com/casey/just/pull/1223))
- Copy changelog into manual ([#1222](https://github.com/casey/just/pull/1222))
- Translate Chinese manual introduction and title ([#1220](https://github.com/casey/just/pull/1220) by [hustcer](https://github.com/hustcer))
- Build Chinese language user manual ([#1219](https://github.com/casey/just/pull/1219))
- Update Chinese translation of README.md ([#1218](https://github.com/casey/just/pull/1218) by [hustcer](https://github.com/hustcer))
- Translate all of README.md into Chinese ([#1217](https://github.com/casey/just/pull/1217) by [hustcer](https://github.com/hustcer))
- Translate all of features in README into Chinese ([#1215](https://github.com/casey/just/pull/1215) by [hustcer](https://github.com/hustcer))
- Make link to examples directory absolute ([#1213](https://github.com/casey/just/pull/1213))
- Translate part of features in README into Chinese ([#1211](https://github.com/casey/just/pull/1211) by [hustcer](https://github.com/hustcer))
- Add JetBrains IDE plugin to readme ([#1209](https://github.com/casey/just/pull/1209) by [linux-china](https://github.com/linux-china))
- Translate features chapter of readme to Chinese ([#1208](https://github.com/casey/just/pull/1208) by [hustcer](https://github.com/hustcer))

[1.2.0](https://github.com/casey/just/releases/tag/1.2.0) - 2022-5-31
---------------------------------------------------------------------

### Added
- Add `windows-shell` setting ([#1198](https://github.com/casey/just/pull/1198))
- SHA-256 and UUID functions ([#1170](https://github.com/casey/just/pull/1170) by [mbodmer](https://github.com/mbodmer))

### Misc
- Translate editor support and quick start to Chinese ([#1206](https://github.com/casey/just/pull/1206) by [hustcer](https://github.com/hustcer))
- Translate first section of readme into Chinese ([#1205](https://github.com/casey/just/pull/1205) by [hustcer](https://github.com/hustcer))
- Fix a bunch of typos ([#1204](https://github.com/casey/just/pull/1204))
- Remove cargo-limit usage from justfile ([#1199](https://github.com/casey/just/pull/1199))
- Add nix package manager install instructions ([#1194](https://github.com/casey/just/pull/1194) by [risingBirdSong](https://github.com/risingBirdSong))
- Fix broken link in readme ([#1183](https://github.com/casey/just/pull/1183) by [Vlad-Shcherbina](https://github.com/Vlad-Shcherbina))
- Add screenshot to manual ([#1181](https://github.com/casey/just/pull/1181))
- Style homepage ([#1180](https://github.com/casey/just/pull/1180))
- Center readme ([#1178](https://github.com/casey/just/pull/1178))
- Style and add links to homepage ([#1177](https://github.com/casey/just/pull/1177))
- Fix readme badge links ([#1176](https://github.com/casey/just/pull/1176))
- Generate book from readme ([#1155](https://github.com/casey/just/pull/1155))

[1.1.3](https://github.com/casey/just/releases/tag/1.1.3) - 2022-5-3
--------------------------------------------------------------------

### Fixed
- Skip duplicate recipe arguments ([#1174](https://github.com/casey/just/pull/1174))

### Misc
- Fix install script ([#1172](https://github.com/casey/just/pull/1172))
- Document that `invocation_directory()` returns an absolute path ([#1162](https://github.com/casey/just/pull/1162))
- Fix absolute_path documentation ([#1160](https://github.com/casey/just/pull/1160))
- Add cross-platform justfile example ([#1152](https://github.com/casey/just/pull/1152) by [presidento](https://github.com/presidento))

[1.1.2](https://github.com/casey/just/releases/tag/1.1.2) - 2022-3-30
---------------------------------------------------------------------

### Misc
- Document indentation rules ([#1142](https://github.com/casey/just/pull/1142))
- Remove stale link from readme ([#1141](https://github.com/casey/just/pull/1141))

### Unstable
- Search for missing recipes in parent directory justfiles ([#1149](https://github.com/casey/just/pull/1149))

[1.1.1](https://github.com/casey/just/releases/tag/1.1.1) - 2022-3-22
---------------------------------------------------------------------

### Misc
- Build MacOS ARM release binaries ([#1138](https://github.com/casey/just/pull/1138))
- Upgrade Windows Actions runners to windows-latest ([#1137](https://github.com/casey/just/pull/1137))

[1.1.0](https://github.com/casey/just/releases/tag/1.1.0) - 2022-3-10
---------------------------------------------------------------------

### Added
- Add `error()` function ([#1118](https://github.com/casey/just/pull/1118) by [chamons](https://github.com/chamons))
- Add `absolute_path` function ([#1121](https://github.com/casey/just/pull/1121) by [Laura7089](https://github.com/Laura7089))

[1.0.1](https://github.com/casey/just/releases/tag/1.0.1) - 2022-2-28
---------------------------------------------------------------------

### Fixed
- Make path_exists() relative to current directory ([#1122](https://github.com/casey/just/pull/1122))

### Misc
- Detail environment variable usage in readme ([#1086](https://github.com/casey/just/pull/1086) by [kenden](https://github.com/kenden))
- Format --init justfile ([#1116](https://github.com/casey/just/pull/1116) by [TheLocehiliosan](https://github.com/TheLocehiliosan))
- Add hint for Node.js script compatibility ([#1113](https://github.com/casey/just/pull/1113))

[1.0.0](https://github.com/casey/just/releases/tag/1.0.0) - 2022-2-22
---------------------------------------------------------------------

### Added
- Add path_exists() function ([#1106](https://github.com/casey/just/pull/1106) by [heavelock](https://github.com/heavelock))

### Misc
- Note that `pipefail` isn't normally set ([#1108](https://github.com/casey/just/pull/1108))

[0.11.2](https://github.com/casey/just/releases/tag/0.11.2) - 2022-2-15
-----------------------------------------------------------------------

### Misc
- Fix dotenv-load documentation ([#1104](https://github.com/casey/just/pull/1104))
- Fixup broken release package script ([#1100](https://github.com/casey/just/pull/1100) by [lutostag](https://github.com/lutostag))

[0.11.1](https://github.com/casey/just/releases/tag/0.11.1) - 2022-2-14
-----------------------------------------------------------------------

### Added
- Allow duplicate recipes ([#1095](https://github.com/casey/just/pull/1095) by [lutostag](https://github.com/lutostag))

### Misc
- Add arrow pointing to table of contents button ([#1096](https://github.com/casey/just/pull/1096))
- Improve readme ([#1093](https://github.com/casey/just/pull/1093) by [halostatue](https://github.com/halostatue))
- Remove asciidoc readme ([#1092](https://github.com/casey/just/pull/1092))
- Convert README.adoc to markdown ([#1091](https://github.com/casey/just/pull/1091))
- Add choco package to README ([#1090](https://github.com/casey/just/pull/1090) by [michidk](https://github.com/michidk))

[0.11.0](https://github.com/casey/just/releases/tag/0.11.0) - 2022-2-3
----------------------------------------------------------------------

### Breaking
- Change dotenv-load default to false ([#1082](https://github.com/casey/just/pull/1082))

[0.10.7](https://github.com/casey/just/releases/tag/0.10.7) - 2022-1-30
-----------------------------------------------------------------------

### Misc
- Don't run tests in release workflow ([#1080](https://github.com/casey/just/pull/1080))
- Fix windows chooser invocation error message test ([#1079](https://github.com/casey/just/pull/1079))
- Remove call to sed in justfile ([#1078](https://github.com/casey/just/pull/1078))

[0.10.6](https://github.com/casey/just/releases/tag/0.10.6) - 2022-1-29
-----------------------------------------------------------------------

### Added
- Add windows-powershell setting ([#1057](https://github.com/casey/just/pull/1057) by [michidk](https://github.com/michidk))

### Changed
- Allow using `-` and `@` in any order ([#1063](https://github.com/casey/just/pull/1063))

### Misc
- Use `Context` suffix for snafu error contexts ([#1068](https://github.com/casey/just/pull/1068))
- Upgrade snafu to 0.7 ([#1067](https://github.com/casey/just/pull/1067) by [shepmaster](https://github.com/shepmaster))
- Mention "$@" in the README ([#1064](https://github.com/casey/just/pull/1064) by [mpdude](https://github.com/mpdude))
- Note how to use PowerShell with CLI in readme ([#1056](https://github.com/casey/just/pull/1056) by [michidk](https://github.com/michidk))
- Link to cheatsheet from readme ([#1053](https://github.com/casey/just/pull/1053))
- Link to Homebrew installation docs in readme ([#1049](https://github.com/casey/just/pull/1049) by [michidk](https://github.com/michidk))
- Workflow tweaks ([#1045](https://github.com/casey/just/pull/1045))
- Push to correct origin in publish recipe ([#1044](https://github.com/casey/just/pull/1044))

[0.10.5](https://github.com/casey/just/releases/tag/0.10.5) - 2021-12-4
-----------------------------------------------------------------------

### Changed
- Use musl libc for ARM binaries ([#1037](https://github.com/casey/just/pull/1037))

### Misc
- Make completions work with Bash alias ([#1035](https://github.com/casey/just/pull/1035) by [kurtbuilds](https://github.com/kurtbuilds))
- Run tests on PRs ([#1040](https://github.com/casey/just/pull/1040))
- Improve GitHub Actions workflow triggers ([#1033](https://github.com/casey/just/pull/1033))
- Publish from GitHub master branch instead of local master ([#1032](https://github.com/casey/just/pull/1032))

[0.10.4](https://github.com/casey/just/releases/tag/0.10.4) - 2021-11-21
------------------------------------------------------------------------

### Added
- Add `--dump-format json` ([#992](https://github.com/casey/just/pull/992))
- Add `quote(s)` function for escaping strings ([#1022](https://github.com/casey/just/pull/1022))
- fmt: check formatting with `--check` ([#1001](https://github.com/casey/just/pull/1001) by [hdhoang](https://github.com/hdhoang))

### Misc
- Refactor github actions ([#1028](https://github.com/casey/just/pull/1028))
- Fix readme formatting ([#1030](https://github.com/casey/just/pull/1030) by [soenkehahn](https://github.com/soenkehahn))
- Use ps1 extension for pwsh shebangs ([#1027](https://github.com/casey/just/pull/1027) by [dmringo](https://github.com/dmringo))
- Ignore leading byte order mark in source files ([#1021](https://github.com/casey/just/pull/1021))
- Add color to `just --fmt --check` diff ([#1015](https://github.com/casey/just/pull/1015))

[0.10.3](https://github.com/casey/just/releases/tag/0.10.3) - 2021-10-30
------------------------------------------------------------------------

### Added
- Add `trim_end(s)` and `trim_start(s)` functions ([#999](https://github.com/casey/just/pull/999))
- Add more string manipulation functions ([#998](https://github.com/casey/just/pull/998))

### Changed
- Make `join` accept two or more arguments ([#1000](https://github.com/casey/just/pull/1000))

### Misc
- Add alternatives and prior art section to readme ([#1008](https://github.com/casey/just/pull/1008))
- Fix readme `make`'s not correctly displayed ([#1007](https://github.com/casey/just/pull/1007) by [peter50216](https://github.com/peter50216))
- Document the default recipe ([#1006](https://github.com/casey/just/pull/1006))
- Document creating user justfile recipe aliases ([#1005](https://github.com/casey/just/pull/1005))
- Fix readme typo ([#1004](https://github.com/casey/just/pull/1004) by [0xflotus](https://github.com/0xflotus))
- Add packaging status table to readme ([#1003](https://github.com/casey/just/pull/1003))
- Reword `sh` not found error messages ([#1002](https://github.com/casey/just/pull/1002) by [hdhoang](https://github.com/hdhoang))
- Only pass +crt-static to cargo build ([#997](https://github.com/casey/just/pull/997))
- Stop using tabs in justfile in editorconfig ([#996](https://github.com/casey/just/pull/996))
- Use consistent rustflags formatting ([#994](https://github.com/casey/just/pull/994))
- Use `cargo build` instead of `cargo rustc` ([#993](https://github.com/casey/just/pull/993))
- Don't skip variables in variable iterator ([#991](https://github.com/casey/just/pull/991))
- Remove deprecated equals error ([#985](https://github.com/casey/just/pull/985))

[0.10.2](https://github.com/casey/just/releases/tag/0.10.2) - 2021-9-26
-----------------------------------------------------------------------

### Added
- Implement regular expression match conditionals ([#970](https://github.com/casey/just/pull/970))

### Misc
- Add detailed instructions for installing prebuilt binaries ([#978](https://github.com/casey/just/pull/978))
- Improve readme package table formatting ([#977](https://github.com/casey/just/pull/977))
- Add conda package to README ([#976](https://github.com/casey/just/pull/976) by [kellpossible](https://github.com/kellpossible))
- Change MSRV to 1.46.0 ([#968](https://github.com/casey/just/pull/968))
- Use stable rustfmt instead of nightly ([#967](https://github.com/casey/just/pull/967))
- Fix readme typo: FOO → WORLD ([#964](https://github.com/casey/just/pull/964))
- Reword Emacs section in readme ([#962](https://github.com/casey/just/pull/962))
- Mention justl mode for Emacs ([#961](https://github.com/casey/just/pull/961) by [psibi](https://github.com/psibi))

[0.10.1](https://github.com/casey/just/releases/tag/0.10.1) - 2021-8-27
-----------------------------------------------------------------------

### Added
- Add flags for specifying name and path to environment file ([#941](https://github.com/casey/just/pull/941) by [Celeo](https://github.com/Celeo))

### Misc
- Fix error message tests for Alpine Linux ([#956](https://github.com/casey/just/pull/956))
- Bump `target` version to 2.0 ([#957](https://github.com/casey/just/pull/957))
- Mention `tree-sitter-just` in readme ([#951](https://github.com/casey/just/pull/951))
- Document release RSS feed in readme ([#950](https://github.com/casey/just/pull/950))
- Add installation instructions for Gentoo Linux ([#946](https://github.com/casey/just/pull/946) by [dm9pZCAq](https://github.com/dm9pZCAq))
- Make GitHub Actions instructions more prominent ([#944](https://github.com/casey/just/pull/944))
- Wrap `--help` text to terminal width ([#940](https://github.com/casey/just/pull/940))
- Add `.justfile` to sublime syntax file_extensions ([#938](https://github.com/casey/just/pull/938))
- Suggest using `~/.global.justfile` instead of `~/.justfile` ([#937](https://github.com/casey/just/pull/937))
- Update man page ([#935](https://github.com/casey/just/pull/935))

[0.10.0](https://github.com/casey/just/releases/tag/0.10.0) - 2021-8-2
----------------------------------------------------------------------

### Changed
- Warn if `.env` file is loaded in `dotenv-load` isn't explicitly set ([#925](https://github.com/casey/just/pull/925))

### Added
- Add `--changelog` subcommand ([#932](https://github.com/casey/just/pull/932))
- Support `.justfile` as an alternative to `justfile` ([#931](https://github.com/casey/just/pull/931))

### Misc
- Use cargo-limit for all recipes ([#928](https://github.com/casey/just/pull/928))
- Fix colors ([#927](https://github.com/casey/just/pull/927))
- Use ColorDisplay trait to print objects to the terminal ([#926](https://github.com/casey/just/pull/926))
- Deduplicate recipe parsing ([#923](https://github.com/casey/just/pull/923))
- Move subcommand functions into Subcommand ([#918](https://github.com/casey/just/pull/918))
- Check GitHub Actions workflow with actionlint ([#921](https://github.com/casey/just/pull/921))
- Add loader and refactor errors ([#917](https://github.com/casey/just/pull/917))
- Rename: Module → Ast ([#915](https://github.com/casey/just/pull/915))

[0.9.9](https://github.com/casey/just/releases/tag/0.9.9) - 2021-7-22
---------------------------------------------------------------------

### Added
- Add subsequent dependencies ([#820](https://github.com/casey/just/pull/820))
- Implement `else if` chaining ([#910](https://github.com/casey/just/pull/910))

### Fixed
- Fix circular variable dependency error message ([#909](https://github.com/casey/just/pull/909))

### Misc
- Improve readme ([#904](https://github.com/casey/just/pull/904) by [mtsknn](https://github.com/mtsknn))
- Add screenshot to readme ([#911](https://github.com/casey/just/pull/911))
- Add install instructions for Fedora Linux ([#898](https://github.com/casey/just/pull/898) by [olivierlemasle](https://github.com/olivierlemasle))
- Fix readme typos ([#903](https://github.com/casey/just/pull/903) by [rokf](https://github.com/rokf))
- Actually fix release tagging and publish changelog with releases ([#901](https://github.com/casey/just/pull/901))
- Fix broken prerelease tagging ([#900](https://github.com/casey/just/pull/900))
- Use string value for ref-type check ([#897](https://github.com/casey/just/pull/897))

[0.9.8](https://github.com/casey/just/releases/tag/0.9.8) - 2021-7-3
--------------------------------------------------------------------

### Misc
- Fix changelog formatting ([#894](https://github.com/casey/just/pull/894))
- Only run install script on CI for non-releases ([#895](https://github.com/casey/just/pull/895))

[0.9.7](https://github.com/casey/just/releases/tag/0.9.7) - 2021-7-3
--------------------------------------------------------------------

### Added
- Add string manipulation functions ([#888](https://github.com/casey/just/pull/888) by [terror](https://github.com/terror))

### Misc
- Remove test-utilities crate ([#892](https://github.com/casey/just/pull/892))
- Remove outdated note in `Cargo.toml` ([#891](https://github.com/casey/just/pull/891))
- Link to GitHub release pages in changelog ([#886](https://github.com/casey/just/pull/886))

[0.9.6](https://github.com/casey/just/releases/tag/0.9.6) - 2021-6-24
---------------------------------------------------------------------

### Added
- Add `clean` function for simplifying paths ([#883](https://github.com/casey/just/pull/883))
- Add `join` function for joining paths ([#882](https://github.com/casey/just/pull/882))
- Add path manipulation functions ([#872](https://github.com/casey/just/pull/872) by [TonioGela](https://github.com/TonioGela))

### Misc
- Add `file_extensions` to Sublime syntax file ([#878](https://github.com/casey/just/pull/878) by [Frederick888](https://github.com/Frederick888))
- Document path manipulation functions in readme ([#877](https://github.com/casey/just/pull/877))

[0.9.5](https://github.com/casey/just/releases/tag/0.9.5) - 2021-6-12
---------------------------------------------------------------------

### Added
- Add `--unstable` flag ([#869](https://github.com/casey/just/pull/869))
- Add Sublime Text syntax file ([#864](https://github.com/casey/just/pull/864))
- Add `--fmt` subcommand ([#837](https://github.com/casey/just/pull/837) by [vglfr](https://github.com/vglfr))

### Misc
- Mention doniogela.dev/just/ in readme ([#866](https://github.com/casey/just/pull/866))
- Mention that vim-just is now available from vim-polyglot ([#865](https://github.com/casey/just/pull/865))
- Mention `--list-heading` newline behavior ([#860](https://github.com/casey/just/pull/860))
- Check for `rg` in `bin/forbid` ([#859](https://github.com/casey/just/pull/859))
- Document that variables are not exported to backticks in the same scope ([#856](https://github.com/casey/just/pull/856))
- Remove `dotenv_load` from tests ([#853](https://github.com/casey/just/pull/853))
- Remove `v` prefix from version ([#850](https://github.com/casey/just/pull/850))
- Improve install script ([#847](https://github.com/casey/just/pull/847))
- Move pages assets back to `docs` ([#846](https://github.com/casey/just/pull/846))
- Move pages assets to `www` ([#845](https://github.com/casey/just/pull/845))

[0.9.4](https://github.com/casey/just/releases/tag/v0.9.4) - 2021-5-27
----------------------------------------------------------------------

### Misc
- Release `aarch64-unknown-linux-gnu` binaries ([#843](https://github.com/casey/just/pull/843))
- Add `$` to non-default parameter grammar ([#839](https://github.com/casey/just/pull/839))
- Add `$` to parameter grammar ([#838](https://github.com/casey/just/pull/838) by [NoahTheDuke](https://github.com/NoahTheDuke))
- Fix readme links ([#836](https://github.com/casey/just/pull/836))
- Add `vim-just` installation instructions to readme ([#835](https://github.com/casey/just/pull/835))
- Refactor shebang handling ([#833](https://github.com/casey/just/pull/833))

[0.9.3](https://github.com/casey/just/releases/tag/v0.9.3) - 2021-5-16
----------------------------------------------------------------------

### Added
- Add shebang support for 'cmd.exe' ([#828](https://github.com/casey/just/pull/828) by [pansila](https://github.com/pansila))
- Add `.exe` to powershell scripts ([#826](https://github.com/casey/just/pull/826) by [sigoden](https://github.com/sigoden))
- Add the `--command` subcommand ([#824](https://github.com/casey/just/pull/824))

### Fixed
- Fix bang lexing and placate clippy ([#821](https://github.com/casey/just/pull/821))

### Misc
- Fixed missing close apostrophe in GRAMMAR.md ([#830](https://github.com/casey/just/pull/830) by [SOF3](https://github.com/SOF3))
- Make 'else' keyword in grammar ([#829](https://github.com/casey/just/pull/829) by [SOF3](https://github.com/SOF3))
- Add forbid script ([#827](https://github.com/casey/just/pull/827))
- Remove `summary` feature ([#823](https://github.com/casey/just/pull/823))
- Document that just is now in Arch official repo ([#814](https://github.com/casey/just/pull/814) by [svenstaro](https://github.com/svenstaro))
- Fix changelog years ([#813](https://github.com/casey/just/pull/813))

[0.9.2](https://github.com/casey/just/releases/tag/v0.9.2) - 2021-5-02
----------------------------------------------------------------------

### Fixed
- Pass evaluated arguments as positional arguments ([#810](https://github.com/casey/just/pull/810))

[0.9.1](https://github.com/casey/just/releases/tag/v0.9.1) - 2021-4-24
----------------------------------------------------------------------

### Added
- Change `--eval` to print variable value only ([#806](https://github.com/casey/just/pull/806))
- Add `positional-arguments` setting ([#804](https://github.com/casey/just/pull/804))
- Allow filtering variables to evaluate ([#795](https://github.com/casey/just/pull/795))

### Changed
- Reform and improve string literals ([#793](https://github.com/casey/just/pull/793))
- Allow evaluating justfiles with no recipes ([#794](https://github.com/casey/just/pull/794))
- Unify string lexing ([#790](https://github.com/casey/just/pull/790))

### Misc
- Test multi-line strings in interpolation ([#789](https://github.com/casey/just/pull/789))
- Add shell setting examples to README ([#787](https://github.com/casey/just/pull/787))
- Disable .env warning for now
- Warn if `.env` file loaded and `dotenv-load` unset ([#784](https://github.com/casey/just/pull/784))

[0.9.0](https://github.com/casey/just/releases/tag/v0.9.0) - 2021-3-28
----------------------------------------------------------------------

### Changed
- Turn `=` deprecation warning into a hard error ([#780](https://github.com/casey/just/pull/780))

[0.8.7](https://github.com/casey/just/releases/tag/v0.8.7) - 2021-3-28
----------------------------------------------------------------------

### Added
- Add `dotenv-load` setting ([#778](https://github.com/casey/just/pull/778))

### Misc
- Change publish recipe to use stable rust ([#777](https://github.com/casey/just/pull/777))

[0.8.6](https://github.com/casey/just/releases/tag/v0.8.6) - 2021-3-28
----------------------------------------------------------------------

### Added
- Add just_executable() function ([#775](https://github.com/casey/just/pull/775) by [bew](https://github.com/bew))
- Prefix parameters with `$` to export to environment ([#773](https://github.com/casey/just/pull/773))
- Add `set export` to export all variables as environment variables ([#767](https://github.com/casey/just/pull/767))

### Changed
- Suppress all output to stderr when `--quiet` ([#771](https://github.com/casey/just/pull/771))

### Misc
- Improve chooser invocation error message ([#772](https://github.com/casey/just/pull/772))
- De-emphasize cmd.exe in readme ([#768](https://github.com/casey/just/pull/768))
- Fix warnings ([#770](https://github.com/casey/just/pull/770))

[0.8.5](https://github.com/casey/just/releases/tag/v0.8.5) - 2021-3-24
----------------------------------------------------------------------

### Added
- Allow escaping double braces with `{{{{` ([#765](https://github.com/casey/just/pull/765))

### Misc
- Reorganize readme to highlight editor support ([#764](https://github.com/casey/just/pull/764))
- Add categories and keywords to Cargo manifest ([#763](https://github.com/casey/just/pull/763))
- Fix command output in readme ([#760](https://github.com/casey/just/pull/760) by [vvv](https://github.com/vvv))
- Note Emacs package `just-mode` in readme ([#759](https://github.com/casey/just/pull/759) by [leon-barrett](https://github.com/leon-barrett))
- Note shebang line splitting inconsistency in readme ([#757](https://github.com/casey/just/pull/757))

[0.8.4](https://github.com/casey/just/releases/tag/v0.8.4) - 2021-2-9
---------------------------------------------------------------------

### Added
- Add options to control list formatting ([#753](https://github.com/casey/just/pull/753))

### Misc
- Document how to change the working directory in a recipe ([#752](https://github.com/casey/just/pull/752))
- Implement `Default` for `Table` ([#748](https://github.com/casey/just/pull/748))
- Add Alpine Linux package to readme ([#736](https://github.com/casey/just/pull/736) by [jirutka](https://github.com/jirutka))
- Update to actions/cache@v2 ([#742](https://github.com/casey/just/pull/742) by [zyctree](https://github.com/zyctree))
- Add link in readme to GitHub Action ([#729](https://github.com/casey/just/pull/729) by [rossmacarthur](https://github.com/rossmacarthur))
- Add docs for justfile() and justfile_directory() ([#726](https://github.com/casey/just/pull/726) by [rminderhoud](https://github.com/rminderhoud))
- Fix CI ([#727](https://github.com/casey/just/pull/727))
- Improve readme ([#725](https://github.com/casey/just/pull/725))
- Replace saythanks.io link with malto: link ([#723](https://github.com/casey/just/pull/723))
- Update man page to v0.8.3 ([#720](https://github.com/casey/just/pull/720))

[0.8.3](https://github.com/casey/just/releases/tag/v0.8.3) - 2020-10-27
-----------------------------------------------------------------------

### Added
- Allow ignoring line endings inside delimiters ([#717](https://github.com/casey/just/pull/717))

[0.8.2](https://github.com/casey/just/releases/tag/v0.8.2) - 2020-10-26
-----------------------------------------------------------------------

### Added
- Add conditional expressions ([#714](https://github.com/casey/just/pull/714))

### Fixed
- Allow completing variables and recipes after `--set` in zsh completion script ([#697](https://github.com/casey/just/pull/697) by [heyrict](https://github.com/heyrict))

### Misc
- Add Parser::forbid ([#712](https://github.com/casey/just/pull/712))
- Automatically track expected tokens while parsing ([#711](https://github.com/casey/just/pull/711))
- Document feature flags in Cargo.toml ([#709](https://github.com/casey/just/pull/709))

[0.8.1](https://github.com/casey/just/releases/tag/v0.8.1) - 2020-10-15
-----------------------------------------------------------------------

### Changed
- Allow choosing multiple recipes to run ([#700](https://github.com/casey/just/pull/700))
- Complete recipes in bash completion script ([#685](https://github.com/casey/just/pull/685) by [vikesh-raj](https://github.com/vikesh-raj))
- Complete recipes names in PowerShell completion script ([#651](https://github.com/casey/just/pull/651) by [Insomniak47](https://github.com/Insomniak47))

### Misc
- Add FreeBSD port to readme ([#705](https://github.com/casey/just/pull/705))
- Placate clippy ([#698](https://github.com/casey/just/pull/698))
- Fix build fix ([#693](https://github.com/casey/just/pull/693))
- Fix readme documentation for ignoring errors ([#692](https://github.com/casey/just/pull/692) by [kenden](https://github.com/kenden))

[0.8.0](https://github.com/casey/just/releases/tag/v0.8.0) - 2020-10-3
----------------------------------------------------------------------

### Breaking
- Allow suppressing failures with `-` prefix ([#687](https://github.com/casey/just/pull/687) by [iwillspeak](https://github.com/iwillspeak))

### Misc
- Document how to ignore errors with `-` in readme ([#690](https://github.com/casey/just/pull/690))
- Install BSD Tar on GitHub Actions to fix CI errors ([#689](https://github.com/casey/just/pull/689))
- Move separate quiet config value to verbosity ([#686](https://github.com/casey/just/pull/686) by [Celeo](https://github.com/Celeo))

[0.7.3](https://github.com/casey/just/releases/tag/v0.7.3) - 2020-9-17
----------------------------------------------------------------------

### Added
- Add the `--choose` subcommand ([#680](https://github.com/casey/just/pull/680))

### Misc
- Combine integration tests into single binary ([#679](https://github.com/casey/just/pull/679))
- Document `--unsorted` flag in readme ([#672](https://github.com/casey/just/pull/672))

[0.7.2](https://github.com/casey/just/releases/tag/v0.7.2) - 2020-8-23
----------------------------------------------------------------------

### Added
- Add option to print recipes in source order ([#669](https://github.com/casey/just/pull/669))

### Misc
- Mention Linux, MacOS and Windows support in readme ([#666](https://github.com/casey/just/pull/666))
- Add list highlighting nice features to readme ([#664](https://github.com/casey/just/pull/664))

[0.7.1](https://github.com/casey/just/releases/tag/v0.7.1) - 2020-7-19
----------------------------------------------------------------------

### Fixed
- Search for `.env` file from working directory ([#661](https://github.com/casey/just/pull/661))

### Misc
- Move link-time optimization config into `Cargo.toml` ([#658](https://github.com/casey/just/pull/658))

[0.7.0](https://github.com/casey/just/releases/tag/v0.7.0) - 2020-7-16
----------------------------------------------------------------------

### Breaking
- Skip `.env` items which are set in environment ([#656](https://github.com/casey/just/pull/656))

### Misc
- Mark tags that start with `v` as releases ([#654](https://github.com/casey/just/pull/654))

[0.6.1](https://github.com/casey/just/releases/tag/v0.6.1) - 2020-6-28
----------------------------------------------------------------------

### Changed
- Only use `cygpath` on shebang if it contains `/` ([#652](https://github.com/casey/just/pull/652))

[0.6.0](https://github.com/casey/just/releases/tag/v0.6.0) - 2020-6-18
----------------------------------------------------------------------

### Changed
- Ignore '@' returned from interpolation evaluation ([#636](https://github.com/casey/just/pull/636) by [rjsberry](https://github.com/rjsberry))
- Strip leading spaces after line continuation ([#635](https://github.com/casey/just/pull/635))

### Added
- Add variadic parameters that accept zero or more arguments ([#645](https://github.com/casey/just/pull/645) by [rjsberry](https://github.com/rjsberry))

### Misc
- Clarify variadic parameter default values ([#646](https://github.com/casey/just/pull/646) by [rjsberry](https://github.com/rjsberry))
- Add keybase example justfile  ([#640](https://github.com/casey/just/pull/640) by [blaggacao](https://github.com/blaggacao))
- Strip trailing whitespace in `examples/pre-commit.just` ([#644](https://github.com/casey/just/pull/644))
- Test that example justfiles successfully parse ([#643](https://github.com/casey/just/pull/643))
- Link example justfiles in readme ([#641](https://github.com/casey/just/pull/641))
- Add example justfile ([#639](https://github.com/casey/just/pull/639) by [blaggacao](https://github.com/blaggacao))
- Document how to run recipes after another recipe ([#630](https://github.com/casey/just/pull/630))

[0.5.11](https://github.com/casey/just/releases/tag/v0.5.11) - 2020-5-23
------------------------------------------------------------------------

### Added
- Don't load `.env` file when `--no-dotenv` is passed ([#627](https://github.com/casey/just/pull/627))

### Changed
- Complete recipe names in fish completion script ([#625](https://github.com/casey/just/pull/625) by [tyehle](https://github.com/tyehle))
- Suggest aliases for unknown recipes ([#624](https://github.com/casey/just/pull/624) by [Celeo](https://github.com/Celeo))

[0.5.10](https://github.com/casey/just/releases/tag/v0.5.10) - 2020-3-18
------------------------------------------------------------------------

[0.5.9](https://github.com/casey/just/releases/tag/v0.5.9) - 2020-3-18
----------------------------------------------------------------------

### Added
- Update zsh completion file ([#606](https://github.com/casey/just/pull/606) by [heyrict](https://github.com/heyrict))
- Add `--variables` subcommand that prints variable names ([#608](https://github.com/casey/just/pull/608))
- Add github pages site with improved install script ([#597](https://github.com/casey/just/pull/597))

### Fixed
- Don't require justfile to print completions ([#596](https://github.com/casey/just/pull/596))

### Misc
- Only build for linux on docs.rs ([#611](https://github.com/casey/just/pull/611))
- Trim completions and ensure final newline ([#609](https://github.com/casey/just/pull/609))
- Trigger build on pushes and pull requests ([#607](https://github.com/casey/just/pull/607))
- Document behavior of `@` on shebang recipes ([#602](https://github.com/casey/just/pull/602))
- Add `.nojekyll` file to github pages site ([#599](https://github.com/casey/just/pull/599))
- Add `:` favicon ([#598](https://github.com/casey/just/pull/598))
- Delete old CI configuration and update build badge ([#595](https://github.com/casey/just/pull/595))
- Add download count badge to readme ([#594](https://github.com/casey/just/pull/594))
- Wrap comments at 80 characters ([#593](https://github.com/casey/just/pull/593))
- Use unstable rustfmt configuration options ([#592](https://github.com/casey/just/pull/592))

[0.5.8](https://github.com/casey/just/releases/tag/v0.5.8) - 2020-1-28
----------------------------------------------------------------------

### Changed
- Only use `cygpath` on windows if present ([#586](https://github.com/casey/just/pull/586))

### Misc
- Improve comments in justfile ([#588](https://github.com/casey/just/pull/588))
- Remove unused dependencies ([#587](https://github.com/casey/just/pull/587))

[0.5.7](https://github.com/casey/just/releases/tag/v0.5.7) - 2020-1-28
----------------------------------------------------------------------

### Misc
- Don't include directories in release archive ([#583](https://github.com/casey/just/pull/583))

[0.5.6](https://github.com/casey/just/releases/tag/v0.5.6) - 2020-1-28
----------------------------------------------------------------------

### Misc
- Build and upload release artifacts from GitHub Actions ([#581](https://github.com/casey/just/pull/581))
- List solus package in readme ([#579](https://github.com/casey/just/pull/579))
- Expand use of GitHub Actions ([#580](https://github.com/casey/just/pull/580))
- Fix readme typo: interpetation -> interpretation ([#578](https://github.com/casey/just/pull/578) by [Plommonsorbet](https://github.com/Plommonsorbet))

[0.5.5](https://github.com/casey/just/releases/tag/v0.5.5) - 2020-1-15
----------------------------------------------------------------------

### Added
- Generate shell completion scripts with `--completions` ([#572](https://github.com/casey/just/pull/572))

### Misc
- Check long lines and FIXME/TODO on CI ([#575](https://github.com/casey/just/pull/575))
- Add additional continuous integration checks ([#574](https://github.com/casey/just/pull/574))

[0.5.4](https://github.com/casey/just/releases/tag/v0.5.4) - 2019-12-25
-----------------------------------------------------------------------

### Added
- Add `justfile_directory()` and `justfile()` ([#569](https://github.com/casey/just/pull/569))

### Misc
- Add table of package managers that include just to readme ([#568](https://github.com/casey/just/pull/568))
- Remove yaourt AUR helper from readme ([#567](https://github.com/casey/just/pull/567) by [ky0n](https://github.com/ky0n))
- Fix regression in error message color printing ([#566](https://github.com/casey/just/pull/566))
- Reform indentation handling ([#565](https://github.com/casey/just/pull/565))
- Update Cargo.lock with new version ([#564](https://github.com/casey/just/pull/564))

[0.5.3](https://github.com/casey/just/releases/tag/v0.5.3) - 2019-12-11
-----------------------------------------------------------------------

### Misc
- Assert that lexer advances over entire input ([#560](https://github.com/casey/just/pull/560))
- Fix typo: `chracter` -> `character` ([#561](https://github.com/casey/just/pull/561))
- Improve pre-publish check ([#562](https://github.com/casey/just/pull/562))

[0.5.2](https://github.com/casey/just/releases/tag/v0.5.2) - 2019-12-7
----------------------------------------------------------------------

### Added
- Add flags to set and clear shell arguments ([#551](https://github.com/casey/just/pull/551))
- Allow passing arguments to dependencies ([#555](https://github.com/casey/just/pull/555))

### Misc
- Un-implement Deref for Table ([#546](https://github.com/casey/just/pull/546))
- Resolve recipe dependencies ([#547](https://github.com/casey/just/pull/547))
- Resolve alias targets ([#548](https://github.com/casey/just/pull/548))
- Remove unnecessary type argument to Alias ([#549](https://github.com/casey/just/pull/549))
- Resolve functions ([#550](https://github.com/casey/just/pull/550))
- Reform scope and binding ([#556](https://github.com/casey/just/pull/556))

[0.5.1](https://github.com/casey/just/releases/tag/v0.5.1) - 2019-11-20
-----------------------------------------------------------------------

### Added
- Add `--init` subcommand ([#541](https://github.com/casey/just/pull/541))

### Changed
- Avoid fs::canonicalize ([#539](https://github.com/casey/just/pull/539))

### Misc
- Mention `set shell` as alternative to installing `sh` ([#533](https://github.com/casey/just/pull/533))
- Refactor Compilation error to contain a Token ([#535](https://github.com/casey/just/pull/535))
- Move lexer comment ([#536](https://github.com/casey/just/pull/536))
- Add missing `--init` test ([#543](https://github.com/casey/just/pull/543))

[0.5.0](https://github.com/casey/just/releases/tag/v0.5.0) - 2019-11-12
-----------------------------------------------------------------------

### Added

- Add `set shell := [...]` to grammar ([#526](https://github.com/casey/just/pull/526))
- Add `shell` setting ([#525](https://github.com/casey/just/pull/525))
- Document settings in readme ([#527](https://github.com/casey/just/pull/527))

### Changed
- Reform positional argument parsing ([#523](https://github.com/casey/just/pull/523))
- Highlight echoed recipe lines in bold by default ([#512](https://github.com/casey/just/pull/512))

### Misc

- Gargantuan refactor ([#522](https://github.com/casey/just/pull/522))
- Move subcommand execution into Subcommand ([#514](https://github.com/casey/just/pull/514))
- Move `cd` out of Config::from_matches ([#513](https://github.com/casey/just/pull/513))
- Remove now-unnecessary borrow checker appeasement ([#511](https://github.com/casey/just/pull/511))
- Reform Parser ([#509](https://github.com/casey/just/pull/509))
- Note need to publish with nightly cargo ([#506](https://github.com/casey/just/pull/506))

[0.4.5](https://github.com/casey/just/releases/tag/v0.4.5) - 2019-10-31
-----------------------------------------------------------------------

### User-visible

### Changed
- Display alias with `--show NAME` if one exists

### Documented
- Document multi-line constructs (for/if/while) ([#453](https://github.com/casey/just/pull/453))
- Generate man page with help2man ([#463](https://github.com/casey/just/pull/463))
- Add context to deprecation warnings ([#473](https://github.com/casey/just/pull/473))
- Improve messages for alias error messages ([#500](https://github.com/casey/just/pull/500))

### Misc

### Cleanup
- Update deprecated rust range patterns and clippy config ([#450](https://github.com/casey/just/pull/450) by [light4](https://github.com/light4))
- Make comments in common.rs lowercase ([#470](https://github.com/casey/just/pull/470))
- Use `pub(crate)` instead of `pub` ([#471](https://github.com/casey/just/pull/471))
- Hide summary functionality behind feature flag ([#472](https://github.com/casey/just/pull/472))
- Fix `summary` feature conditional compilation ([#475](https://github.com/casey/just/pull/475))
- Allow integration test cases to omit common values ([#480](https://github.com/casey/just/pull/480))
- Add `unindent()` for nicer integration test strings ([#481](https://github.com/casey/just/pull/481))
- Start pulling argument parsing out of run::run() ([#483](https://github.com/casey/just/pull/483))
- Add explicit `Subcommand` enum ([#484](https://github.com/casey/just/pull/484))
- Avoid using error code `1` in integration tests ([#486](https://github.com/casey/just/pull/486))
- Use more indented strings in integration tests ([#489](https://github.com/casey/just/pull/489))
- Refactor `run::run` and Config ([#490](https://github.com/casey/just/pull/490))
- Remove `misc.rs` ([#491](https://github.com/casey/just/pull/491))
- Remove unused `use` statements ([#497](https://github.com/casey/just/pull/497))
- Refactor lexer tests ([#498](https://github.com/casey/just/pull/498))
- Use constants instead of literals in arg parser ([#504](https://github.com/casey/just/pull/504))

### Infrastructure
- Add repository attribute to Cargo.toml ([#493](https://github.com/casey/just/pull/493) by [SOF3](https://github.com/SOF3))
- Check minimal version compatibility before publishing ([#487](https://github.com/casey/just/pull/487))

### Continuous Integration
- Disable FreeBSD builds ([#474](https://github.com/casey/just/pull/474))
- Use `bash` as shell for all integration tests ([#479](https://github.com/casey/just/pull/479))
- Don't install `dash` on Travis ([#482](https://github.com/casey/just/pull/482))

### Dependencies
- Use `tempfile` crate instead of `tempdir` ([#455](https://github.com/casey/just/pull/455) by [NickeZ](https://github.com/NickeZ))
- Bump clap dependency to 2.33.0 ([#458](https://github.com/casey/just/pull/458) by [NickeZ](https://github.com/NickeZ))
- Minimize dependency version requirements ([#461](https://github.com/casey/just/pull/461))
- Remove dependency on brev ([#462](https://github.com/casey/just/pull/462))
- Update dependencies ([#501](https://github.com/casey/just/pull/501))

[0.4.4](https://github.com/casey/just/releases/tag/v0.4.4) - 2019-06-02
-----------------------------------------------------------------------

### Changed
- Ignore file name case while searching for justfile ([#436](https://github.com/casey/just/pull/436) by [shevtsiv](https://github.com/shevtsiv))

### Added
- Display alias target with `--show` ([#443](https://github.com/casey/just/pull/443))

[0.4.3](https://github.com/casey/just/releases/tag/v0.4.3) - 2019-05-07
-----------------------------------------------------------------------

### Changed
- Deprecate `=` in assignments, aliases, and exports in favor of `:=` ([#413](https://github.com/casey/just/pull/413))

### Added
- Pass stdin handle to backtick process ([#409](https://github.com/casey/just/pull/409))

### Documented
- Fix readme command line ([#411](https://github.com/casey/just/pull/411))
- Typo: "command equivelant" -> "command equivalent" ([#418](https://github.com/casey/just/pull/418))
- Mention Make’s “phony target” workaround in the comparison ([#421](https://github.com/casey/just/pull/421) by [roryokane](https://github.com/roryokane))
- Add Void Linux install instructions to readme ([#423](https://github.com/casey/just/pull/423))

### Cleaned up or Refactored
- Remove stray source files ([#408](https://github.com/casey/just/pull/408))
- Replace some calls to brev crate ([#410](https://github.com/casey/just/pull/410))
- Lexer code deduplication and refactoring ([#414](https://github.com/casey/just/pull/414))
- Refactor and rename test macros ([#415](https://github.com/casey/just/pull/415))
- Move CompilationErrorKind into separate module ([#416](https://github.com/casey/just/pull/416))
- Remove `write_token_error_context` ([#417](https://github.com/casey/just/pull/417))

[0.4.2](https://github.com/casey/just/releases/tag/v0.4.2) - 2019-04-12
-----------------------------------------------------------------------

### Changed
- Regex-based lexer replaced with much nicer character-at-a-time lexer ([#406](https://github.com/casey/just/pull/406))

[0.4.1](https://github.com/casey/just/releases/tag/v0.4.1) - 2019-04-12
-----------------------------------------------------------------------

### Changed
- Make summary function non-generic ([#404](https://github.com/casey/just/pull/404))

[0.4.0](https://github.com/casey/just/releases/tag/v0.4.0) - 2019-04-12
-----------------------------------------------------------------------

### Added
- Add recipe aliases ([#390](https://github.com/casey/just/pull/390) by [ryloric](https://github.com/ryloric))
- Allow arbitrary expressions as default arguments ([#400](https://github.com/casey/just/pull/400))
- Add justfile summaries ([#399](https://github.com/casey/just/pull/399))
- Allow outer shebang lines so justfiles can be used as scripts ([#393](https://github.com/casey/just/pull/393))
- Allow `--justfile` without `--working-directory` ([#392](https://github.com/casey/just/pull/392) by [smonami](https://github.com/smonami))
- Add link to Chinese translation of readme by chinanf-boy ([#377](https://github.com/casey/just/pull/377))

### Changed
- Upgrade to Rust 2018 ([#394](https://github.com/casey/just/pull/394))
- Format the codebase with rustfmt ([#346](https://github.com/casey/just/pull/346))

[0.3.13](https://github.com/casey/just/releases/tag/v0.3.13) - 2018-11-06
-------------------------------------------------------------------------

### Added
- Print recipe signature if missing arguments ([#369](https://github.com/casey/just/pull/369) by [ladysamantha](https://github.com/ladysamantha))
- Add grandiloquent verbosity level that echos shebang recipes ([#348](https://github.com/casey/just/pull/348))
- Wait for child processes to finish ([#345](https://github.com/casey/just/pull/345))
- Improve invalid escape sequence error messages ([#328](https://github.com/casey/just/pull/328))

### Fixed
- Use PutBackN instead of PutBack in parser ([#364](https://github.com/casey/just/pull/364))

[0.3.12](https://github.com/casey/just/releases/tag/v0.3.12) - 2018-06-19
-------------------------------------------------------------------------

### Added
- Implemented invocation_directory function

[0.3.11](https://github.com/casey/just/releases/tag/v0.3.11) - 2018-05-6
------------------------------------------------------------------------

### Fixed
- Fixed colors on windows ([#317](https://github.com/casey/just/pull/317))

[0.3.10](https://github.com/casey/just/releases/tag/v0.3.10) - 2018-3-19
------------------------------------------------------------------------

### Added
- Make .env vars available in env_var functions ([#310](https://github.com/casey/just/pull/310))

[0.3.8](https://github.com/casey/just/releases/tag/v0.3.8) - 2018-3-5
---------------------------------------------------------------------

### Added
- Add dotenv integration ([#306](https://github.com/casey/just/pull/306))

[0.3.7](https://github.com/casey/just/releases/tag/v0.3.7) - 2017-12-11
-----------------------------------------------------------------------

### Fixed
- Fix error if ! appears in comment ([#296](https://github.com/casey/just/pull/296))

[0.3.6](https://github.com/casey/just/releases/tag/v0.3.6) - 2017-12-11
-----------------------------------------------------------------------

### Fixed
- Lex CRLF line endings properly ([#292](https://github.com/casey/just/pull/292))

[0.3.5](https://github.com/casey/just/releases/tag/v0.3.5) - 2017-12-11
-----------------------------------------------------------------------

### Added
- Align doc-comments in `--list` output ([#273](https://github.com/casey/just/pull/273))
- Add `arch()`, `os()`, and `os_family()` functions ([#277](https://github.com/casey/just/pull/277))
- Add `env_var(key)` and `env_var_or_default(key, default)` functions ([#280](https://github.com/casey/just/pull/280))

[0.3.4](https://github.com/casey/just/releases/tag/v0.3.4) - 2017-10-06
-----------------------------------------------------------------------

### Added
- Do not evaluate backticks in assignments during dry runs ([#253](https://github.com/casey/just/pull/253) by [aoeu](https://github.com/aoeu))

### Changed
- Change license to CC0 going forward ([#270](https://github.com/casey/just/pull/270))

[0.3.1](https://github.com/casey/just/releases/tag/v0.3.1) - 2017-10-06
-----------------------------------------------------------------------

### Added
- Started keeping a changelog in CHANGELOG.md ([#220](https://github.com/casey/just/pull/220))
- Recipes whose names begin with an underscore will not appear in `--list` or `--summary` ([#229](https://github.com/casey/just/pull/229))
