mod geom;

use rand::prelude::thread_rng;
use rand::Rng;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::mem;

extern crate rand;

//----------------------
pub struct Triangle {
    pub tr0: usize,
    pub tr1: usize,
    pub tr2: usize,
}

impl Triangle {
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
            return Some(self.l[(self.l.len() - 1)]);
        } else {
            return Some(self.l[(pos - 1)]);
        }
    }
}

impl std::ops::Index<usize> for Link {
    type Output = usize;
    fn index(&self, idx: usize) -> &usize {
        &self.l[idx as usize]
    }
}
// impl std::ops::Index<u32> for Link {
//     type Output = u32;
//     fn index(&self, idx: u32) -> &u32 {
//         &self.l[idx as usize]
//     }
// }

//----------------------
pub struct Star {
    pub pt: [f64; 3],
    pub link: Link,
}

impl Star {
    pub fn new(x: f64, y: f64, z: f64) -> Star {
        // let s: Vec<usize> = Vec::with_capacity(8);
        let l = Link::new();
        Star {
            pt: [x, y, z],
            // link: s,
            link: l,
        }
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
}

impl Triangulation {
    //-- new
    pub fn new() -> Triangulation {
        let mut l: Vec<Star> = Vec::with_capacity(100000);
        // let mut l: Vec<Star> = Vec::new();
        l.push(Star::new(0.0, 0.0, 0.0));
        unsafe {
            geom::shewchuk::exactinit();
        }
        Triangulation {
            stars: l,
            snaptol: 0.001,
            cur: 0,
            is_init: false,
            jump_and_walk: true,
            robust_predicates: true,
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
                self.stars[0].link.push(a);
                self.stars[0].link.push(c);
                self.stars[0].link.push(b);
                self.stars[a].link.push(0);
                self.stars[a].link.push(b);
                self.stars[a].link.push(c);
                self.stars[b].link.push(0);
                self.stars[b].link.push(c);
                self.stars[b].link.push(a);
                self.stars[c].link.push(0);
                self.stars[c].link.push(a);
                self.stars[c].link.push(b);
                self.is_init = true;
            } else if re == -1 {
                // println!("init: ({},{},{})", a, c, b);
                self.stars[0].link.push(a);
                self.stars[0].link.push(b);
                self.stars[0].link.push(c);
                self.stars[a].link.push(0);
                self.stars[a].link.push(c);
                self.stars[a].link.push(b);
                self.stars[b].link.push(0);
                self.stars[b].link.push(a);
                self.stars[b].link.push(c);
                self.stars[c].link.push(0);
                self.stars[c].link.push(b);
                self.stars[c].link.push(a);
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

    pub fn set_snap_tolerance(&mut self, snaptol: f64) -> f64 {
        if snaptol > 0.0 {
            self.snaptol = snaptol;
        }
        self.snaptol
    }

    pub fn get_snap_tolerance(&self) -> f64 {
        self.snaptol
    }

    pub fn set_jump_and_walk(&mut self, b: bool) {
        self.jump_and_walk = b;
    }

    pub fn get_robust_predicates(&self) -> bool {
        self.robust_predicates
    }

    pub fn set_robust_predicates(&mut self, b: bool) {
        self.robust_predicates = b;
    }

    //-- insert_one_pt
    pub fn insert_one_pt(&mut self, px: f64, py: f64, pz: f64) -> Result<usize, usize> {
        // println!("-->{}", p);
        if self.is_init == false {
            return self.insert_one_pt_init_phase(px, py, pz);
        }
        //-- walk
        // println!("Walking");
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
        self.stars.push(Star::new(px, py, pz));
        let pi: usize = self.stars.len() - 1;
        //-- flip13()
        self.flip13(pi, &tr);
        //-- update_dt()
        self.update_dt(pi);

        self.cur = self.stars.len() - 1;
        Ok(self.stars.len() - 1)
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
                    let (ret0, ret1) = self.flip(&tr, opposite);
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
                        let (ret0, ret1) = self.flip(&tr, 0);
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
                        let (ret0, ret1) = self.flip(&tr, opposite);
                        mystack.push(ret0);
                        mystack.push(ret1);
                    }
                }
            }
        }
    }

    fn flip13(&mut self, pi: usize, tr: &Triangle) {
        self.stars[pi].link.push(tr.tr0);
        self.stars[pi].link.push(tr.tr1);
        self.stars[pi].link.push(tr.tr2);
        self.stars[tr.tr0].link.insert_after_v(pi, tr.tr1);
        self.stars[tr.tr1].link.insert_after_v(pi, tr.tr2);
        self.stars[tr.tr2].link.insert_after_v(pi, tr.tr0);
        //-- put infinite vertex first in list
        self.stars[pi].link.infinite_first();
    }

    pub fn get_point(&self, i: usize) -> Vec<f64> {
        self.stars[i].pt.to_vec()
    }

    pub fn is_triangle(&self, tr: &Triangle) -> bool {
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

    pub fn stats_degree(&self) -> (f64, usize, usize) {
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

    pub fn number_of_vertices(&self) -> usize {
        //-- number of finite vertices
        (self.stars.len() - 1)
    }

    pub fn number_of_triangles(&self) -> usize {
        //-- number of finite triangles
        let mut count: usize = 0;
        for (i, star) in self.stars.iter().enumerate() {
            for (j, value) in star.link.l.iter().enumerate() {
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

    // Get convex hull, oriented CCW
    // as a list of vertices (first != last)
    pub fn get_convex_hull(&self) -> Vec<usize> {
        let mut re: Vec<usize> = Vec::new();
        for x in self.stars[0].link.l.iter().rev() {
            re.push(*x);
        }
        re
    }

    pub fn number_of_vertices_on_convex_hull(&self) -> usize {
        //-- number of finite vertices on the boundary of the convex hull
        if self.is_init == false {
            return 0;
        }
        return self.stars[0].link.len();
    }

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

    fn flip(&mut self, tr: &Triangle, opposite: usize) -> (Triangle, Triangle) {
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

    pub fn get_vertices(&self) -> Vec<Vec<f64>> {
        let mut pts: Vec<Vec<f64>> = Vec::with_capacity(self.stars.len() - 1);
        for i in 1..self.stars.len() {
            pts.push(self.stars[i].pt.to_vec());
        }
        pts
    }

    fn get_triangles(&self) -> Vec<Triangle> {
        let mut trs: Vec<Triangle> = Vec::new();
        for (i, star) in self.stars.iter().enumerate() {
            //-- reconstruct triangles
            for (j, value) in star.link.l.iter().enumerate() {
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

    pub fn is_delaunay(&self) -> bool {
        let mut re = true;
        let trs = self.get_triangles();
        for tr in trs.iter() {
            for i in 1..self.stars.len() {
                if geom::incircle(
                    &self.stars[tr.tr0].pt,
                    &self.stars[tr.tr1].pt,
                    &self.stars[tr.tr2].pt,
                    &self.stars[i].pt,
                    self.robust_predicates,
                ) > 0
                {
                    // println!("NOT DELAUNAY FFS!");
                    re = false
                }
            }
        }
        re
    }

    pub fn write_obj(&self, path: String, twod: bool) -> std::io::Result<()> {
        let trs = self.get_triangles();
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
