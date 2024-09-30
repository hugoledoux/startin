

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.8.1] - 2024-09-30
### Changed
- fix bug with Laplace/NNI interpolation when interpolate same location

## [0.8.0] - 2024-07-16
## Added
- vertices of the DT can now have attributes attached to them, just a JSON dict (a serde Value is used). You need to define a schema first. It's probably easier to use in Python, integrated with NumPy to some extent
- xy-duplicates are now handled and the behaviour can be configured. In previous versions, it was first-come-first-served, but now these 4 options are possible: First/Last/Highest/Lowest (First==default). This means that if a new vertex is an xy-duplicate (based on `dt.snap_tolerance`), the z value kept is depending on the configuration (the same is done if extra attributes are stored for the vertex).
- added functions calculate area2d/3d and volume of triangles, due to popular demand
## Changed
- many bugs were fixed, eg PLY output is now valid and outputs all extra attributes, the CityJSON output now correctly omits the infinity vertex, if removed() created a non-initialised DT then it doesn't panic anymore

## [0.7.1] - 2023-12-14
## Changed
- fix a bug where function `adjacent_triangles_to_triangle()` returned only the finite triangles, while all (incl. infinite) should be returned
- more recent crate "robust" is used

## [0.7.0] - 2023-08-16
## Changed
- interpolate methods are now in a separate module and thus the way to call them is totally different. Not backwards compatible either, see `/examples/interpol.rs` for how to use them.
- IDW was added to the list of interpolation, the simplest version with a search radius.
- functions are now clearer: `all_triangles()` and `all_finite_triangles()`
- a few bugs were fixed, related to functions used before the triangulation was initialised.
## Removed
- the export to GeoJSON was removed in order to keep the dependencies of the library as little as possible. The Python bindings now have that function, you can fetch the code if you need to do this (https://github.com/hugoledoux/startinpy/blob/develop/src/lib.rs#L762)

## [0.6.2] - 2023-01-04
## Changed
- interpolate_nni() is now faster and better and more stable: the unbounded Voronoi cells are not bounded anymore, instead a trick is used to calculate a fake area and then since the trick is used twice (and values subtracted) then everything cancels out.
- fixed a bug when remove() meant that the DT was formed of only collinear points. Thanks @OliverJPost for finding that bug.

## [0.6.1] - 2022-09-28
## Added
- collect_garbage() to remove from memory/array the vertices/stars marked as to be removed
- has_garbage() to know if above should be used
## Changed
- insert() with BBow now performs automatically collect_garbage(), otherwise one had 4 vertices they never seen there in the array and that was problematic

## [0.6.0] - 2022-09-20
### Added
- get_bbox() function, an AABB one that is
- insert() now as insertion strategies: as is (order given is inserted), and BBox where a square containing all points is first inserted (and deleted at the end), this speeds up points in raster runlength order a lot.
- a few functions added to c_api
- export to PLY format
### Changed
- a Point is not a Vec anymore, but an array [f64;3]. If you want 2D, add 0.0 as z-value for your points.
- fixed some minor bugs, eg with the OBJ export when points had been deleted
- uses now Rust 2021
- most functions now return a Result if something outside the convexhull is done or if a vertex ID doesn't exist, and `StartinError` are defined: VertexUnknown, VertexRemoved, VertexInfinite, TriangleNotPresent, OutsideConvexHull, NoTriangleinTIN, 
- docs was improved

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

[0.8.1]: https://github.com/hugoledoux/startin/compare/0.8.0...0.8.1
[0.8.0]: https://github.com/hugoledoux/startin/compare/0.7.1...0.8.0
[0.7.0]: https://github.com/hugoledoux/startin/compare/0.6.1...0.7.0
[0.6.2]: https://github.com/hugoledoux/startin/compare/0.6.1...0.6.2
[0.6.1]: https://github.com/hugoledoux/startin/compare/0.6.0...0.6.1
[0.6.0]: https://github.com/hugoledoux/startin/compare/0.5.3...0.6.0
