//! # startin
//!
//! [![crates.io](https://img.shields.io/crates/v/startin.svg)](https://crates.io/crates/startin)
//!
//! A Delaunay triangulator where the input are 2.5D points, the DT is computed in 2D but the elevation of the vertices are kept.
//! This is used mostly for the modelling of terrains.
//!
//! The construction algorithm used is an incremental insertion based on flips, and the data structure is a cheap implementation of the star-based structure defined in [Blandford et al. (2003)](https://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.9.6823), cheap because the link of each vertex is stored a simple array (`Vec`) and not in an optimised blob like they did.
//! It results in a pretty fast library (comparison will come at some point), but it uses more space than the optimised one.
//!
//! The deletion of a vertex is also possible. The algorithm implemented is a modification of the one of [Mostafavi, Gold, and Dakowicz (2003)](https://doi.org/10.1016/S0098-3004(03)00017-7). The ears are filled by flipping, so it's in theory more robust. I have also extended the algorithm to allow the deletion of vertices on the boundary of the convex hull. The algorithm is sub-optimal, but in practice the number of neighbours of a given vertex in a DT is only 6, so it doesn't really matter.
//!
//! Robust arithmetic for the geometric predicates are used ([Shewchuk's predicates](https://www.cs.cmu.edu/~quake/robust.html)), so the library is robust and shouldn't crash (touch wood).
//!
//! I made this in Rust because I wanted to learn Rust.
//!
//! But if you prefer Python, I made bindings (very beta): [https://github.com/hugoledoux/startin_python/](https://github.com/hugoledoux/startin_python/)
//!
//!
//! # Usage
//!
//! ```rust
//! extern crate startin;
//!
//! fn main() {
//!     let mut pts: Vec<Vec<f64>> = Vec::new();
//!     pts.push(vec![20.0, 30.0, 2.0]);
//!     pts.push(vec![120.0, 33.0, 12.5]);
//!     pts.push(vec![124.0, 222.0, 7.65]);
//!     pts.push(vec![20.0, 133.0, 21.0]);
//!     pts.push(vec![60.0, 60.0, 33.0]);
//!
//!     let mut dt = startin::Triangulation::new();
//!     dt.insert(&pts);
//!
//!     println!("*****");
//!     println!("Number of points in DT: {}", dt.number_of_vertices());
//!     println!("Number of triangles in DT: {}", dt.number_of_triangles());
//!
//!     //-- print all the vertices
//!     for (i, each) in dt.all_vertices().iter().enumerate() {
//!         // skip the first one, the infinite vertex
//!         if i > 0 {
//!             println!("#{}: ({:.3}, {:.3}, {:.3})", i, each[0], each[1], each[2]);
//!         }
//!     }
//!
//!     //-- insert a new vertex
//!     let re = dt.insert_one_pt(22.2, 33.3, 4.4);
//!     match re {
//!         Ok(_v) => println!("Inserted new point"),
//!         Err(v) => println!("Duplicate of vertex #{}, not inserted", v),
//!     }
//!     //-- remove it
//!     let re = dt.remove(6);
//!     if re.is_err() == true {
//!         println!("!!! Deletion error: {:?}", re.unwrap_err());
//!     } else {
//!         println!("Deleted vertex");
//!     }
//!
//!     //-- get the convex hull
//!     let ch = dt.convex_hull();
//!     println!("Convex hull: {:?}", ch);
//!
//!     //-- fetch triangle containing (x, y)
//!     let re = dt.locate(50.0, 50.0);
//!     if re.is_some() {
//!         let t = re.unwrap();
//!         println!("The triangle is {}", t);
//!         assert!(dt.is_triangle(&t));
//!     } else {
//!         println!("Outside convex hull");
//!     }
//!
//!     //-- some stats
//!     println!("Number of points in DT: {}", dt.number_of_vertices());
//!     println!("Number of triangles in DT: {}", dt.number_of_triangles());
//! }
//! ```

mod geom;

