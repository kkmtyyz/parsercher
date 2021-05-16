# Changelog

## [3.1.0] - 2021-05-16

### Additions

- Added new API `parsercher::search_attrs()`.


## [3.0.0] - 2021-05-16

### Additions

- Added new API `parsercher::search_attr()`.
- Added PartialEq to Dom Types.

### Changes

- Changed the `satisfy_sufficient_condition()` to the `Dom::p_implies_q()`.
- Changed constructor arguments to slices (Tag, Text, Comment).


## [2.1.0] - 2021-05-14

### Additions

- Added new API `parsercher::search_dom()`
- Added new API `Dom::p_implies_q()`
- Added new API `Dom::p_implies_q_tree()`


## [2.0.0] - 2021-05-09

### Additions

- Various tests.

### Changes

- Changed the `Tag::set_attr()` to the `Tag::set_attrs()`.
- Changed the `Tag::get_attr()` to the `Tag::get_attrs()`.
- Updated the `Tag::set_attr()` and the `Tag::get_attr()`.
- Moved the `satisfy_sufficient_condition()` to the `tag` mod from the `searcher` mod.


## [1.0.0] - 2021-05-08
Initial release.

[3.1.0]: https://github.com/kkmtyyz/parsercher/compare/3.0.0...3.1.0
[3.0.0]: https://github.com/kkmtyyz/parsercher/compare/2.1.0...3.0.0
[2.1.0]: https://github.com/kkmtyyz/parsercher/compare/2.0.0...2.1.0
[2.0.0]: https://github.com/kkmtyyz/parsercher/compare/1.0.0...2.0.0
[1.0.0]: https://github.com/kkmtyyz/parsercher/compare/1.0.0
