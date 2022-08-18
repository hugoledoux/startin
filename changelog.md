

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.5.3] - 2021-12-24
### Changed
- added wrapper in c_api around some interpolation functions
- changed slightly the c_api with better return types

## [0.5.2] - 2021-12-16
### Changed
- fixed a bug that returned the wrong nearest neighbour (and thus wrong interpolate_nn() results)
- improve the geojson output, now the id+z of the vertices are saved

## [0.5.1] - 2021-06-10
### Changed
- fixed a small bug that arised sometimes when deleting a vertex on the convex hull

## [0.5.0] - 2021-04-15
### Added
- interplation with natural neighbour (nni, or Sibson's method) is added. 
- saving of the triangulation to GeoJSON is added
### Changed
- Delete the robust arithmetic code copied from spades, and use Rust crate "robust"
- interpolation functions are more robust (if no DT exists, if estimation at known vertex)

## [0.4.9] - 2021-03-07
### Added
- Added basic C interface, so startin can be called from other languages (such as C or Julia). Build with `cargo build --features c_api`.

## [0.4.8] - 2021-02-05 
### Changed
- Fix a small bug in walk, that seemed to have no real effect (except slowly down a bit)

## [0.4.7] - 2019-11-20
### Changed
- Fix the bug about predicates.rs raised by Martijn Meijers (https://github.com/Stoeoef/spade/issues/48)

## [0.4.6] - 2019-08-22
### Added
- 3 interpolation functions, based on the DT, added: nearest-neighbour, linear in TIN, Laplace.
### Changed
- fixed a bug with walking that sometimes crashed when point outside convex hull were inserted
- the OBJ writer is now about 1283X faster


## [0.4.5] - 2019-07-30
### Changed
- closest_vertex() is now returning the real natural neighbour, and not an approximation


## [0.4.4] - 2019-07-29
### Changed
- fixed a few bugs, most important is when walking when starting vertex was infinity vertex, now no crash
- `all_edges()` function to draw faster, used by startin_wasm project


## [0.4.3] - 2019-07-26
### Changed
- minor improvements to the API, alignment with CGAL SurfaceMesh functions (more or less)
- better doc


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

