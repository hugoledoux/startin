//! # startin
//!
//! A Delaunay triangulator where the input are 2.5D points, the DT is computed in 2D but the elevation of the vertices are kept.
//! This is used mostly for the modelling of terrains.
//! Constructing a 2D Delaunay triangulation is also possible.
//!
//! The construction algorithm used is an incremental insertion based on flips, and the data structure is a cheap implementation of the star-based structure defined in [Blandford et al. (2003)](https://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.9.6823), cheap because the link of each vertex is stored a simple array (`Vec`) and not in an optimised blob like they did.
//! It results in a pretty fast library (comparison will come at some point), but it uses more space than the optimised one.
//!
//! The deletion of a vertex is also possible. The algorithm implemented is a modification of the one of [Mostafavi, Gold, and Dakowicz (2003)](https://doi.org/10.1016/S0098-3004(03)00017-7). The ears are filled by flipping, so it's in theory more robust.
//! I have also extended the algorithm to allow the deletion of vertices on the boundary of the convex hull.
//! The algorithm is sub-optimal, but in practice the number of neighbours of a given vertex in a DT is only 6, so it doesn't really matter.
//!
//! Robust arithmetic for the geometric predicates are used ([Shewchuk's predicates](https://www.cs.cmu.edu/~quake/robust.html), well the [Rust port of the code (robust crate)](https://crates.io/crates/robust)), so startin is robust and shouldn't crash (touch wood).
//!
//! There are a few interpolation functions implemented: (1) nearest-neighbour, (2) linear in TIN, (3) Laplace, (4) natural neighbour (aka Sibson's interpolation), (5) IDW.
//!
//!
//! # Web-demo with WebAssembly
//!
//! Rust can be compiled to [WebAssembly](https://www.rust-lang.org/what/wasm), and you can see a demo of some of the possibilities of startin (all computations are done locally and it's fast!).
//!
//! [--> web-demo](https://hugoledoux.github.io/startin_wasm/)
//!
//!
//! # Python bindings
//!
//! If you prefer Python, I made bindings: [https://github.com/hugoledoux/startinpy/](https://github.com/hugoledoux/startinpy/)
//!
//! There are a few more functions (eg reading GeoTIFF/LAZ, exporting GeoJSON/CityJSON) and it works with Numpy.
//!
//!
//! # C interface
//!
//! A basic C interface is available in `src/c_interface.rs`, to compile it:
//!
//! ```bash
//! cargo build --features c_api
//! ```
//!
//! # Usage
//!
//! ```rust
//! extern crate startin;
//!
//! fn main() {
//!     let mut pts: Vec<[f64; 3]> = Vec::new();
//!     pts.push([20.0, 30.0, 2.0]);
//!     pts.push([120.0, 33.0, 12.5]);
//!     pts.push([124.0, 222.0, 7.65]);
//!     pts.push([20.0, 133.0, 21.0]);
//!     pts.push([60.0, 60.0, 33.0]);
//!     let mut dt = startin::Triangulation::new();
//!     dt.insert(&pts, startin::InsertionStrategy::AsIs);
//!     println!("{}", dt);
//!     //-- print all the vertices
//!     for (i, each) in dt.all_vertices().iter().enumerate() {
//!         // skip the first one, the infinite vertex
//!         if i > 0 {
//!             println!("#{}: ({:.3}, {:.3}, {:.3})", i, each[0], each[1], each[2]);
//!         }
//!     }
//!     //-- insert a new vertex
//!     let re = dt.insert_one_pt(22.2, 33.3, 4.4);
//!     match re {
//!         Ok(_v) => println!(
//!             "Inserted new point, now the DT has {} vertices",
//!             dt.number_of_vertices()
//!         ),
//!         Err((v, _b)) => println!("Duplicate of vertex #{}, not inserted", v),
//!     }
//!     //-- remove it
//!     match dt.remove(6) {
//!         Ok(num) => println!("Vertex deleted, now the DT has {} vertices", num),
//!         Err(why) => println!("!!! Deletion error: {:?}", why),
//!     }
//!     //-- get the convex hull
//!     let ch = dt.convex_hull();
//!     println!("Convex hull: {:?}", ch);
//!     //-- fetch triangle containing (x, y)
//!     match dt.locate(50.0, 50.0) {
//!         Ok(tr) => println!("The triangle is {}", tr),
//!         Err(why) => println!("Error: {:?}", why),
//!     }
//!     //-- interpolate with Laplace interpolation at 2 locations
//!     let locs = vec![[51.0, 22.0], [50.3, 19.9]];
//!     let interpolant = startin::interpolation::Laplace {};
//!     let zs = startin::interpolation::interpolate(&interpolant, &mut dt, &locs);
//!     for z in &zs {
//!         match z {
//!             Ok(value) => println!("z = {}", value),
//!             Err(why) => println!("Interplation impossible: {:?}", why),
//!         }
//!     }
//!
//!     //-- save the triangulation in geojson for debug purposes
//!     let _re = dt.write_obj("/home/elvis/tr.obj".to_string());
//! }
//! ```

pub mod geom;
pub mod interpolation;

#[cfg(feature = "c_api")]
mod c_interface;

use rand::prelude::thread_rng;
use rand::Rng;
use serde_json::Map;

use serde_json::json;
use serde_json::Value;

use std::fmt;
use std::fs::File;
use std::io::Write;

/// Errors that arise while using startin
#[derive(Debug, PartialEq)]
pub enum StartinError {
    EmptyTriangulation,
    OutsideConvexHull,
    SearchCircleEmpty,
    TriangleNotPresent,
    VertexInfinite,
    VertexRemoved,
    VertexUnknown,
    TinHasNoAttributes,
    WrongAttribute,
}

/// Possibilities for the insertion (with `insert()`)
pub enum InsertionStrategy {
    AsIs,
    BBox,
    // Sprinkle,
    // ConBRIO,
}

/// Handling of xy-duplicates (which z do we keep?)
pub enum DuplicateHandling {
    First,
    Last,
    Highest,
    Lowest,
    // Average not possible I guess,
}
impl fmt::Display for DuplicateHandling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DuplicateHandling::First => write!(f, "First"),
            DuplicateHandling::Last => write!(f, "Last"),
            DuplicateHandling::Highest => write!(f, "Highest"),
            DuplicateHandling::Lowest => write!(f, "Lowest"),
        }
    }
}

/// A Triangle is a triplet of indices
#[derive(Debug, PartialEq, Clone)]
pub struct Triangle {
    pub v: [usize; 3],
}

impl Triangle {
    /// Checks whether a Triangle is "infinite",
    /// ie if one its vertices is the infinite vertex
    fn is_infinite(&self) -> bool {
        if self.v[0] == 0 || self.v[1] == 0 || self.v[2] == 0 {
            return true;
        }
        false
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.v[0], self.v[1], self.v[2])
    }
}

//----------------------
#[repr(C)]
#[derive(Debug, Clone)]
struct Link(Vec<usize>);