use rand::prelude::thread_rng;
use rand::Rng;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::mem;

extern crate rand;

/// A Triangle is a triplet of indices
pub struct Triangle {
    pub tr0: usize,
    pub tr1: usize,
    pub tr2: usize,
}

impl Triangle {
    /// Checks whether a Triangle is "infinite",
    /// ie if one its vertices is the infinite vertex
    fn is_infinite(&self) -> bool {
        if self.tr0 == 0 || self.tr1 == 0 || self.tr2 == 0 {
            return true;
        }
        return false;
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.tr0, self.tr1, self.tr2)
    }
}

//----------------------
pub struct Link(Vec<usize>);

impl Link {
    fn new() -> Link {
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
            return Some(self.0[(pos + 1)]);
        }
    }

    fn get_prev_vertex(&self, v: usize) -> Option<usize> {
        let re = self.get_index(v);
        if re.is_none() {
            return None;
        }
        let pos = re.unwrap();
        if pos == 0 {
            return Some(self.0[(self.0.len() - 1)]);
        } else {
            return Some(self.0[(pos - 1)]);
        }
    }

    fn iter(&self) -> Iter {
        Iter(Box::new(self.0.iter()))
    }
}

//-- taken from https://stackoverflow.com/questions/40668074/am-i-incorrectly-implementing-intoiterator-for-a-reference-or-is-this-a-rust-bug
struct Iter<'a>(Box<Iterator<Item = &'a usize> + 'a>);

impl<'a> Iterator for Iter<'a> {
    type Item = &'a usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

// impl<'a> IntoIterator for &'a Link {
//     type Item = &'a usize;
//     type IntoIter = Iter<'a>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.iter()
//     }
// }

// impl IntoIterator for Link {
//     type Item = usize;
//     type IntoIter = ::std::vec::IntoIter<usize>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

// impl Iterator for Link {
//     type Item = usize;

//     fn next(&mut self) -> Option<usize> {
//         // Increment our count. This is why we started at zero.
//         self.count += 1;

//         // Check to see if we've finished counting or not.
//         if self.count < 6 {
//             Some(self.count)
//         } else {
//             None
//         }
//     }
// }

impl std::ops::Index<usize> for Link {
    type Output = usize;
    fn index(&self, idx: usize) -> &usize {
        &self.0[idx as usize]
    }
}

/// A triangulation is a collection of Stars, each Star has its (x,y,z)
/// and a Link (an array of adjacent vertices, ordered CCW)
pub struct Star {
    pub pt: [f64; 3],
    pub link: Link,
}

impl Star {
    pub fn new(x: f64, y: f64, z: f64) -> Star {
        let l = Link::new();
        Star {
            pt: [x, y, z],
            link: l,
        }
    }
    pub fn is_deleted(&self) -> bool {
        self.link.is_empty()
    }
}

//----------------------
pub struct Triangulation {
    stars: Vec<Star>,
    snaptol: f64,
    cur: usize,
    is_init: bool,
    jump_and_walk: bool,
    robust_predicates: bool,
    free_indices: Vec<usize>,
}

impl Triangulation {
    //-- new
    pub fn new() -> Triangulation {
        // TODO: allocate a certain number?
        // let mut l: Vec<Star> = Vec::with_capacity(100000);
        let mut l: Vec<Star> = Vec::new();
        l.push(Star::new(-999.9, -999.9, -999.9));
        let es: Vec<usize> = Vec::new();
        Triangulation {
            stars: l,
            snaptol: 0.001,
            cur: 0,
            is_init: false,
            jump_and_walk: false,
            robust_predicates: true,
            free_indices: es,
        }
    }

