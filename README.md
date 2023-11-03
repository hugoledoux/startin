# startin

[![crates.io](https://img.shields.io/crates/v/startin?color=%2355ab4e&style=for-the-badge)](https://crates.io/crates/startin)
[![docs.rs](https://img.shields.io/docsrs/startin?style=for-the-badge)](https://docs.rs/startin)
[![PyPI](https://img.shields.io/pypi/v/startinpy?style=for-the-badge)](https://pypi.org/project/startinpy/)

A Delaunay triangulator where the input are 2.5D points, the DT is computed in 2D but the elevation of the vertices are kept.
This is used mostly for the modelling of terrains.
Constructing a 2D Delaunay triangulation is also possible.

The construction algorithm used is an incremental insertion based on flips, and the data structure is a cheap implementation of the star-based structure defined in [Blandford et al. (2003)](https://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.9.6823), cheap because the link of each vertex is stored a simple array (`Vec`) and not in an optimised blob like they did.
It results in a pretty fast library (comparison will come at some point), but it uses more space than the optimised one.

The deletion of a vertex is also possible. The algorithm implemented is a modification of the one of [Mostafavi, Gold, and Dakowicz (2003)](https://doi.org/10.1016/S0098-3004(03)00017-7). 
The ears are filled by flipping, so it's in theory more robust. 
I have also extended the algorithm to allow the deletion of vertices on the boundary of the convex hull. 
The algorithm is sub-optimal, but in practice the number of neighbours of a given vertex in a DT is only 6, so it doesn't really matter.

Robust arithmetic for the geometric predicates are used ([Shewchuk's predicates](https://www.cs.cmu.edu/~quake/robust.html), well the [Rust port of the code (robust crate)](https://crates.io/crates/robust)), so startin is robust and shouldn't crash (touch wood). 

There are a few interpolation functions implemented: (1) nearest-neighbour, (2) linear in TIN, (3) Laplace, (4) natural neighbour (aka Sibson's interpolation), (5) IDW with search-radius.


# Web-demo with WebAssembly

Rust can be compiled to [WebAssembly](https://www.rust-lang.org/what/wasm), and you can see a demo of some of the possibilities of startin (all computations are done locally and it's fast!).

[--> web-demo](https://hugoledoux.github.io/startin_wasm/)


# Python bindings

If you prefer Python, I made bindings: [https://github.com/hugoledoux/startinpy/](https://github.com/hugoledoux/startinpy/)

There are a few more functions (eg reading GeoTIFF, LAZ) and it works with Numpy.


# C interface

A basic C interface is available in `src/c_interface.rs`, to compile it:

```bash
cargo build --features c_api
```

# Documentation

You can read the complete documentation [here](https://docs.rs/startin)

# Usage

```rust
extern crate startin;
fn main() {
    let mut pts = Vec::new();
    pts.push([20.0, 30.0, 2.0]);
    pts.push([120.0, 33.0, 12.5]);
    pts.push([124.0, 222.0, 7.65]);
    pts.push([20.0, 133.0, 21.0]);
    pts.push([60.0, 60.0, 33.0]);
    let mut dt = startin::Triangulation::new();
    dt.insert(&pts, startin::InsertionStrategy::AsIs);
    println!("{}", dt);
    //-- print all the vertices
    for (i, each) in dt.all_vertices().iter().enumerate() {
        // skip the first one, the infinite vertex
        if i > 0 {
            println!("#{}: ({:.3}, {:.3}, {:.3})", i, each[0], each[1], each[2]);
        }
   }
   //-- insert a new vertex
   let re = dt.insert_one_pt(22.2, 33.3, 4.4);
   match re {
        Ok(_v) => println!(
            "Inserted new point, now the DT has {} vertices",
            dt.number_of_vertices()
        ),
        Err(v) => println!("Duplicate of vertex #{}, not inserted", v),
   }
   //-- remove it
   match dt.remove(6) {
        Ok(num) => println!("Vertex deleted, now the DT has {} vertices", num),
        Err(why) => println!("!!! Deletion error: {:?}", why),
   }
   //-- get the convex hull
   let ch = dt.convex_hull();
   println!("Convex hull: {:?}", ch);
   //-- fetch triangle containing (x, y)
   match dt.locate(50.0, 50.0) {
        Ok(tr) => println!("The triangle is {}", tr),
        Err(why) => println!("Error: {:?}", why),
   }
   //-- interpolate with Laplace interpolation at 2 locations
   let locs = vec![[51.0, 22.0], [50.3, 19.9]];
   let interpolant = startin::interpolation::Laplace {};
   let zs = startin::interpolation::interpolate(&interpolant, &mut dt, &locs);
   for z in &zs {
        match z {
            Ok(value) => println!("z = {}", value),
            Err(why) => println!("Interplation impossible: {:?}", why),
        }
   }
   //-- save the triangulation in OBJ for debug purposes
    let _re = dt.write_obj("/home/elvis/tr.obj".to_string());
}
```