impl Link {
    fn new() -> Link {
        // Link(Vec::new())
        Link(Vec::with_capacity(8))
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn is_empty(&self) -> bool {
        self.0.len() == 0
    }
    fn add(&mut self, v: usize) {
        self.0.push(v);
    }
    fn insert_after_v(&mut self, v: usize, after: usize) {
        let pos = self.0.iter().position(|&x| x == after).unwrap();
        self.0.insert(pos + 1, v);
    }
    fn delete(&mut self, v: usize) {
        let re = self.0.iter().position(|&x| x == v);
        if re.is_some() {
            self.0.remove(re.unwrap());
        }
    }
    fn replace(&mut self, v: usize, newv: usize) {
        let re = self.0.iter().position(|&x| x == v);
        if re.is_some() {
            self.0[re.unwrap()] = newv;
            // self.0.remove(re.unwrap());
        }
    }
    fn infinite_first(&mut self) {
        let re = self.0.iter().position(|&x| x == 0);
        if re.is_some() {
            let posinf = re.unwrap();
            if posinf == 0 {
                return;
            }
            let mut newstar: Vec<usize> = Vec::new();
            for j in posinf..self.0.len() {
                newstar.push(self.0[j]);
            }
            for j in 0..posinf {
                newstar.push(self.0[j]);
            }
            // println!("newstar: {:?}", newstar);
            self.0 = newstar;
        }
    }
    fn clear(&mut self) {
        self.0.clear();
    }
    fn contains_infinite_vertex(&self) -> bool {
        let pos = self.0.iter().position(|&x| x == 0);
        pos.is_some()
    }
    fn next_index(&self, i: usize) -> usize {
        if i == (self.0.len() - 1) {
            0
        } else {
            i + 1
        }
    }
    fn prev_index(&self, i: usize) -> usize {
        if i == 0 {
            self.0.len() - 1
        } else {
            i - 1
        }
    }
    fn get_index(&self, v: usize) -> Option<usize> {
        return self.0.iter().position(|&x| x == v);
    }
    fn get_next_vertex(&self, v: usize) -> Option<usize> {
        let re = self.get_index(v);
        re?;
        let pos = re.unwrap();
        if pos == (self.0.len() - 1) {
            Some(self.0[0])
        } else {
            Some(self.0[pos + 1])
        }
    }
    fn get_prev_vertex(&self, v: usize) -> Option<usize> {
        let re = self.get_index(v);
        re?;
        let pos = re.unwrap();
        if pos == 0 {
            Some(self.0[self.0.len() - 1])
        } else {
            Some(self.0[pos - 1])
        }
    }
    fn iter(&self) -> Iter {
        Iter(Box::new(self.0.iter()))
    }
}

//-- taken from https://stackoverflow.com/questions/40668074/am-i-incorrectly-implementing-intoiterator-for-a-reference-or-is-this-a-rust-bug
struct Iter<'a>(Box<dyn Iterator<Item = &'a usize> + 'a>);

impl<'a> Iterator for Iter<'a> {
    type Item = &'a usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl std::ops::Index<usize> for Link {
    type Output = usize;
    fn index(&self, idx: usize) -> &usize {
        &self.0[idx]
    }
}

impl fmt::Display for Link {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        // fmt.write_str("pt: {}\n", self.pt)?;
        fmt.write_str(&format!("link: {:?}\n", self.0))?;
        Ok(())
    }
}

/// A triangulation is a collection of Stars, each Star has its (x,y,z)
/// and a Link (an array of adjacent vertices, ordered CCW)
#[repr(C)]
struct Star {
    pt: [f64; 3],
    link: Link,
}

impl Star {
    fn new(x: f64, y: f64, z: f64) -> Star {
        let l = Link::new();
        Star {
            pt: [x, y, z],
            link: l,
        }
    }
    fn is_deleted(&self) -> bool {
        self.link.is_empty()
    }
}

/// Represents a triangulation
#[repr(C)]
pub struct Triangulation {
    stars: Vec<Star>,
    attributes: Option<Vec<Value>>,
    attributes_schema: Vec<(String, String)>,
    snaptol: f64,
    cur: usize,
    is_init: bool,
    jump_and_walk: bool,
    robust_predicates: bool,
    removed_indices: Vec<usize>,
    duplicates_handling: DuplicateHandling,
}

impl Default for Triangulation {
    fn default() -> Self {
        Self::new()
    }
}

impl Triangulation {
    pub fn new() -> Triangulation {
        // let mut l: Vec<Star> = Vec::with_capacity(100000);
        let l: Vec<Star> = vec![Star::new(f64::INFINITY, f64::INFINITY, f64::INFINITY)];
        let es: Vec<usize> = Vec::new();
        Triangulation {
            stars: l,
            attributes: None,
            attributes_schema: Vec::new(),
            snaptol: 0.001,
            cur: 0,
            is_init: false,
            jump_and_walk: false,
            robust_predicates: true,
            removed_indices: es,
            duplicates_handling: DuplicateHandling::First,
        }
    }

    fn insert_one_pt_init_phase(&mut self, x: f64, y: f64, z: f64) -> Result<usize, (usize, bool)> {
        let p: [f64; 3] = [x, y, z];
        for i in 1..self.stars.len() {
            if geom::distance2d_squared(&self.stars[i].pt, &p) <= (self.snaptol * self.snaptol) {
                return Err((i, self.update_z_value_duplicate(i, z)));
            }
        }
        self.collect_garbage();
        //-- add point to Triangulation and create its empty star
        self.stars.push(Star::new(x, y, z));
        //-- form the first triangles (finite + infinite)
        let l = self.stars.len();
        if l >= 4 {
            let a = l - 3;
            let b = l - 2;
            let c = l - 1;
            let re = geom::orient2d(
                &self.stars[a].pt,
                &self.stars[b].pt,
                &self.stars[c].pt,
                self.robust_predicates,
            );
            if re == 1 {
                // println!("init: ({},{},{})", a, b, c);
                self.stars[0].link.add(a);
                self.stars[0].link.add(c);
                self.stars[0].link.add(b);
                self.stars[a].link.add(0);
                self.stars[a].link.add(b);
                self.stars[a].link.add(c);
                self.stars[b].link.add(0);
                self.stars[b].link.add(c);
                self.stars[b].link.add(a);
                self.stars[c].link.add(0);
                self.stars[c].link.add(a);
                self.stars[c].link.add(b);
                self.is_init = true;
            } else if re == -1 {
                // println!("init: ({},{},{})", a, c, b);
                self.stars[0].link.add(a);
                self.stars[0].link.add(b);
                self.stars[0].link.add(c);
                self.stars[a].link.add(0);
                self.stars[a].link.add(c);
                self.stars[a].link.add(b);
                self.stars[b].link.add(0);
                self.stars[b].link.add(a);
                self.stars[b].link.add(c);
                self.stars[c].link.add(0);
                self.stars[c].link.add(b);
                self.stars[c].link.add(a);
                self.is_init = true;
            }
        }
        self.cur = l - 1;
        if self.is_init {
            //-- insert the previous vertices in the dt
            for j in 1..(l - 3) {
                let tr = self.walk(&self.stars[j].pt);
                // println!("found tr: {}", tr);
                self.flip13(j, &tr);
                self.update_dt(j);
            }
        }
        match &mut self.attributes {
            Some(x) => x.push(json!({})),
            _ => (),
        };

        Ok(self.cur)
    }

    /// Set a snap tolerance when inserting new points: if the newly inserted
    /// one is closer than `snap_tol` to another one, then it is not inserted.
    /// Avoids having very close vertices (like at 0.00007mm)
    /// Default is 0.001unit (thus 1mm for most datasets).
    pub fn set_snap_tolerance(&mut self, snaptol: f64) -> f64 {
        if snaptol > 0.0 {
            self.snaptol = snaptol;
        }
        self.snaptol
    }

    pub fn get_snap_tolerance(&self) -> f64 {
        self.snaptol
    }

    /// Activate/deactive the jump-and-walk strategy for [`Triangulation::locate()`].
    /// (deactivated by default)
    /// If deactivated, then the walk starts from the last inserted triangle.
    pub fn set_jump_and_walk(&mut self, b: bool) {
        self.jump_and_walk = b;
    }

    pub fn get_jump_and_walk(&self) -> bool {
        self.jump_and_walk
    }

    /// Is using robut predicates (with [crate robust](https://docs.rs/robust))?
    /// (activated by default)
    pub fn is_using_robust_predicates(&self) -> bool {
        self.robust_predicates
    }

    /// Activate/deactivate [robust predictates](https://docs.rs/robust)
    pub fn use_robust_predicates(&mut self, b: bool) {
        self.robust_predicates = b;
    }

    /// Set the method to used to handle duplicates
    /// (Last by default)
    pub fn set_duplicates_handling(&mut self, method: DuplicateHandling) {
        self.duplicates_handling = method;
    }

    /// Get the method to used to handle duplicates
    pub fn get_duplicates_handling(&self) -> String {
        self.duplicates_handling.to_string()
    }

    /// Insert a [`Vec`] of [`array`] (`[f64; 3]`) values.
    /// If [`InsertionStrategy::AsIs`] is used, then [`Triangulation::insert_one_pt()`] is called
    /// for each point in the order given.
    ///
    /// # Arguments
    ///
    /// * `pts` - a [`Vec`] of `[f64; 3]`
    /// * `strategy` - the [`InsertionStrategy`] to use for the insertion
    pub fn insert(&mut self, pts: &Vec<[f64; 3]>, strategy: InsertionStrategy) {
        match strategy {
            InsertionStrategy::BBox => {
                //-- find the bbox
                let mut bbox = geom::bbox2d(&pts);
                //-- "padding" of the bbox to avoid conflicts
                bbox[0] -= 10.0;
                bbox[1] -= 10.0;
                bbox[2] += 10.0;
                bbox[3] += 10.0;
                self.insert_with_bbox(&pts, &bbox);
            }
            InsertionStrategy::AsIs => {
                for each in pts {
                    let _re = self.insert_one_pt(each[0], each[1], each[2]);
                }
            } // InsertionStrategy::Sprinkle => println!("Sprinkle not implemented yet"),
              // InsertionStrategy::ConBRIO => println!("ConBRIO not implemented yet"),
        }
    }

