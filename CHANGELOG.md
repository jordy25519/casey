
# Change Log
All notable changes to this project will be documented in this file.
 
The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [0.3.3] - 2020-06-10

### Fixed
Apply ident transformation to TokenTree::Groups

## [0.3.2] - 2020-01-10

### Fixed
Ignore keywords in ident transformations

## [0.3.1] - 2020-30-09

### Fixed
- pascal! macro handles SHOUTY_CASE transforms

## [0.2.0] - 2019-02-08

### Added

### Changed
- Exported macros now receive and operate on a token stream
- Removed heck dependency

### Fixed
- Camel case macros were actually Pascal case, renamed as such