    fn insert_one_pt_init_phase(&mut self, x: f64, y: f64, z: f64) -> Result<usize, usize> {
        let p: [f64; 3] = [x, y, z];
        for i in 1..self.stars.len() {
            if geom::distance2d_squared(&self.stars[i].pt, &p) <= (self.snaptol * self.snaptol) {
                return Err(i);
            }
        }
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
    /// one is closer than snap_tolerance to another one, then it is not inserted.
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

    /// Activate/deactive the jump-and-walk strategy for locate().
    /// If deactivated, then the walk starts from the last inserted triangle.
    pub fn set_jump_and_walk(&mut self, b: bool) {
        self.jump_and_walk = b;
    }

    pub fn is_using_robust_predicates(&self) -> bool {
        self.robust_predicates
    }

    pub fn use_robust_predicates(&mut self, b: bool) {
        self.robust_predicates = b;
    }

    pub fn insert(&mut self, pts: &Vec<Vec<f64>>) {
        let mut duplicates = 0;
        for each in pts {
            let re = self.insert_one_pt(each[0], each[1], each[2]);
            match re {
                Ok(_x) => continue,
                Err(_e) => duplicates = duplicates + 1,
            };
        }
    }

    //-- insert_one_pt
    pub fn insert_one_pt(&mut self, px: f64, py: f64, pz: f64) -> Result<usize, usize> {
        // println!("-->{}", p);
        if self.is_init == false {
            return self.insert_one_pt_init_phase(px, py, pz);
        }
        //-- walk
        let p: [f64; 3] = [px, py, pz];
        let tr = self.walk(&p);
        // println!("STARTING TR: {}", tr);
        if geom::distance2d_squared(&self.stars[tr.tr0].pt, &p) <= (self.snaptol * self.snaptol) {
            return Err(tr.tr0);
        }
        if geom::distance2d_squared(&self.stars[tr.tr1].pt, &p) <= (self.snaptol * self.snaptol) {
            return Err(tr.tr1);
        }
        if geom::distance2d_squared(&self.stars[tr.tr2].pt, &p) <= (self.snaptol * self.snaptol) {
            return Err(tr.tr2);
        }
        //-- ok we now insert the point in the data structure
        let pi: usize;
        if self.free_indices.is_empty() == true {
            self.stars.push(Star::new(px, py, pz));
            pi = self.stars.len() - 1;
        } else {
            // self.stars.push(Star::new(px, py, pz));
            pi = self.free_indices.pop().unwrap();
            self.stars[pi].pt[0] = px;
            self.stars[pi].pt[1] = py;
            self.stars[pi].pt[2] = pz;
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
            tr0: pi,
            tr1: self.stars[pi].link[0],
            tr2: self.stars[pi].link[1],
        });
        mystack.push(Triangle {
            tr0: pi,
            tr1: self.stars[pi].link[1],
            tr2: self.stars[pi].link[2],
        });
        mystack.push(Triangle {
            tr0: pi,
            tr1: self.stars[pi].link[2],
            tr2: self.stars[pi].link[0],
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
                if tr.tr0 == 0 {
                    a = geom::orient2d(
                        &self.stars[opposite].pt,
                        &self.stars[tr.tr1].pt,
                        &self.stars[tr.tr2].pt,
                        self.robust_predicates,
                    );
                } else if tr.tr1 == 0 {
                    a = geom::orient2d(
                        &self.stars[tr.tr0].pt,
                        &self.stars[opposite].pt,
                        &self.stars[tr.tr2].pt,
                        self.robust_predicates,
                    );
                } else if tr.tr2 == 0 {
                    a = geom::orient2d(
                        &self.stars[tr.tr0].pt,
                        &self.stars[tr.tr1].pt,
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
                        &self.stars[tr.tr0].pt,
                        &self.stars[tr.tr1].pt,
                        &self.stars[tr.tr2].pt,
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
                        &self.stars[tr.tr0].pt,
                        &self.stars[tr.tr1].pt,
                        &self.stars[tr.tr2].pt,
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
        self.stars[pi].link.add(tr.tr0);
        self.stars[pi].link.add(tr.tr1);
        self.stars[pi].link.add(tr.tr2);
        self.stars[tr.tr0].link.insert_after_v(pi, tr.tr1);
        self.stars[tr.tr1].link.insert_after_v(pi, tr.tr2);
        self.stars[tr.tr2].link.insert_after_v(pi, tr.tr0);
        //-- put infinite vertex first in list
        self.stars[pi].link.infinite_first();
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
        self.stars[v].pt[0] = -999.9;
        self.stars[v].pt[1] = -999.9;
        self.stars[v].pt[2] = -999.9;
        self.free_indices.push(v);
        self.cur = ns[0];
    }

    /// Returns the coordinates of the vertex v in a Vec [x,y,z]
    pub fn get_point(&self, v: usize) -> Vec<f64> {
        self.stars[v].pt.to_vec()
    }

    pub fn adjacent_triangles_to_triangle(&self, tr: &Triangle) -> Vec<Triangle> {
        let mut trs: Vec<Triangle> = Vec::new();
        if self.is_triangle(&tr) == false || tr.is_infinite() == true {
            return trs;
        }
        let mut opp = self.stars[tr.tr2].link.get_next_vertex(tr.tr1).unwrap();
        if opp != 0 {
            trs.push(Triangle {
                tr0: tr.tr1,
                tr1: opp,
                tr2: tr.tr2,
            });
        }
        opp = self.stars[tr.tr0].link.get_next_vertex(tr.tr2).unwrap();
        if opp != 0 {
            trs.push(Triangle {
                tr0: tr.tr2,
                tr1: opp,
                tr2: tr.tr0,
            });
        }
        opp = self.stars[tr.tr1].link.get_next_vertex(tr.tr0).unwrap();
        if opp != 0 {
            trs.push(Triangle {
                tr0: tr.tr0,
                tr1: opp,
                tr2: tr.tr1,
            });
        }
        trs
    }

    // Returns a Vec of Triangles (finite + infinite) to the vertex v.
    // If v doesn't exist, then an empty Vec is returned.
    pub fn incident_triangles_to_vertex(&self, v: usize) -> Vec<Triangle> {
        let mut trs: Vec<Triangle> = Vec::new();
        if v >= self.stars.len() {
            return trs;
        }
        for (i, each) in self.stars[v].link.iter().enumerate() {
            let j = self.stars[v].link.next_index(i);
            trs.push(Triangle {
                tr0: v,
                tr1: *each,
                tr2: self.stars[v].link[j],
            });
        }
        trs
    }

    pub fn adjacent_vertices_to_vertex(&self, v: usize) -> Vec<usize> {
        // TODO: should infinite vertex be returned here? I guess not?
        let mut adjs: Vec<usize> = Vec::new();
        if v >= self.stars.len() {
            return adjs;
        }
        for each in self.stars[v].link.iter() {
            adjs.push(*each);
        }
        adjs
    }

    /// Returns whether a triplet of indices is a Triangle in the triangulation.
    pub fn is_triangle(&self, tr: &Triangle) -> bool {
        // TODO: what about infinite triangles?
        let re = self.stars[tr.tr0].link.get_next_vertex(tr.tr1);
        if re.is_none() {
            return false;
        } else {
            if re.unwrap() == tr.tr2 {
                return true;
            } else {
                return false;
            }
        }
    }

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
    pub fn number_of_vertices(&self) -> usize {
        //-- number of finite vertices
        (self.stars.len() - 1 - self.free_indices.len())
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
                        let tr = Triangle {
                            tr0: i,
                            tr1: *value,
                            tr2: k,
                        };
                        if tr.is_infinite() == false {
                            count = count + 1;
                        }
                    }
                }
            }
        }
        count
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

    /// Returns size of the convex hull of the dataset
    pub fn number_of_vertices_on_convex_hull(&self) -> usize {
        //-- number of finite vertices on the boundary of the convex hull
        if self.is_init == false {
            return 0;
        }
        return self.stars[0].link.len();
    }

    /// Returns true if the vertex v is part of the boundary of the convex
    /// hull of the dataset. False otherwise.
    pub fn is_vertex_convex_hull(&self, v: usize) -> bool {
        if v == 0 {
            return false;
        }
        if v >= self.stars.len() {
            return false;
        }
        self.stars[v].link.contains_infinite_vertex()
    }

    /// Returns, if it exists, the Triangle containing (px,py).
    /// If it is direction on a vertex/edge, then one is randomly chosen.
    pub fn locate(&self, px: f64, py: f64) -> Option<Triangle> {
        let p: [f64; 3] = [px, py, 0.0];
        let re = self.walk(&p);
        match re.is_infinite() {
            true => None,
            false => Some(re),
        }
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
        let mut tr = Triangle {
            tr0: 0,
            tr1: 0,
            tr2: 0,
        };
        // println!("cur: {}", cur);
        //-- 1. find a finite triangle
        tr.tr0 = cur;
        if self.stars[cur].link[0] == 0 {
            tr.tr1 = self.stars[cur].link[1];
            tr.tr2 = self.stars[cur].link[2];
        } else {
            tr.tr1 = self.stars[cur].link[0];
            tr.tr2 = self.stars[cur].link[1];
        }
        //-- 2. order it such that tr0-tr1-x is CCW
        if geom::orient2d(
            &self.stars[tr.tr0].pt,
            &self.stars[tr.tr1].pt,
            &x,
            self.robust_predicates,
        ) == -1
        {
            if geom::orient2d(
                &self.stars[tr.tr1].pt,
                &self.stars[tr.tr2].pt,
                &x,
                self.robust_predicates,
            ) != -1
            {
                mem::swap(&mut tr.tr0, &mut tr.tr1);
                mem::swap(&mut tr.tr1, &mut tr.tr2);
            } else {
                mem::swap(&mut tr.tr1, &mut tr.tr2);
                mem::swap(&mut tr.tr0, &mut tr.tr1);
            }
        }
        //-- 3. start the walk
        //-- we know that tr0-tr1-x is CCW
        loop {
            if tr.is_infinite() == true {
                break;
            }
            if geom::orient2d(
                &self.stars[tr.tr1].pt,
                &self.stars[tr.tr2].pt,
                &x,
                self.robust_predicates,
            ) != -1
            {
                if geom::orient2d(
                    &self.stars[tr.tr2].pt,
                    &self.stars[tr.tr0].pt,
                    &x,
                    self.robust_predicates,
                ) != -1
                {
                    break;
                } else {
                    //-- walk to incident to tr1,tr2
                    // println!("here");
                    let prev = self.stars[tr.tr2].link.get_prev_vertex(tr.tr0).unwrap();
                    tr.tr1 = tr.tr2;
                    tr.tr2 = prev;
                }
            } else {
                //-- walk to incident to tr1,tr2
                // a.iter().position(|&x| x == 2), Some(1)
                let prev = self.stars[tr.tr1].link.get_prev_vertex(tr.tr2).unwrap();
                tr.tr0 = tr.tr2;
                tr.tr2 = prev;
            }
        }
        return tr;
    }

    fn flip22(&mut self, tr: &Triangle, opposite: usize) -> (Triangle, Triangle) {
        //-- step 1.
        self.stars[tr.tr0].link.insert_after_v(opposite, tr.tr1);
        //-- step 2.
        self.stars[tr.tr1].link.delete(tr.tr2);
        //-- step 3.
        self.stars[opposite].link.insert_after_v(tr.tr0, tr.tr2);
        //-- step 4.
        self.stars[tr.tr2].link.delete(tr.tr1);
        //-- make 2 triangles to return (to stack)
        let ret0 = Triangle {
            tr0: tr.tr0,
            tr1: tr.tr1,
            tr2: opposite,
        };
        let ret1 = Triangle {
            tr0: tr.tr0,
            tr1: opposite,
            tr2: tr.tr2,
        };
        (ret0, ret1)
    }

    fn get_opposite_vertex(&self, tr: &Triangle) -> usize {
        self.stars[tr.tr2].link.get_next_vertex(tr.tr1).unwrap()
    }

    /// Returns a Vec<Vec<f64>> of all the vertices (including the infinite one)
    pub fn all_vertices(&self) -> Vec<Vec<f64>> {
        let mut pts: Vec<Vec<f64>> = Vec::with_capacity(self.stars.len() - 1);
        for i in 0..self.stars.len() {
            pts.push(self.stars[i].pt.to_vec());
        }
        pts
    }

    pub fn all_triangles(&self) -> Vec<Triangle> {
        let mut trs: Vec<Triangle> = Vec::new();
        for (i, star) in self.stars.iter().enumerate() {
            //-- reconstruct triangles
            for (j, value) in star.link.iter().enumerate() {
                if i < *value {
                    // let k = star.l[self.nexti(star.link.len(), j)];
                    let k = star.link[star.link.next_index(j)];
                    if i < k {
                        let tr = Triangle {
                            tr0: i,
                            tr1: *value,
                            tr2: k,
                        };
                        if tr.is_infinite() == false {
                            // println!("{}", tr);
                            trs.push(tr);
                        }
                    }
                }
            }
        }
        trs
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid_ch_convex() && self.is_valid_circumcircle()
    }

    fn is_valid_circumcircle(&self) -> bool {
        let mut re = true;
        let trs = self.all_triangles();
        for tr in trs.iter() {
            for i in 1..self.stars.len() {
                if self.stars[i].is_deleted() == false
                    && geom::incircle(
                        &self.stars[tr.tr0].pt,
                        &self.stars[tr.tr1].pt,
                        &self.stars[tr.tr2].pt,
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

    pub fn remove_on_convex_hull(&mut self, v: usize) -> Result<usize, &'static str> {
        // println!("!!! REMOVE ON CONVEX HULL");
        let mut adjs: Vec<usize> = Vec::new();
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
                        tr0: adjs[a],
                        tr1: adjs[b],
                        tr2: v,
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
            self.stars[v].pt[0] = -999.9;
            self.stars[v].pt[1] = -999.9;
            self.stars[v].pt[2] = -999.9;
            self.free_indices.push(v);
            return Ok(self.stars.len() - 1);
        }
    }

    pub fn remove(&mut self, v: usize) -> Result<usize, &'static str> {
        // println!("REMOVE vertex {}", v);
        if v == 0 {
            return Err("Cannot remove the infinite vertex");
        }
        if (v >= self.stars.len()) || (self.stars[v].link.len() == 0) {
            return Err("Vertex does not exist");
        }
        if self.is_vertex_convex_hull(v) {
            return self.remove_on_convex_hull(v);
        }
        let mut adjs: Vec<usize> = Vec::new();
        for each in self.stars[v].link.iter() {
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
                        tr0: adjs[a],
                        tr1: adjs[b],
                        tr2: v,
                    };
                    self.flip22(&t, adjs[c]);
                    adjs.remove((cur + 1) % adjs.len());
                }
            }
            cur = cur + 1;
        }
        //-- flip31 to remove the vertex
        self.flip31(v);
        return Ok(self.stars.len() - 1);
    }

    pub fn write_obj(&self, path: String, twod: bool) -> std::io::Result<()> {
        let trs = self.all_triangles();
        let mut f = File::create(path)?;
        for i in 1..self.stars.len() {
            if twod == true {
                write!(
                    f,
                    "v {} {} {}\n",
                    self.stars[i].pt[0], self.stars[i].pt[1], 0
                )
                .unwrap();
            } else {
                write!(
                    f,
                    "v {} {} {}\n",
                    self.stars[i].pt[0], self.stars[i].pt[1], self.stars[i].pt[2]
                )
                .unwrap();
            }
        }
        for tr in trs.iter() {
            write!(f, "f {} {} {}\n", tr.tr0, tr.tr1, tr.tr2).unwrap();
        }
        Ok(())
    }

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
        fmt.write_str(&format!("---\nrobust: {}\n", self.robust_predicates))?;
        fmt.write_str("===============================\n")?;
        Ok(())
    }
}