    fn insert_with_bbox(&mut self, pts: &Vec<[f64; 3]>, bbox: &[f64; 4]) {
        let mut c4: Vec<usize> = Vec::new();
        //-- insert the 4 corners
        c4.push(self.insert_one_pt(bbox[0], bbox[1], 0.0).unwrap());
        c4.push(self.insert_one_pt(bbox[2], bbox[1], 0.0).unwrap());
        c4.push(self.insert_one_pt(bbox[2], bbox[3], 0.0).unwrap());
        c4.push(self.insert_one_pt(bbox[0], bbox[3], 0.0).unwrap());
        for each in pts {
            let _re = self.insert_one_pt(each[0], each[1], each[2]);
        }
        //-- remove the 4 corners
        for each in &c4 {
            let _re = self.remove(*each);
        }
        //-- collect garbage: remove the 4 added vertices and "shift" all the vertex ids
        self.collect_garbage();
    }

    /// Insert the point (`px`, `py`, `pz`) in the triangulation.
    /// Returns the vertex ID of the point if the vertex didn't exist.
    /// If there was a vertex at that location, an Error is thrown with a tuple
    /// indicating 1) the vertex ID of the existing vertex; 2) true/false whether the
    /// z-value and attributes were updated.
    pub fn insert_one_pt(&mut self, px: f64, py: f64, pz: f64) -> Result<usize, (usize, bool)> {
        if !self.is_init {
            return self.insert_one_pt_init_phase(px, py, pz);
        }
        //-- walk
        let p: [f64; 3] = [px, py, pz];
        let tr = self.walk(&p);
        if geom::distance2d_squared(&self.stars[tr.v[0]].pt, &p) <= (self.snaptol * self.snaptol) {
            return Err((tr.v[0], self.update_z_value_duplicate(tr.v[0], pz)));
        }
        if geom::distance2d_squared(&self.stars[tr.v[1]].pt, &p) <= (self.snaptol * self.snaptol) {
            return Err((tr.v[1], self.update_z_value_duplicate(tr.v[1], pz)));
        }
        if geom::distance2d_squared(&self.stars[tr.v[2]].pt, &p) <= (self.snaptol * self.snaptol) {
            return Err((tr.v[2], self.update_z_value_duplicate(tr.v[2], pz)));
        }
        //-- ok we now insert the point in the data structure
        let pi: usize;
        if self.removed_indices.is_empty() {
            self.stars.push(Star::new(px, py, pz));
            pi = self.stars.len() - 1;
        } else {
            // self.stars.push(Star::new(px, py, pz));
            pi = self.removed_indices.pop().unwrap();
            self.stars[pi].pt[0] = px;
            self.stars[pi].pt[1] = py;
            self.stars[pi].pt[2] = pz;
        }
        //-- flip13()
        self.flip13(pi, &tr);
        //-- update_dt()
        self.update_dt(pi);
        self.cur = pi;
        //-- extra attributes
        match &mut self.attributes {
            Some(x) => x.push(json!({})),
            _ => (),
        }
        Ok(pi)
    }

    fn update_z_value_duplicate(&mut self, vi: usize, newz: f64) -> bool {
        let mut re = false;
        match self.duplicates_handling {
            DuplicateHandling::Last => {
                self.stars[vi].pt[2] = newz;
                re = true;
            }
            DuplicateHandling::Highest => {
                if newz > self.stars[vi].pt[2] {
                    self.stars[vi].pt[2] = newz;
                    re = true;
                }
            }
            DuplicateHandling::Lowest => {
                if newz < self.stars[vi].pt[2] {
                    self.stars[vi].pt[2] = newz;
                    re = true;
                }
            }
            DuplicateHandling::First => (),
        }
        re
    }

    fn update_dt(&mut self, pi: usize) {
        // println!("--> Update DT");
        let mut mystack: Vec<Triangle> = Vec::new();
        mystack.push(Triangle {
            v: [pi, self.stars[pi].link[0], self.stars[pi].link[1]],
        });
        mystack.push(Triangle {
            v: [pi, self.stars[pi].link[1], self.stars[pi].link[2]],
        });
        mystack.push(Triangle {
            v: [pi, self.stars[pi].link[2], self.stars[pi].link[0]],
        });

        loop {
            let tr = match mystack.pop() {
                None => break,
                Some(x) => x,
            };
            let opposite = self.get_opposite_vertex(&tr);
            // println!("stacked: {} {}", tr, opposite);

            if tr.is_infinite() {
                let mut a: i8 = 0;
                if tr.v[0] == 0 {
                    a = geom::orient2d(
                        &self.stars[opposite].pt,
                        &self.stars[tr.v[1]].pt,
                        &self.stars[tr.v[2]].pt,
                        self.robust_predicates,
                    );
                } else if tr.v[1] == 0 {
                    a = geom::orient2d(
                        &self.stars[tr.v[0]].pt,
                        &self.stars[opposite].pt,
                        &self.stars[tr.v[2]].pt,
                        self.robust_predicates,
                    );
                } else if tr.v[2] == 0 {
                    a = geom::orient2d(
                        &self.stars[tr.v[0]].pt,
                        &self.stars[tr.v[1]].pt,
                        &self.stars[opposite].pt,
                        self.robust_predicates,
                    );
                }
                // println!("TODO: INCIRCLE FOR INFINITY {}", a);
                if a > 0 {
                    // println!("FLIPPED0 {} {}", tr, opposite);
                    let (ret0, ret1) = self.flip22(&tr, opposite);
                    mystack.push(ret0);
                    mystack.push(ret1);
                }
            } else {
                if opposite == 0 {
                    //- if insertion on CH then break the edge, otherwise do nothing
                    //-- TODO sure the flips are okay here?
                    if geom::orient2d(
                        &self.stars[tr.v[0]].pt,
                        &self.stars[tr.v[1]].pt,
                        &self.stars[tr.v[2]].pt,
                        self.robust_predicates,
                    ) == 0
                    {
                        // println!("FLIPPED1 {} {}", tr, 0);
                        let (ret0, ret1) = self.flip22(&tr, 0);
                        mystack.push(ret0);
                        mystack.push(ret1);
                    }
                } else {
                    if geom::incircle(
                        &self.stars[tr.v[0]].pt,
                        &self.stars[tr.v[1]].pt,
                        &self.stars[tr.v[2]].pt,
                        &self.stars[opposite].pt,
                        self.robust_predicates,
                    ) > 0
                    {
                        // println!("FLIPPED2 {} {}", tr, opposite);
                        let (ret0, ret1) = self.flip22(&tr, opposite);
                        mystack.push(ret0);
                        mystack.push(ret1);
                    }
                }
            }
        }
    }

    fn flip13(&mut self, pi: usize, tr: &Triangle) {
        self.stars[pi].link.add(tr.v[0]);
        self.stars[pi].link.add(tr.v[1]);
        self.stars[pi].link.add(tr.v[2]);
        self.stars[tr.v[0]].link.insert_after_v(pi, tr.v[1]);
        self.stars[tr.v[1]].link.insert_after_v(pi, tr.v[2]);
        self.stars[tr.v[2]].link.insert_after_v(pi, tr.v[0]);
        //-- put infinite vertex first in list
        self.stars[pi].link.infinite_first();
    }

    fn flip31(&mut self, vi: usize) {
        // println!("FLIP31");
        let mut ns: Vec<usize> = Vec::new();
        for each in self.stars[vi].link.iter() {
            ns.push(*each);
        }
        for n in ns.iter() {
            self.stars[*n].link.delete(vi);
        }
        self.stars[vi].link.clear();
        self.stars[vi].pt[0] = f64::NAN;
        self.stars[vi].pt[1] = f64::NAN;
        self.stars[vi].pt[2] = f64::NAN;
        self.removed_indices.push(vi);
        if self.attributes.is_some() {
            let _ = self.reset_vertex_attributes(vi);
        }
        if ns[0] != 0 {
            self.cur = ns[0];
        } else if ns[1] != 0 {
            self.cur = ns[1];
        } else if ns[2] != 0 {
            self.cur = ns[2];
        }
    }

