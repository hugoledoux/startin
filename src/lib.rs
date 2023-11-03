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
//!         Err(v) => println!("Duplicate of vertex #{}, not inserted", v),
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

mod geom;
pub mod interpolation;

#[cfg(feature = "c_api")]
mod c_interface;

use rand::prelude::thread_rng;
use rand::Rng;
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
}

/// Possibilities for the insertion (with `insert()`)
pub enum InsertionStrategy {
    AsIs,
    BBox,
    // Sprinkle,
    // ConBRIO,
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
        return false;
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
        if self.0.len() == 0 {
            true
        } else {
            false
        }
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
        if re != None {
            self.0.remove(re.unwrap());
        }
    }
    fn replace(&mut self, v: usize, newv: usize) {
        let re = self.0.iter().position(|&x| x == v);
        if re != None {
            self.0[re.unwrap()] = newv;
            // self.0.remove(re.unwrap());
        }
    }
    fn infinite_first(&mut self) {
        let re = self.0.iter().position(|&x| x == 0);
        if re != None {
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
        if pos == None {
            return false;
        } else {
            return true;
        }
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
        if re.is_none() {
            return None;
        }
        let pos = re.unwrap();
        if pos == (self.0.len() - 1) {
            return Some(self.0[0]);
        } else {
            return Some(self.0[pos + 1]);
        }
    }
    fn get_prev_vertex(&self, v: usize) -> Option<usize> {
        let re = self.get_index(v);
        if re.is_none() {
            return None;
        }
        let pos = re.unwrap();
        if pos == 0 {
            return Some(self.0[self.0.len() - 1]);
        } else {
            return Some(self.0[pos - 1]);
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
        &self.0[idx as usize]
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
struct Star<T> {
    pt: [f64; 2],
    data: T,
    link: Link,
}

impl<T: Default> Star<T> {
    fn new(x: f64, y: f64, d: T) -> Star<T> {
        let l = Link::new();
        Star {
            pt: [x, y],
            data: d,
            link: l,
        }
    }
    fn is_deleted(&self) -> bool {
        self.link.is_empty()
    }
}

/// Represents a triangulation
#[repr(C)]
pub struct Triangulation<T> {
    stars: Vec<Star<T>>,
    snaptol: f64,
    cur: usize,
    is_init: bool,
    jump_and_walk: bool,
    robust_predicates: bool,
    removed_indices: Vec<usize>,
}

impl<T: Default> Triangulation<T> {
    pub fn new() -> Triangulation<T> {
        // TODO: allocate a certain number?
        // let mut l: Vec<Star> = Vec::with_capacity(100000);
        let mut l: Vec<Star<T>> = Vec::new();
        l.push(Star::new(f64::INFINITY, f64::INFINITY, T::default()));
        let es: Vec<usize> = Vec::new();
        Triangulation {
            stars: l,
            snaptol: 0.001,
            cur: 0,
            is_init: false,
            jump_and_walk: false,
            robust_predicates: true,
            removed_indices: es,
        }
    }

    fn insert_one_pt_init_phase(&mut self, x: f64, y: f64, data: T) -> Result<usize, usize> {
        let p: [f64; 2] = [x, y];
        for i in 1..self.stars.len() {
            if geom::distance2d_squared(&self.stars[i].pt, &p) <= (self.snaptol * self.snaptol) {
                return Err(i);
            }
        }
        self.collect_garbage();
        //-- add point to Triangulation and create its empty star
        self.stars.push(Star::new(x, y, data));
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
        if self.is_init == true {
            //-- insert the previous vertices in the dt
            for j in 1..(l - 3) {
                let tr = self.walk(&self.stars[j].pt);
                // println!("found tr: {}", tr);
                self.flip13(j, &tr);
                self.update_dt(j);
            }
        }
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

    /// Is using robut predicates (with [crate robust](https://docs.rs/robust))?
    /// (activated by default)
    pub fn is_using_robust_predicates(&self) -> bool {
        self.robust_predicates
    }

    /// Activate/deactivate [robust predictates](https://docs.rs/robust)
    pub fn use_robust_predicates(&mut self, b: bool) {
        self.robust_predicates = b;
    }

    /// Insert a [`Vec`] of [`array`] (`[f64; 3]`) values.
    /// If [`InsertionStrategy::AsIs`] is used, then [`Triangulation::insert_one_pt()`] is called
    /// for each point in the order given.
    ///
    /// # Arguments
    ///
    /// * `pts` - a [`Vec`] of `[f64; 3]`
    /// * `strategy` - the [`InsertionStrategy`] to use for the insertion
    pub fn insert(&mut self, pts: Vec<(f64, f64, T)>, strategy: InsertionStrategy) {
        match strategy {
            InsertionStrategy::BBox => {
                //-- find the bbox

                let pts_2d = pts.iter().map(|(x,y,_)| [*x,*y]).collect::<Vec<_>>();
                let mut bbox = geom::bbox2d(&pts_2d);
                //-- "padding" of the bbox to avoid conflicts
                bbox[0] = bbox[0] - 10.0;
                bbox[1] = bbox[1] - 10.0;
                bbox[2] = bbox[2] + 10.0;
                bbox[3] = bbox[3] + 10.0 ;
                self.insert_with_bbox(pts, &bbox);
            }
            InsertionStrategy::AsIs => {
                for (x, y, data) in pts.into_iter() {
                    let _re = self.insert_one_pt(x, y, data);
                }
            }
            // InsertionStrategy::Sprinkle => println!("Sprinkle not implemented yet"),
            // InsertionStrategy::ConBRIO => println!("ConBRIO not implemented yet"),
        }
    }

    fn insert_with_bbox(&mut self, pts: Vec<(f64, f64, T)>, bbox: &[f64; 4]) {
        let mut c4: Vec<usize> = Vec::new();
        //-- insert the 4 corners
        c4.push(self.insert_one_pt(bbox[0], bbox[1], T::default()).unwrap());
        c4.push(self.insert_one_pt(bbox[2], bbox[1], T::default()).unwrap());
        c4.push(self.insert_one_pt(bbox[2], bbox[3], T::default()).unwrap());
        c4.push(self.insert_one_pt(bbox[0], bbox[3], T::default()).unwrap());
        for (x, y, data) in pts {
            let _re = self.insert_one_pt(x, y, data);
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
    /// If there was a vertex at that location, an Error is thrown with the already
    /// existing vertex ID.
    pub fn insert_one_pt(&mut self, px: f64, py: f64, data: T) -> Result<usize, usize> {
        if self.is_init == false {
            return self.insert_one_pt_init_phase(px, py, data);
        }
        //-- walk
        let p: [f64; 2] = [px, py];
        let tr = self.walk(&p);
        // println!("STARTING TR: {}", tr);
        if geom::distance2d_squared(&self.stars[tr.v[0]].pt, &p) <= (self.snaptol * self.snaptol) {
            return Err(tr.v[0]);
        }
        if geom::distance2d_squared(&self.stars[tr.v[1]].pt, &p) <= (self.snaptol * self.snaptol) {
            return Err(tr.v[1]);
        }
        if geom::distance2d_squared(&self.stars[tr.v[2]].pt, &p) <= (self.snaptol * self.snaptol) {
            return Err(tr.v[2]);
        }
        //-- ok we now insert the point in the data structure
        let pi: usize;
        if self.removed_indices.is_empty() == true {
            self.stars.push(Star::new(px, py, data));
            pi = self.stars.len() - 1;
        } else {
            // self.stars.push(Star::new(px, py, pz));
            pi = self.removed_indices.pop().unwrap();
            self.stars[pi].pt[0] = px;
            self.stars[pi].pt[1] = py;
            self.stars[pi].data = data;
        }
        //-- flip13()
        self.flip13(pi, &tr);
        //-- update_dt()
        self.update_dt(pi);

        self.cur = pi;
        Ok(pi)
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

            if tr.is_infinite() == true {
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
        // self.stars[pi].link.infinite_first();
    }

    fn flip31(&mut self, v: usize) {
        // println!("FLIP31");
        let mut ns: Vec<usize> = Vec::new();
        for each in self.stars[v].link.iter() {
            ns.push(*each);
        }
        for n in ns.iter() {
            self.stars[*n].link.delete(v);
        }
        self.stars[v].link.clear();
        self.stars[v].pt[0] = f64::NAN;
        self.stars[v].pt[1] = f64::NAN;
        self.stars[v].pt[2] = f64::NAN;
        self.removed_indices.push(v);
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
            Err(why) => return Err(why),
            Ok(b) => match b {
                true => return Err(StartinError::VertexRemoved),
                false => Ok(self.stars[vi].pt.to_vec()),
            },
        }
    }

    /// Returns the 3 adjacents (finite + infinite) [`Triangle`] to a triangle.
    pub fn adjacent_triangles_to_triangle(
        &self,
        tr: &Triangle,
    ) -> Result<Vec<Triangle>, StartinError> {
        if self.is_triangle(&tr) == false || tr.is_infinite() == true {
            return Err(StartinError::TriangleNotPresent);
        }
        let mut trs: Vec<Triangle> = Vec::new();
        let mut opp = self.stars[tr.v[2]].link.get_next_vertex(tr.v[1]).unwrap();
        if opp != 0 {
            trs.push(Triangle {
                v: [tr.v[1], opp, tr.v[2]],
            });
        }
        opp = self.stars[tr.v[0]].link.get_next_vertex(tr.v[2]).unwrap();
        if opp != 0 {
            trs.push(Triangle {
                v: [tr.v[2], opp, tr.v[0]],
            });
        }
        opp = self.stars[tr.v[1]].link.get_next_vertex(tr.v[0]).unwrap();
        if opp != 0 {
            trs.push(Triangle {
                v: [tr.v[0], opp, tr.v[1]],
            });
        }
        Ok(trs)
    }

    /// Returns a [`Vec`] of [`Triangle`]s (finite + infinite) to the vertex `vi`.
    pub fn incident_triangles_to_vertex(&self, vi: usize) -> Result<Vec<Triangle>, StartinError> {
        match self.is_vertex_removed(vi) {
            Err(why) => return Err(why),
            Ok(b) => match b {
                true => return Err(StartinError::VertexRemoved),
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

    /// Returns the degree of the vertex with ID `vi`.
    pub fn degree(&self, vi: usize) -> Result<usize, StartinError> {
        match self.is_vertex_removed(vi) {
            Err(why) => return Err(why),
            Ok(b) => match b {
                true => return Err(StartinError::VertexRemoved),
                false => return Ok(self.stars[vi].link.len()),
            },
        }
    }

    /// Returns a list (`Vec<usize>`) (ordered CCW) of the adjacent vertices to `vi`.
    pub fn adjacent_vertices_to_vertex(&self, vi: usize) -> Result<Vec<usize>, StartinError> {
        match self.is_vertex_removed(vi) {
            Err(why) => return Err(why),
            Ok(b) => match b {
                true => return Err(StartinError::VertexRemoved),
                false => {
                    let mut adjs: Vec<usize> = Vec::new();
                    for each in self.stars[vi].link.iter() {
                        adjs.push(*each);
                    }
                    return Ok(adjs);
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
        if tr.is_infinite() {
            return false;
        } else {
            return true;
        }
    }

    /// Returns some statistics about the triangulation.
    pub fn statistics_degree(&self) -> (f64, usize, usize) {
        let mut total: f64 = 0.0;
        let mut min: usize = usize::max_value();
        let mut max: usize = usize::min_value();
        for i in 1..self.stars.len() {
            total = total + self.stars[i].link.len() as f64;
            if self.stars[i].link.len() > max {
                max = self.stars[i].link.len();
            }
            if self.stars[i].link.len() < min {
                min = self.stars[i].link.len();
            }
        }
        total = total / (self.stars.len() - 2) as f64;
        return (total, min, max);
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
                        if tr.is_infinite() == false {
                            count = count + 1;
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
        if self.is_init == false {
            return 0;
        }
        return self.stars[0].link.len();
    }

    /// Returns `true` if the vertex `vi` is part of the boundary of the convex
    /// hull of the dataset; `false` otherwise.
    pub fn is_vertex_convex_hull(&self, vi: usize) -> bool {
        if vi == 0 {
            return false;
        }
        if self.is_vertex_valid(vi) == false {
            return false;
        }
        self.stars[vi].link.contains_infinite_vertex()
    }

    /// Returns, if it exists, the [`Triangle`] containing `(px, py)`.
    /// If it is direction on a vertex/edge, then one is randomly chosen.
    pub fn locate(&self, px: f64, py: f64) -> Result<Triangle, StartinError> {
        if self.is_init == false {
            return Err(StartinError::EmptyTriangulation);
        }
        let p: [f64; 3] = [px, py, 0.0];
        let re = self.walk(&p);
        match re.is_infinite() {
            true => return Err(StartinError::OutsideConvexHull),
            false => return Ok(re),
        }
    }

    /// Returns closest point (in 2D) to a query point `(x, y)`.
    /// if `(px, py)` is outside the convex hull then [`StartinError::OutsideConvexHull`] is raised.
    pub fn closest_point(&self, px: f64, py: f64) -> Result<usize, StartinError> {
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
            if found_one_closer == false {
                break;
            }
        }
        Ok(closest)
    }

    fn walk(&self, x: &[f64]) -> Triangle {
        //-- find the starting tr
        let mut cur = self.cur;
        //-- jump-and-walk
        if self.jump_and_walk == true {
            let mut rng = thread_rng();
            let mut d: f64 = geom::distance2d_squared(&self.stars[self.cur].pt, &x);
            let n = (self.stars.len() as f64).powf(0.25);
            // let n = (self.stars.len() as f64).powf(0.25) * 7.0;
            for _i in 0..n as i32 {
                let re: usize = rng.gen_range(1, self.stars.len());
                // let dtemp = x.square_2d_distance(&self.stars[re].pt);
                if self.stars[re].is_deleted() == true {
                    continue;
                }
                let dtemp = geom::distance2d_squared(&self.stars[re].pt, &x);
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
            &x,
            self.robust_predicates,
        ) == -1
        {
            if geom::orient2d(
                &self.stars[tr.v[1]].pt,
                &self.stars[tr.v[2]].pt,
                &x,
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
            if tr.is_infinite() == true {
                break;
            }
            if geom::orient2d(
                &self.stars[tr.v[1]].pt,
                &self.stars[tr.v[2]].pt,
                &x,
                self.robust_predicates,
            ) != -1
            {
                if geom::orient2d(
                    &self.stars[tr.v[2]].pt,
                    &self.stars[tr.v[0]].pt,
                    &x,
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
        return tr;
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
            if t.is_infinite() == false {
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
                if self.stars[i].is_deleted() == false
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
        if re == false {
            println!("CONVEX NOT CONVEX");
        }
        return re;
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
                if isdel == true {
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
            return Ok(self.stars.len() - 1);
        } else {
            //-- convex part is filled, and we need to apply a special "flip"
            //-- to delete the vertex v and its incident edges
            // println!("FLIP-FOR-CH");
            self.stars[adjs[1]].link.delete(v);
            self.stars[*(adjs.last().unwrap())].link.delete(v);
            for i in 2..(adjs.len() - 1) {
                if self.is_vertex_convex_hull(adjs[i]) == true {
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
        if self.is_init == false {
            self.stars[vi].pt[0] = f64::NAN;
            self.stars[vi].pt[1] = f64::NAN;
            self.stars[vi].pt[2] = f64::NAN;
            self.removed_indices.push(vi);
        }
        match self.is_vertex_removed(vi) {
            Err(why) => return Err(why),
            Ok(b) => {
                if b == true {
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
                if isdel == true {
                    // println!("flip22");
                    let t = Triangle {
                        v: [adjs[a], adjs[b], vi],
                    };
                    self.flip22(&t, adjs[c]);
                    adjs.remove((cur + 1) % adjs.len());
                }
            }
            cur = cur + 1;
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
        let mut onegoodpt = vec![1.0, 1.0, 1.0];
        for i in 1..self.stars.len() {
            if self.stars[i].is_deleted() == false {
                onegoodpt[0] = self.stars[i].pt[0];
                onegoodpt[1] = self.stars[i].pt[1];
                onegoodpt[2] = self.stars[i].pt[2];
                break;
            }
        }
        for i in 1..self.stars.len() {
            if self.stars[i].is_deleted() == true {
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
        write!(f, "ply\n").unwrap();
        write!(f, "format ascii 1.0\n").unwrap();
        write!(f, "comment made by startin\n").unwrap();
        write!(f, "element vertex {}\n", self.stars.len() - 1).unwrap();
        write!(f, "property float x\n").unwrap();
        write!(f, "property float y\n").unwrap();
        write!(f, "property float z\n").unwrap();
        write!(f, "element face {}\n", trs.len()).unwrap();
        write!(f, "property list uchar int vertex_indices\n").unwrap();
        write!(f, "end_header\n").unwrap();
        //-- find one good vertice to replace the deleted one
        let mut onegoodpt = vec![1.0, 1.0, 1.0];
        for i in 1..self.stars.len() {
            if self.stars[i].is_deleted() == false {
                onegoodpt[0] = self.stars[i].pt[0];
                onegoodpt[1] = self.stars[i].pt[1];
                onegoodpt[2] = self.stars[i].pt[2];
                break;
            }
        }
        let mut s = String::new();
        for i in 1..self.stars.len() {
            if self.stars[i].is_deleted() == true {
                s.push_str(&format!(
                    "{} {} {}\n",
                    onegoodpt[0], onegoodpt[1], onegoodpt[2]
                ));
                continue;
            }
            s.push_str(&format!(
                "{} {} {}\n",
                self.stars[i].pt[0], self.stars[i].pt[1], self.stars[i].pt[2]
            ));
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
            s.push_str(&format!("]\n"));
            if withxyz == true {
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
        if self.is_vertex_valid(vi) == false {
            return None;
        }
        if (ignore_infinity == false) && (self.is_vertex_convex_hull(vi) == true) {
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
            totalarea += geom::area_triangle(&self.stars[vi].pt, &c[0], &c[1]);
        }
        Some(totalarea)
    }

    fn is_vertex_valid(&self, v: usize) -> bool {
        let mut re = true;
        if v >= self.stars.len() || self.stars[v].is_deleted() == true {
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
        for i in 1..self.stars.len() {
            if self.is_vertex_removed(i).unwrap() == true {
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
        vec![minx, miny, maxx, maxy]
    }

    /// Exaggerates vertically the z-values, used for visualisation mostly.
    ///
    /// The value can be <1.0 to have negative exaggeration.
    pub fn vertical_exaggeration(&mut self, factor: f64) {
        let mut minz: f64 = std::f64::MAX;
        for i in 1..self.stars.len() {
            if self.stars[i].is_deleted() == true {
                continue;
            }
            if self.stars[i].pt[2] < minz {
                minz = self.stars[i].pt[2];
            }
        }
        for i in 1..self.stars.len() {
            if self.stars[i].is_deleted() == true {
                continue;
            }
            let z2 = ((self.stars[i].pt[2] - minz) * factor) + minz;
            self.stars[i].pt[2] = z2;
        }
    }

    pub fn has_garbage(&self) -> bool {
        if self.number_of_removed_vertices() > 0 {
            true
        } else {
            false
        }
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
            offset += 1;
        }
        self.removed_indices.clear();
    }
}
impl<T: Default> fmt::Display for Triangulation<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("======== TRIANGULATION ========\n")?;
        fmt.write_str(&format!("# vertices: {:19}\n", self.number_of_vertices()))?;
        fmt.write_str(&format!("# triangles: {:18}\n", self.number_of_triangles()))?;
        fmt.write_str(&format!(
            "# convex hull: {:16}\n",
            self.number_of_vertices_on_convex_hull()
        ))?;
        fmt.write_str(&format!("---\n"))?;
        fmt.write_str(&format!("robust: {}\n", self.robust_predicates))?;
        fmt.write_str(&format!("tolerance: {}\n", self.snaptol))?;
        fmt.write_str("===============================\n")?;
        Ok(())
    }
}
