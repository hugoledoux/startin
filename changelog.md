

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.4.2] - 2019-06-12
### Changed
- predicates.c is not used anymore. The Rust port of it (https://github.com/Stoeoef/spade/blob/master/src/exactpred.rs) is used.
- dependencies for the examples are not used/listed for the library anymore.


## [0.4.1] - 2019-06-11
### Changed
- predicates.c has sys removed from include for time.h
- jump-and-walk is not the default anymore, walk starts from last one (no randomness by default thus)


## [0.4.0] - 2019-06-06
### Added
- Deletion of vertices now possible, even those on the boundary of the convex hull
- Integration tests available in the /tests/ folder

## [0.3.1] - 2019-05-06
### Changed
- more examples
- fix of readme.md and a few things for crates.io

## [0.3.0] - 2019-05-02
### Added
- first release and upload to crates.io