    /// Returns the coordinates of the vertex v.
    /// A [`StartinError`] is returned if `vi` doesn't exist
    /// or is a removed vertex.
    pub fn get_point(&self, vi: usize) -> Result<Vec<f64>, StartinError> {
        match self.is_vertex_removed(vi) {
            Err(why) => Err(why),
            Ok(b) => match b {
                true => Err(StartinError::VertexRemoved),
                false => Ok(self.stars[vi].pt.to_vec()),
            },
        }
    }

    /// TODO: write the docs
    pub fn get_attributes_schema(&self) -> Vec<(String, String)> {
        self.attributes_schema.clone()
    }

    /// Configure the extra attributes that each vertex can store.
    /// Each entry is a name (eg "classification") and a data type
    /// (the allowed types are: "f64", "i64", "u64", "bool", and "String" (given as a String)).
    /// This resets all the extra attributes that were potentially stored with a previous schema.
    pub fn set_attributes_schema(
        &mut self,
        att_schema: Vec<(String, String)>,
    ) -> Result<(), StartinError> {
        let dtypes_allowed: Vec<String> = vec![
            "f64".to_string(),
            "i64".to_string(),
            "u64".to_string(),
            "bool".to_string(),
            "String".to_string(),
        ];
        for each in &att_schema {
            if dtypes_allowed.iter().any(|e| *e == *each.1) {
                self.attributes_schema
                    .push((each.0.clone(), each.1.clone()));
            } else {
                return Err(StartinError::WrongAttribute);
            }
        }
        //-- reset all the extra attributes
        self.attributes = Some(vec![json!({}); self.stars.len()]);
        Ok(())
    }

    pub fn get_vertex_attributes(&self, vi: usize) -> Result<Value, StartinError> {
        match self.is_vertex_removed(vi) {
            Err(why) => Err(why),
            Ok(b) => match b {
                true => Err(StartinError::VertexRemoved),
                false => match &self.attributes {
                    Some(x) => Ok(x.get(vi).unwrap().clone()),
                    None => Err(StartinError::TinHasNoAttributes),
                },
            },
        }
    }

    fn reset_vertex_attributes(&mut self, vi: usize) -> Result<bool, StartinError> {
        match self.is_vertex_removed(vi) {
            Err(why) => Err(why),
            Ok(_b) => match &mut self.attributes {
                Some(x) => {
                    *x.get_mut(vi).unwrap() = json!({});
                    return Ok(true);
                }
                None => {
                    return Err(StartinError::TinHasNoAttributes);
                }
            },
        }
    }

    pub fn add_vertex_attributes(&mut self, vi: usize, a: Value) -> Result<bool, StartinError> {
        match self.is_vertex_removed(vi) {
            Err(why) => Err(why),
            Ok(_b) => match &mut self.attributes {
                Some(x) => {
                    if a.is_object() == false {
                        return Ok(false);
                    }
                    let mut a2: Map<String, Value> = Map::new();
                    let a1 = a.as_object().unwrap();
                    for (p, v2) in a1 {
                        let c = self
                            .attributes_schema
                            .iter()
                            .position(|(first, _)| first == p);
                        if c.is_some() {
                            match self.attributes_schema.get(c.unwrap()).unwrap().1.as_ref() {
                                "f64" => {
                                    if v2.is_number() {
                                        a2.insert(p.to_string(), v2.clone());
                                    }
                                }
                                "i64" => {
                                    if v2.is_i64() {
                                        a2.insert(p.to_string(), v2.clone());
                                    }
                                }
                                "u64" => {
                                    if v2.is_u64() {
                                        a2.insert(p.to_string(), v2.clone());
                                    }
                                }
                                "String" => {
                                    if v2.is_string() {
                                        a2.insert(p.to_string(), v2.clone());
                                    }
                                }
                                "bool" => {
                                    if v2.is_boolean() {
                                        a2.insert(p.to_string(), v2.clone());
                                    }
                                }
                                _ => continue,
                            }
                        }
                    }
                    let v = x.get_mut(vi).unwrap();
                    let vo = v.as_object_mut().unwrap();
                    vo.append(&mut a2);
                    Ok(true)
                }
                None => {
                    return Err(StartinError::TinHasNoAttributes);
                }
            },
        }
    }

    /// Returns a [`Vec`]<[`Vec`]<[`Value`]>> of all the vertex attributes
    /// (including the infinite one and the removed ones)
    pub fn all_attributes(&self) -> Option<Vec<Value>> {
        self.attributes.clone()
    }

    /// Returns the 3 adjacents (finite + infinite) [`Triangle`] to a triangle.
    pub fn adjacent_triangles_to_triangle(
        &self,
        tr: &Triangle,
    ) -> Result<Vec<Triangle>, StartinError> {
        if !self.is_triangle(&tr) || tr.is_infinite() {
            return Err(StartinError::TriangleNotPresent);
        }
        let mut trs: Vec<Triangle> = Vec::new();
        let mut opp = self.stars[tr.v[2]].link.get_next_vertex(tr.v[1]).unwrap();
        trs.push(Triangle {
            v: [tr.v[1], opp, tr.v[2]],
        });
        opp = self.stars[tr.v[0]].link.get_next_vertex(tr.v[2]).unwrap();
        trs.push(Triangle {
            v: [tr.v[2], opp, tr.v[0]],
        });
        opp = self.stars[tr.v[1]].link.get_next_vertex(tr.v[0]).unwrap();
        trs.push(Triangle {
            v: [tr.v[0], opp, tr.v[1]],
        });
        Ok(trs)
    }

    /// Returns a [`Vec`] of [`Triangle`]s (finite + infinite) to the vertex `vi`.
    pub fn incident_triangles_to_vertex(&self, vi: usize) -> Result<Vec<Triangle>, StartinError> {
        match self.is_vertex_removed(vi) {
            Err(why) => Err(why),
            Ok(b) => match b {
                true => Err(StartinError::VertexRemoved),
                false => {
                    let mut trs: Vec<Triangle> = Vec::new();
                    for (i, each) in self.stars[vi].link.iter().enumerate() {
                        let j = self.stars[vi].link.next_index(i);
                        trs.push(Triangle {
                            v: [vi, *each, self.stars[vi].link[j]],
                        });
                    }
                    Ok(trs)
                }
            },
        }
    }

    /// Returns the normal of the vertex with ID `vi`.
    pub fn normal_vertex(&self, vi: usize) -> Result<Vec<f64>, StartinError> {
        match self.incident_triangles_to_vertex(vi) {
            Err(why) => Err(why),
            Ok(trs) => {
                let mut avgns: Vec<f64> = vec![0.0, 0.0, 0.0];
                let mut no_finite_tr = 0;
                for tr in &trs {
                    // println!("{:?} {:?}", tr, self.is_finite(tr));
                    if self.is_finite(tr) == false {
                        continue;
                    }
                    no_finite_tr += 1;
                    let n = geom::normal_triangle(
                        &self.stars[tr.v[0]].pt,
                        &self.stars[tr.v[1]].pt,
                        &self.stars[tr.v[2]].pt,
                        true,
                    );
                    for j in 0..3 {
                        avgns[j] += n[j];
                    }
                }
                for j in 0..3 {
                    avgns[j] /= no_finite_tr as f64;
                }
                let norm =
                    ((avgns[0] * avgns[0]) + (avgns[1] * avgns[1]) + (avgns[2] * avgns[2])).sqrt();
                Ok(vec![avgns[0] / norm, avgns[1] / norm, avgns[2] / norm])
            }
        }
    }

    pub fn normal_triangle(&self, tr: &Triangle) -> Result<Vec<f64>, StartinError> {
        match self.is_triangle(tr) {
            false => Err(StartinError::TriangleNotPresent),
            true => Ok(geom::normal_triangle(
                &self.stars[tr.v[0]].pt,
                &self.stars[tr.v[1]].pt,
                &self.stars[tr.v[2]].pt,
                true,
            )),
        }
    }

    pub fn area2d_triangle(&self, tr: &Triangle) -> Result<f64, StartinError> {
        match self.is_triangle(tr) {
            false => Err(StartinError::TriangleNotPresent),
            true => Ok(geom::area2d_triangle(
                &self.stars[tr.v[0]].pt,
                &self.stars[tr.v[1]].pt,
                &self.stars[tr.v[2]].pt,
            )),
        }
    }

