# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- Don't use query feeds when generating RSS xml.

## [1.0.5] 2024-12-30
### Added
- Add RSS channel for all the feeds being processed
- Generate OPML file for the Liveboat feeds

## [1.0.4] 2024-12-27
### Fixed
- Fix parsing age for articles having incorrectly set date (into the future)

### Added
- Add missing `is_query` field for serialized feeds
- Add option to override config dir with env var

### Changed
- Refactored command handlers into separate module
- Added builders module
- Update colors for default template

## [1.0.3] 2024-12-23
### Changed
- Use crates.io version of libnewsboat
 
### Fixed
- Don't overwrite user defined settings during template update

## [1.0.2] 2024-12-22
### Added
- Refactor release scripts

## [1.0.1] 2024-12-22
### Added
- Add files for AUR release

## [1.0.1] 2024-12-22
### Added
- Add files for AUR release

## [1.0.1] 2024-12-22
### Added
- Add files for AUR release

## [1.0.0] 2024-12-21
### Fixed
- Fix sorting article items in proper descending order
 
### Added
- Add build time query param to feed calls make sure no calls are cached by the browser
- Add modules for updating Liveboat binaries and templates
- Add ability to override template_dir with env variable
 
## [0.9.0] 2024-12-15

### Added
- Initial version of Liveboat