    pub fn area3d_triangle(&self, tr: &Triangle) -> Result<f64, StartinError> {
        match self.is_triangle(tr) {
            false => Err(StartinError::TriangleNotPresent),
            true => Ok(geom::area3d_triangle(
                &self.stars[tr.v[0]].pt,
                &self.stars[tr.v[1]].pt,
                &self.stars[tr.v[2]].pt,
            )),
        }
    }

    /// Returns the degree of the vertex with ID `vi`.
    pub fn degree(&self, vi: usize) -> Result<usize, StartinError> {
        match self.is_vertex_removed(vi) {
            Err(why) => Err(why),
            Ok(b) => match b {
                true => Err(StartinError::VertexRemoved),
                false => Ok(self.stars[vi].link.len()),
            },
        }
    }

    /// Returns a list (`Vec<usize>`) (ordered CCW) of the adjacent vertices to `vi`.
    pub fn adjacent_vertices_to_vertex(&self, vi: usize) -> Result<Vec<usize>, StartinError> {
        match self.is_vertex_removed(vi) {
            Err(why) => Err(why),
            Ok(b) => match b {
                true => Err(StartinError::VertexRemoved),
                false => {
                    let mut adjs: Vec<usize> = Vec::new();
                    for each in self.stars[vi].link.iter() {
                        adjs.push(*each);
                    }
                    Ok(adjs)
                }
            },
        }
    }

    /// Returns whether a triplet of indices is a [`Triangle`] in the triangulation.
    pub fn is_triangle(&self, tr: &Triangle) -> bool {
        if tr.v[0] >= self.stars.len() || tr.v[1] >= self.stars.len() || tr.v[2] >= self.stars.len()
        {
            return false;
        }
        let re = self.stars[tr.v[0]].link.get_next_vertex(tr.v[1]);
        if re.is_none() {
            return false;
        } else {
            if re.unwrap() == tr.v[2] {
                return true;
            } else {
                return false;
            }
        }
    }

    /// Returns whether a [`Triangle`] is finite, or not
    pub fn is_finite(&self, tr: &Triangle) -> bool {
        return !tr.is_infinite();
    }

    /// Returns some statistics about the triangulation.
    pub fn statistics_degree(&self) -> (f64, usize, usize) {
        let mut total: f64 = 0.0;
        let mut min: usize = usize::max_value();
        let mut max: usize = usize::min_value();
        for i in 1..self.stars.len() {
            total += self.stars[i].link.len() as f64;
            if self.stars[i].link.len() > max {
                max = self.stars[i].link.len();
            }
            if self.stars[i].link.len() < min {
                min = self.stars[i].link.len();
            }
        }
        total /= (self.stars.len() - 2) as f64;
        (total, min, max)
    }

    /// Returns number of finite vertices in the triangulation.
    /// The removed vertices are not counted.
    pub fn number_of_vertices(&self) -> usize {
        //-- number of finite vertices
        self.stars.len() - 1 - self.removed_indices.len()
    }

    /// Returns number of finite triangles in the triangulation.
    pub fn number_of_triangles(&self) -> usize {
        //-- number of finite triangles
        let mut count: usize = 0;
        for (i, star) in self.stars.iter().enumerate() {
            for (j, value) in star.link.iter().enumerate() {
                if i < *value {
                    let k = star.link[star.link.next_index(j)];
                    if i < k {
                        let tr = Triangle { v: [i, *value, k] };
                        if !tr.is_infinite() {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    /// Returns the number of vertices which are marked as "removed"
    pub fn number_of_removed_vertices(&self) -> usize {
        self.removed_indices.len()
    }

    /// Returns whether the vertex `vi` is removed or not.
    pub fn is_vertex_removed(&self, vi: usize) -> Result<bool, StartinError> {
        if vi >= self.stars.len() {
            return Err(StartinError::VertexUnknown);
        }
        Ok(self.stars[vi].is_deleted())
    }

    /// Returns the convex hull of the dataset, oriented CCW.
    /// It is a list of vertex indices (first != last)
    pub fn convex_hull(&self) -> Vec<usize> {
        let mut re: Vec<usize> = Vec::new();
        for x in self.stars[0].link.iter() {
            re.push(*x);
        }
        re.reverse();
        re
    }

    /// Returns the size (ie the number of vertices) of the convex hull of the dataset
    pub fn number_of_vertices_on_convex_hull(&self) -> usize {
        //-- number of finite vertices on the boundary of the convex hull
        if !self.is_init {
            return 0;
        }
        self.stars[0].link.len()
    }

    /// Returns `true` if the vertex `vi` is part of the boundary of the convex
    /// hull of the dataset; `false` otherwise.
    pub fn is_vertex_convex_hull(&self, vi: usize) -> bool {
        if vi == 0 {
            return false;
        }
        if !self.is_vertex_valid(vi) {
            return false;
        }
        self.stars[vi].link.contains_infinite_vertex()
    }

    /// Returns, if it exists, the [`Triangle`] containing `(px, py)`.
    /// If it is direction on a vertex/edge, then one is randomly chosen.
    pub fn locate(&mut self, px: f64, py: f64) -> Result<Triangle, StartinError> {
        if !self.is_init {
            return Err(StartinError::EmptyTriangulation);
        }
        let p: [f64; 3] = [px, py, 0.0];
        let re = self.walk(&p);
        match re.is_infinite() {
            true => Err(StartinError::OutsideConvexHull),
            false => {
                self.cur = re.v[0];
                return Ok(re);
            }
        }
    }

    /// Returns closest point (in 2D) to a query point `(x, y)`.
    /// if `(px, py)` is outside the convex hull then [`StartinError::OutsideConvexHull`] is raised.
    pub fn closest_point(&mut self, px: f64, py: f64) -> Result<usize, StartinError> {
        let re = self.locate(px, py);
        if re.is_err() {
            return Err(re.err().unwrap());
        }
        let p: [f64; 3] = [px, py, 0.0];
        let tr = re.unwrap();
        let mut d = std::f64::MAX;
        let mut closest: usize = 0;
        //-- find closest vertex in the triangle containing p
        for each in tr.v.iter() {
            let dtmp = geom::distance2d_squared(&self.stars[*each].pt, &p);
            if dtmp < d {
                d = dtmp;
                closest = *each;
            }
        }
        loop {
            let mut found_one_closer = false;
            for each in self.stars[closest].link.iter() {
                let dtmp = geom::distance2d_squared(&self.stars[*each].pt, &p);
                if dtmp < d {
                    d = dtmp;
                    closest = *each;
                    found_one_closer = true;
                    break;
                }
            }
            if !found_one_closer {
                break;
            }
        }
        Ok(closest)
    }

    fn walk(&self, x: &[f64]) -> Triangle {
        //-- find the starting tr
        let mut cur = self.cur;
        //-- jump-and-walk
        if self.jump_and_walk {
            let mut rng = thread_rng();
            let mut d: f64 = geom::distance2d_squared(&self.stars[self.cur].pt, x);
            let n = (self.stars.len() as f64).powf(0.25);
            // let n = (self.stars.len() as f64).powf(0.25) * 7.0;
            for _i in 0..n as i32 {
                let re: usize = rng.gen_range(1, self.stars.len());
                // let dtemp = x.square_2d_distance(&self.stars[re].pt);
                if self.stars[re].is_deleted() {
                    continue;
                }
                let dtemp = geom::distance2d_squared(&self.stars[re].pt, x);
                if dtemp < d {
                    cur = re;
                    d = dtemp;
                }
            }
        }
        let mut tr = Triangle { v: [0, 0, 0] };
        // println!("cur: {}", cur);

        //-- 1. find a finite triangle
        tr.v[0] = cur;
        let l = &self.stars[cur].link;
        for i in 0..(l.len() - 1) {
            if (l[i] != 0) && (l[i + 1] != 0) {
                tr.v[1] = l[i];
                tr.v[2] = l[i + 1];
                break;
            }
        }
        //-- 2. order it such that tr0-tr1-x is CCW
        if geom::orient2d(
            &self.stars[tr.v[0]].pt,
            &self.stars[tr.v[1]].pt,
            x,
            self.robust_predicates,
        ) == -1
        {
            if geom::orient2d(
                &self.stars[tr.v[1]].pt,
                &self.stars[tr.v[2]].pt,
                x,
                self.robust_predicates,
            ) != -1
            {
                let tmp: usize = tr.v[0];
                tr.v[0] = tr.v[1];
                tr.v[1] = tr.v[2];
                tr.v[2] = tmp;
            } else {
                let tmp: usize = tr.v[1];
                tr.v[1] = tr.v[0];
                tr.v[0] = tr.v[2];
                tr.v[2] = tmp;
            }
        }
        //-- 3. start the walk
        //-- we know that tr0-tr1-x is CCW
        loop {
            if tr.is_infinite() {
                break;
            }
            if geom::orient2d(
                &self.stars[tr.v[1]].pt,
                &self.stars[tr.v[2]].pt,
                x,
                self.robust_predicates,
            ) != -1
            {
                if geom::orient2d(
                    &self.stars[tr.v[2]].pt,
                    &self.stars[tr.v[0]].pt,
                    x,
                    self.robust_predicates,
                ) != -1
                {
                    break;
                } else {
                    //-- walk to incident to tr1,tr2
                    // println!("here");
                    let prev = self.stars[tr.v[2]].link.get_prev_vertex(tr.v[0]).unwrap();
                    tr.v[1] = tr.v[2];
                    tr.v[2] = prev;
                }
            } else {
                //-- walk to incident to tr1,tr2
                // a.iter().position(|&x| x == 2), Some(1)
                let prev = self.stars[tr.v[1]].link.get_prev_vertex(tr.v[2]).unwrap();
                tr.v[0] = tr.v[2];
                tr.v[2] = prev;
            }
        }
        tr
    }

    fn flip22(&mut self, tr: &Triangle, opposite: usize) -> (Triangle, Triangle) {
        //-- step 1.
        self.stars[tr.v[0]].link.insert_after_v(opposite, tr.v[1]);
        //-- step 2.
        self.stars[tr.v[1]].link.delete(tr.v[2]);
        //-- step 3.
        self.stars[opposite].link.insert_after_v(tr.v[0], tr.v[2]);
        //-- step 4.
        self.stars[tr.v[2]].link.delete(tr.v[1]);
        //-- make 2 triangles to return (to stack)
        let ret0 = Triangle {
            v: [tr.v[0], tr.v[1], opposite],
        };
        let ret1 = Triangle {
            v: [tr.v[0], opposite, tr.v[2]],
        };
        (ret0, ret1)
    }

    fn get_opposite_vertex(&self, tr: &Triangle) -> usize {
        self.stars[tr.v[2]].link.get_next_vertex(tr.v[1]).unwrap()
    }

    /// Returns a [`Vec`]<[`Vec`]<[`f64`]>> of all the vertices
    /// (including the infinite one and the removed ones)
    pub fn all_vertices(&self) -> Vec<Vec<f64>> {
        let mut pts: Vec<Vec<f64>> = Vec::with_capacity(self.stars.len() - 1);
        for i in 0..self.stars.len() {
            pts.push(self.stars[i].pt.to_vec());
        }
        pts
    }

    /// Returns a [`Vec`]<[`usize`]> of all the finite edges (implicitly grouped by 2)
    pub fn all_finite_edges(&self) -> Vec<usize> {
        let mut edges: Vec<usize> = Vec::new();
        for i in 1..self.stars.len() {
            for value in self.stars[i].link.iter() {
                if (*value != 0) && (i < *value) {
                    edges.push(i);
                    edges.push(*value);
                }
            }
        }
        edges
    }

    /// Returns a [`Vec`]<[`Triangle`]> of all the (finite + infinite) triangles
    pub fn all_triangles(&self) -> Vec<Triangle> {
        let mut trs: Vec<Triangle> = Vec::new();
        for (i, star) in self.stars.iter().enumerate() {
            //-- reconstruct triangles
            for (j, value) in star.link.iter().enumerate() {
                if i < *value {
                    // let k = star.l[self.nexti(star.link.len(), j)];
                    let k = star.link[star.link.next_index(j)];
                    if i < k {
                        trs.push(Triangle { v: [i, *value, k] });
                    }
                }
            }
        }
        trs
    }

    /// Returns a [`Vec`]<[`Triangle`]> of all the finite triangles
    pub fn all_finite_triangles(&self) -> Vec<Triangle> {
        let alltrs = self.all_triangles();
        let mut re: Vec<Triangle> = Vec::new();
        for t in &alltrs {
            if !t.is_infinite() {
                re.push(t.clone());
            }
        }
        re
    }

    /// Validates the Delaunay triangulation:
    /// (1) checks each triangle against each vertex (circumcircle tests); very slow
    /// (2) checks whether the convex hull is really convex
    pub fn is_valid(&self) -> bool {
        self.is_valid_ch_convex() && self.is_valid_circumcircle()
    }

    fn is_valid_circumcircle(&self) -> bool {
        let mut re = true;
        let trs = self.all_finite_triangles();
        for tr in trs.iter() {
            for i in 1..self.stars.len() {
                if !self.stars[i].is_deleted()
                    && geom::incircle(
                        &self.stars[tr.v[0]].pt,
                        &self.stars[tr.v[1]].pt,
                        &self.stars[tr.v[2]].pt,
                        &self.stars[i].pt,
                        self.robust_predicates,
                    ) > 0
                {
                    println!("NOT DELAUNAY FFS!");
                    println!("{} with {}", tr, i);
                    re = false
                }
            }
        }
        re
    }

    fn is_valid_ch_convex(&self) -> bool {
        let mut re = true;
        let ch = self.convex_hull();
        for i in 0..ch.len() {
            if geom::orient2d(
                &self.stars[ch[i % ch.len()]].pt,
                &self.stars[ch[(i + 1) % ch.len()]].pt,
                &self.stars[ch[(i + 2) % ch.len()]].pt,
                self.robust_predicates,
            ) == -1
            {
                re = false;
                break;
            }
        }
        if !re {
            println!("CONVEX NOT CONVEX");
        }
        re
    }

    fn remove_on_convex_hull(&mut self, v: usize) -> Result<usize, StartinError> {
        // println!("!!! REMOVE ON CONVEX HULL");
        let mut adjs: Vec<usize> = Vec::new();
        //-- necessary because assumptions below for start-end line on CH
        self.stars[v].link.infinite_first();
        for each in self.stars[v].link.iter() {
            adjs.push(*each);
        }
        // println!("adjs: {:?}", adjs);
        let mut cur: usize = 0;
        //-- 1. find and create finite triangles only
        let mut nadjs = adjs.len();
        let mut steps = 0;
        while adjs.len() > 3 {
            //-- control the loops to avoid infinite loop, when all options in a temp
            //-- star have been tried it's because we're stuck (and done actually)
            if steps == nadjs {
                break;
            }
            if adjs.len() == nadjs {
                steps += 1;
            } else {
                nadjs = adjs.len();
                steps = 0;
            }
            //-- define the ear
            let a = cur % adjs.len();
            let b = (cur + 1) % adjs.len();
            let c = (cur + 2) % adjs.len();
            // println!("cur ear--> {:?} {}/{}/{}", adjs, a, b, c);
            if adjs[a] == 0 || adjs[b] == 0 || adjs[c] == 0 {
                //-- do not process infinite ear
                cur += 1;
                continue;
            }
            if (geom::orient2d(
                &self.stars[adjs[a]].pt,
                &self.stars[adjs[b]].pt,
                &self.stars[adjs[c]].pt,
                self.robust_predicates,
            ) == 1)
                && (geom::orient2d(
                    &self.stars[adjs[a]].pt,
                    &self.stars[adjs[c]].pt,
                    &self.stars[v].pt,
                    self.robust_predicates,
                ) >= 0)
            {
                // println!("ear {}-{}-{}", adjs[a], adjs[b], adjs[c]);
                //-- test incircle with all other vertices in the "hole"
                let cur2 = cur + 3;
                let mut isdel = true;
                for i in 0..adjs.len() - 3 {
                    // println!("test ear with {}", adjs[(cur2 + i) % adjs.len()]);
                    if adjs[(cur2 + i) % adjs.len()] != 0
                        && geom::incircle(
                            &self.stars[adjs[a]].pt,
                            &self.stars[adjs[b]].pt,
                            &self.stars[adjs[c]].pt,
                            &self.stars[adjs[(cur2 + i) % adjs.len()]].pt,
                            self.robust_predicates,
                        ) > 0
                    {
                        isdel = false;
                        break;
                    }
                }
                if isdel {
                    // println!("flip22");
                    let t = Triangle {
                        v: [adjs[a], adjs[b], v],
                    };
                    self.flip22(&t, adjs[c]);
                    adjs.remove((cur + 1) % adjs.len());
                }
            }
            cur += 1;
        }
        //-- flip31 to remove the vertex
        if adjs.len() == 3 {
            self.flip31(v);
            if self.number_of_vertices() < 3 {
                //-- going back to a line, no triangles
                //-- wipe it all and start the insert_init_phase again
                for i in 0..self.stars.len() {
                    self.stars[i].link.clear();
                }
                self.stars[v].pt[0] = f64::NAN;
                self.stars[v].pt[1] = f64::NAN;
                self.stars[v].pt[2] = f64::NAN;
                // self.removed_indices.push(v);
                self.is_init = false;
            }
            Ok(self.stars.len() - 1)
        } else {
            //-- convex part is filled, and we need to apply a special "flip"
            //-- to delete the vertex v and its incident edges
            // println!("FLIP-FOR-CH");
            self.stars[adjs[1]].link.delete(v);
            self.stars[*(adjs.last().unwrap())].link.delete(v);
            for i in 2..(adjs.len() - 1) {
                if self.is_vertex_convex_hull(adjs[i]) {
                    //-- going back to a line, no triangles
                    //-- wipe it all and start the insert_init_phase again
                    for i in 0..self.stars.len() {
                        self.stars[i].link.clear();
                    }
                    self.stars[v].pt[0] = f64::NAN;
                    self.stars[v].pt[1] = f64::NAN;
                    self.stars[v].pt[2] = f64::NAN;
                    self.removed_indices.push(v);
                    self.is_init = false;
                    return Ok(self.stars.len() - 1);
                }
                self.stars[adjs[i]].link.replace(v, 0);
                self.stars[adjs[i]].link.infinite_first();
            }
            let mut prev = v;
            for i in 2..(adjs.len() - 1) {
                self.stars[0].link.insert_after_v(adjs[i], prev);
                prev = adjs[i];
            }
            self.stars[adjs[0]].link.delete(v);
            self.stars[v].link.clear();
            self.stars[v].pt[0] = f64::NAN;
            self.stars[v].pt[1] = f64::NAN;
            self.stars[v].pt[2] = f64::NAN;
            self.removed_indices.push(v);
            for i in 0..1000 {
                if adjs[i] != 0 {
                    self.cur = adjs[i];
                    break;
                }
            }
            Ok(self.stars.len() - 1)
        }
    }

    /// Removes the vertex `vi` from the [`Triangulation`] and updates for the "Delaunay-ness".
    ///
    /// The vertex is not removed from memory but flagged as removed, thus all the other vertices
    /// keep their IDs.
    /// The following insertion of a point will reuse this ID.
    /// It is therefore possible to have an array that contains unused/removed vertices.
    pub fn remove(&mut self, vi: usize) -> Result<usize, StartinError> {
        // println!("REMOVE vertex {}", v);
        if vi == 0 {
            return Err(StartinError::VertexInfinite);
        }
        if !self.is_init {
            self.stars[vi].pt[0] = f64::NAN;
            self.stars[vi].pt[1] = f64::NAN;
            self.stars[vi].pt[2] = f64::NAN;
            self.removed_indices.push(vi);
            if self.attributes.is_some() {
                let _ = self.reset_vertex_attributes(vi);
            }
        }
        match self.is_vertex_removed(vi) {
            Err(why) => return Err(why),
            Ok(b) => {
                if b {
                    return Err(StartinError::VertexRemoved);
                }
            }
        }
        if self.is_vertex_convex_hull(vi) {
            return self.remove_on_convex_hull(vi);
        }
        let mut adjs: Vec<usize> = Vec::new();
        for each in self.stars[vi].link.iter() {
            adjs.push(*each);
        }
        // println!("adjs: {:?}", adjs);
        let mut cur: usize = 0;
        while adjs.len() > 3 {
            let a = cur % adjs.len();
            let b = (cur + 1) % adjs.len();
            let c = (cur + 2) % adjs.len();
            // println!("cur ear--> {:?} {}/{}/{}", adjs, a, b, c);
            if (geom::orient2d(
                &self.stars[adjs[a]].pt,
                &self.stars[adjs[b]].pt,
                &self.stars[adjs[c]].pt,
                self.robust_predicates,
            ) == 1)
                && (geom::orient2d(
                    &self.stars[adjs[a]].pt,
                    &self.stars[adjs[c]].pt,
                    &self.stars[vi].pt,
                    self.robust_predicates,
                ) >= 0)
            {
                // println!("ear {}-{}-{}", adjs[a], adjs[b], adjs[c]);
                //-- test incircle with all other vertices in the "hole"
                let cur2 = cur + 3;
                let mut isdel = true;
                for i in 0..adjs.len() - 3 {
                    // println!("test ear with {}", adjs[(cur2 + i) % adjs.len()]);
                    if geom::incircle(
                        &self.stars[adjs[a]].pt,
                        &self.stars[adjs[b]].pt,
                        &self.stars[adjs[c]].pt,
                        &self.stars[adjs[(cur2 + i) % adjs.len()]].pt,
                        self.robust_predicates,
                    ) > 0
                    {
                        isdel = false;
                        break;
                    }
                }
                if isdel {
                    // println!("flip22");
                    let t = Triangle {
                        v: [adjs[a], adjs[b], vi],
                    };
                    self.flip22(&t, adjs[c]);
                    adjs.remove((cur + 1) % adjs.len());
                }
            }
            cur += 1;
        }
        //-- flip31 to remove the vertex
        self.flip31(vi);
        Ok(self.stars.len() - 1)
    }

    /// Write an [OBJ file](https://en.wikipedia.org/wiki/Wavefront_.obj_file) to disk.
    pub fn write_obj(&self, path: String) -> std::io::Result<()> {
        let trs = self.all_finite_triangles();
        let mut f = File::create(path)?;
        let mut s = String::new();
        //-- find one good vertice to replace the deleted one
        let mut onegoodpt = [1.0, 1.0, 1.0];
        for i in 1..self.stars.len() {
            if !self.stars[i].is_deleted() {
                onegoodpt[0] = self.stars[i].pt[0];
                onegoodpt[1] = self.stars[i].pt[1];
                onegoodpt[2] = self.stars[i].pt[2];
                break;
            }
        }
        for i in 1..self.stars.len() {
            if self.stars[i].is_deleted() {
                s.push_str(&format!(
                    "v {} {} {}\n",
                    onegoodpt[0], onegoodpt[1], onegoodpt[2]
                ));
                continue;
            }
            s.push_str(&format!(
                "v {} {} {}\n",
                self.stars[i].pt[0], self.stars[i].pt[1], self.stars[i].pt[2]
            ));
        }
        write!(f, "{}", s).unwrap();
        let mut s = String::new();
        for tr in trs.iter() {
            s.push_str(&format!("f {} {} {}\n", tr.v[0], tr.v[1], tr.v[2]));
        }
        write!(f, "{}", s).unwrap();
        // println!("write fobj: {:.2?}", starttime.elapsed());
        Ok(())
    }

    /// Write a [PLY file](https://en.wikipedia.org/wiki/PLY_(file_format)) to disk.
    pub fn write_ply(&self, path: String) -> std::io::Result<()> {
        let trs = self.all_finite_triangles();
        let mut f = File::create(path)?;
        //-- header
        writeln!(f, "ply").unwrap();
        writeln!(f, "format ascii 1.0").unwrap();
        writeln!(f, "comment made by startin").unwrap();
        writeln!(f, "element vertex {}", self.stars.len() - 1).unwrap();
        writeln!(f, "property double x").unwrap();
        writeln!(f, "property double y").unwrap();
        writeln!(f, "property double z").unwrap();
        for each in &self.attributes_schema {
            // println!("{:?}", each);
            match each.1.as_ref() {
                "f64" => {
                    writeln!(f, "property double {}", each.0).unwrap();
                }
                "i64" => {
                    writeln!(f, "property int {}", each.0).unwrap();
                }
                "u64" => {
                    writeln!(f, "property uint {}", each.0).unwrap();
                }
                "bool" => {
                    writeln!(f, "property uint {}", each.0).unwrap();
                }
                _ => (),
            }
        }
        writeln!(f, "element face {}", trs.len()).unwrap();
        writeln!(f, "property list uchar int vertex_indices").unwrap();
        writeln!(f, "end_header").unwrap();
        //-- find one good vertice to replace the deleted one
        let mut onegoodpt = [1.0, 1.0, 1.0];
        for i in 1..self.stars.len() {
            if !self.stars[i].is_deleted() {
                onegoodpt[0] = self.stars[i].pt[0];
                onegoodpt[1] = self.stars[i].pt[1];
                onegoodpt[2] = self.stars[i].pt[2];
                break;
            }
        }

        let mut s = String::new();
        for i in 1..self.stars.len() {
            if self.stars[i].is_deleted() {
                s.push_str(&format!(
                    "{} {} {}",
                    onegoodpt[0], onegoodpt[1], onegoodpt[2]
                ));
            } else {
                s.push_str(&format!(
                    "{} {} {}",
                    self.stars[i].pt[0], self.stars[i].pt[1], self.stars[i].pt[2]
                ));
            }
            //-- extra attributes
            for each in &self.attributes_schema {
                match each.1.as_ref() {
                    "f64" | "i64" | "u64" => {
                        let a = self.attributes.as_ref().unwrap();
                        let b = &a[i];
                        let c = &b[&each.0];
                        s.push_str(&format!(" {}", c));
                    }
                    "bool" => {
                        let a = self.attributes.as_ref().unwrap();
                        let b = &a[i];
                        let c = &b[&each.0];
                        if c == true {
                            s.push_str(" 1");
                        } else {
                            s.push_str(" 0");
                        }
                    }
                    _ => (),
                }
            }
            s.push_str("\n");
        }
        write!(f, "{}", s).unwrap();
        let mut s = String::new();
        for tr in trs.iter() {
            s.push_str(&format!(
                "3 {} {} {}\n",
                tr.v[0] - 1,
                tr.v[1] - 1,
                tr.v[2] - 1
            ));
        }
        write!(f, "{}", s).unwrap();
        Ok(())
    }

    /// Returns a [`String`] containing different statistics about the triangulation.
    pub fn printme(&self, withxyz: bool) -> String {
        let mut s = String::from("**********\n");
        // s.push_str(&format!("#pts: {}\n", self.number_pts()));
        for (i, p) in self.stars.iter().enumerate() {
            // s.push_str(&format!("{}: {}\n", i, self.stars[i].link));
            s.push_str(&format!("{}: [", i));
            for each in p.link.iter() {
                s.push_str(&format!("{} - ", each));
            }
            s.push_str(&"]\n".to_string());
            if withxyz {
                s.push_str(&format!("\t{:?}\n", self.stars[i].pt));
            }
        }
        s.push_str("**********\n");
        s
    }

    /// Returns the area of the Voronoi cell of `vi`.
    ///
    /// # Arguments
    ///
    /// * `vi` - the index of the vertex
    /// * `ignore_infinity` - calculate the area even is `vi` is on the convex hull.
    ///    This is used by [`interpolation::NNI`] when neighbours have no area, this bounds
    ///    arbitrarily the area and because we take the different the interpolated value
    ///    is the same at the end.
    pub fn voronoi_cell_area(&self, vi: usize, ignore_infinity: bool) -> Option<f64> {
        if !self.is_vertex_valid(vi) {
            return None;
        }
        if !ignore_infinity && self.is_vertex_convex_hull(vi) {
            return Some(f64::INFINITY);
        }
        //-- process non-CH points that exists
        let mut centres: Vec<Vec<f64>> = Vec::new();
        let mut l = self.stars[vi].link.clone();
        if l.contains_infinite_vertex() {
            l.delete(0);
        }
        for (i, n) in l.iter().enumerate() {
            let j = l.next_index(i);
            centres.push(geom::circle_centre(
                &self.stars[vi].pt,
                &self.stars[*n].pt,
                &self.stars[l[j]].pt,
            ));
        }
        //-- copy first to make circular
        centres.push(vec![centres[0][0], centres[0][1]]);
        let mut totalarea = 0.0_f64;
        for c in centres.windows(2) {
            totalarea += geom::area2d_triangle(&self.stars[vi].pt, &c[0], &c[1]);
        }
        Some(totalarea)
    }

    fn is_vertex_valid(&self, v: usize) -> bool {
        let mut re = true;
        if v >= self.stars.len() || self.stars[v].is_deleted() {
            re = false;
        }
        re
    }

    /// Returns the (axis-aligned) bounding box of the triangulation.
    pub fn get_bbox(&self) -> Vec<f64> {
        let mut minx: f64 = std::f64::MAX;
        let mut miny: f64 = std::f64::MAX;
        let mut maxx: f64 = std::f64::MIN;
        let mut maxy: f64 = std::f64::MIN;
        if self.stars.len() == 1 {
            minx = std::f64::NEG_INFINITY;
            miny = std::f64::NEG_INFINITY;
            maxx = std::f64::INFINITY;
            maxy = std::f64::INFINITY;
        } else {
            for i in 1..self.stars.len() {
                if self.is_vertex_removed(i).unwrap() {
                    continue;
                }
                if self.stars[i].pt[0] < minx {
                    minx = self.stars[i].pt[0];
                }
                if self.stars[i].pt[1] < miny {
                    miny = self.stars[i].pt[1];
                }
                if self.stars[i].pt[0] > maxx {
                    maxx = self.stars[i].pt[0];
                }
                if self.stars[i].pt[1] > maxy {
                    maxy = self.stars[i].pt[1];
                }
            }
        }
        vec![minx, miny, maxx, maxy]
    }

    /// Exaggerates vertically the z-values, used for visualisation mostly.
    ///
    /// The value can be <1.0 to have negative exaggeration.
    pub fn vertical_exaggeration(&mut self, factor: f64) {
        let mut minz: f64 = std::f64::MAX;
        for i in 1..self.stars.len() {
            if self.stars[i].is_deleted() {
                continue;
            }
            if self.stars[i].pt[2] < minz {
                minz = self.stars[i].pt[2];
            }
        }
        for i in 1..self.stars.len() {
            if self.stars[i].is_deleted() {
                continue;
            }
            let z2 = ((self.stars[i].pt[2] - minz) * factor) + minz;
            self.stars[i].pt[2] = z2;
        }
    }

    /// Update/set the z-value for a given vertex
    pub fn update_vertex_z_value(&mut self, vi: usize, z: f64) -> Result<bool, StartinError> {
        if vi == 0 {
            return Ok(false);
        }
        match self.is_vertex_removed(vi) {
            Err(why) => Err(why),
            Ok(_b) => {
                self.stars[vi].pt[2] = z;
                Ok(true)
            }
        }
    }

    pub fn has_garbage(&self) -> bool {
        self.number_of_removed_vertices() > 0
    }

    /// Collect garbage, that is remove from memory the vertices
    /// marked as removed.
    ///
    /// Watch out: the vertices get new IDs (and thus the triangles) too. And this can
    /// be a slow operation.
    pub fn collect_garbage(&mut self) {
        self.removed_indices.sort_unstable();
        for star in self.stars.iter_mut() {
            for value in star.link.0.iter_mut() {
                let pos = self.removed_indices.binary_search(value).unwrap_err();
                let newv = *value - pos;
                *value = newv;
            }
        }
        let mut offset = 0;
        for each in &self.removed_indices {
            self.stars.remove(each - offset);
            match &mut self.attributes {
                Some(x) => {
                    let _ = x.remove(each - offset);
                }
                None => (),
            };
            offset += 1;
        }
        self.removed_indices.clear();
    }
}

impl fmt::Display for Triangulation {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("======== TRIANGULATION ========\n")?;
        fmt.write_str(&format!("# vertices: {:19}\n", self.number_of_vertices()))?;
        fmt.write_str(&format!("# triangles: {:18}\n", self.number_of_triangles()))?;
        fmt.write_str(&format!(
            "# convex hull: {:16}\n",
            self.number_of_vertices_on_convex_hull()
        ))?;
        fmt.write_str("---\n")?;
        fmt.write_str(&format!(
            "extra_attributes: {:>13}\n",
            self.attributes.is_some()
        ))?;
        fmt.write_str(&format!("snap_tolerance: {:>15}\n", self.snaptol))?;
        fmt.write_str(&format!("jump_and_walk: {:>16}\n", self.jump_and_walk))?;
        fmt.write_str(&format!(
            "duplicates_handling: {:>10}\n",
            self.duplicates_handling.to_string()
        ))?;
        // if self.attributes.is_some() {
        //     fmt.write_str("---\n")?;
        //     let a: &Vec<Value> = self.attributes.as_ref().unwrap();
        //     for (i, _p) in self.stars.iter().enumerate() {
        //         // println!("vi:{} -- {:?}", i, a[i]);
        //         fmt.write_str(&format!("vi:{} -- {:?}\n", i, a[i]))?;
        //     }
        // }
        fmt.write_str("===============================\n")?;
        Ok(())
    }
}
