mod geom;

use rand::prelude::thread_rng;
use rand::Rng;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::mem;

extern crate rand;

pub struct Point3d {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3d {
    fn square_2d_distance(&self, p: &Point3d) -> f64 {
        (p.x - self.x) * (p.x - self.x) + (p.y - self.y) * (p.y - self.y)
    }
}

impl fmt::Display for Point3d {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "POINT({:.3}, {:.3}, {:.3})", self.x, self.y, self.z)
    }
}

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
pub struct Star {
    pub pt: Point3d,
    pub link: Vec<usize>,
}

impl Star {
    pub fn new(p: Point3d) -> Star {
        let s: Vec<usize> = Vec::with_capacity(8);
        let p = Point3d {
            x: p.x,
            y: p.y,
            z: p.z,
        };
        Star { pt: p, link: s }
    }
}

//----------------------
pub struct Triangulation {
    stars: Vec<Star>,
    snaptol: f64,
    cur: usize,
    is_init: bool,
}

impl Triangulation {
    //-- new
    pub fn new() -> Triangulation {
        let infinity = Point3d {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let mut l: Vec<Star> = Vec::with_capacity(100000);
        // let mut l: Vec<Star> = Vec::new();
        l.push(Star::new(infinity));
        Triangulation {
            stars: l,
            snaptol: 0.001,
            cur: 0,
            is_init: false,
        }
    }

    fn insert_one_pt_init_phase(&mut self, p: Point3d) -> Result<usize, usize> {
        for i in 1..self.stars.len() {
            if self.stars[i].pt.square_2d_distance(&p) <= (self.snaptol * self.snaptol) {
                return Err(i);
            }
        }
        //-- add point to Triangulation and create its empty star
        self.stars.push(Star::new(p));
        //-- form the first triangles (finite + infinite)
        let l = self.stars.len();
        if l >= 4 {
            let a = l - 3;
            let b = l - 2;
            let c = l - 1;
            let re = geom::orient2d(&self.stars[a].pt, &self.stars[b].pt, &self.stars[c].pt);
            if re == 1 {
                // println!("init: ({},{},{})", a, b, c);
                let mut v = vec![a, c, b];
                self.stars[0].link.append(&mut v);
                v = vec![0, b, c];
                self.stars[a].link.append(&mut v);
                v = vec![0, c, a];
                self.stars[b].link.append(&mut v);
                v = vec![0, a, b];
                self.stars[c].link.append(&mut v);
                self.is_init = true;
            } else if re == -1 {
                // println!("init: ({},{},{})", a, c, b);
                let mut v = vec![a, b, c];
                self.stars[0].link.append(&mut v);
                v = vec![0, c, b];
                self.stars[a].link.append(&mut v);
                v = vec![0, a, c];
                self.stars[b].link.append(&mut v);
                v = vec![0, b, a];
                self.stars[c].link.append(&mut v);
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

    //-- insert_one_pt
    pub fn insert_one_pt(&mut self, px: f64, py: f64, pz: f64) -> Result<usize, usize> {
        let p = Point3d {
            x: px,
            y: py,
            z: pz,
        };
        // println!("-->{}", p);
        if self.is_init == false {
            return self.insert_one_pt_init_phase(p);
        }
        //-- walk
        // println!("Walking");
        let tr = self.walk(&p);
        // println!("STARTING TR: {}", tr);
        if p.square_2d_distance(&self.stars[tr.tr0].pt) < (self.snaptol * self.snaptol) {
            return Err(tr.tr0);
        }
        if p.square_2d_distance(&self.stars[tr.tr1].pt) < (self.snaptol * self.snaptol) {
            return Err(tr.tr1);
        }
        if p.square_2d_distance(&self.stars[tr.tr2].pt) < (self.snaptol * self.snaptol) {
            return Err(tr.tr2);
        }
        self.stars.push(Star::new(p));
        let pi = self.stars.len() - 1;
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
                    );
                } else if tr.tr1 == 0 {
                    a = geom::orient2d(
                        &self.stars[tr.tr0].pt,
                        &self.stars[opposite].pt,
                        &self.stars[tr.tr2].pt,
                    );
                } else if tr.tr2 == 0 {
                    a = geom::orient2d(
                        &self.stars[tr.tr0].pt,
                        &self.stars[tr.tr1].pt,
                        &self.stars[opposite].pt,
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
        let mut i = self.index_in_star(&self.stars[tr.tr0].link, tr.tr1);
        self.stars[tr.tr0].link.insert(i + 1, pi);
        i = self.index_in_star(&self.stars[tr.tr1].link, tr.tr2);
        self.stars[tr.tr1].link.insert(i + 1, pi);
        i = self.index_in_star(&self.stars[tr.tr2].link, tr.tr0);
        self.stars[tr.tr2].link.insert(i + 1, pi);
        //-- put infinite vertex first in list
        self.star_update_infinite_first(pi);
    }

    pub fn get_point(&self, i: usize) -> Point3d {
        let p = &self.stars[i].pt;
        Point3d {
            x: p.x,
            y: p.y,
            z: p.z,
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
            for (j, value) in star.link.iter().enumerate() {
                if i < *value {
                    let k = star.link[self.nexti(star.link.len(), j)];
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
        for x in self.stars[0].link.iter().rev() {
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

    fn walk(&self, x: &Point3d) -> Triangle {
        //-- TODO: random sample some and pick closest?
        //-- find the starting tr

        let mut cur = self.cur;

        //-- jump-and-walk
        // let mut rng = thread_rng();
        // let mut d: f64 = x.square_2d_distance(&self.pts[self.cur]);
        // // let n = (self.pts.len() as f64).powf(0.25);
        // let n = (self.pts.len() as f64).powf(0.25) * 7.0;
        // for _i in 0..n as i32 {
        //     let re: usize = rng.gen_range(1, self.pts.len());
        //     let dtemp = x.square_2d_distance(&self.pts[re]);
        //     if dtemp < d {
        //         cur = re;
        //         d = dtemp;
        //     }
        // }

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
        if geom::orient2d(&self.stars[tr.tr0].pt, &self.stars[tr.tr1].pt, &x) == -1 {
            if geom::orient2d(&self.stars[tr.tr1].pt, &self.stars[tr.tr2].pt, &x) != -1 {
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
            if geom::orient2d(&self.stars[tr.tr1].pt, &self.stars[tr.tr2].pt, &x) != -1 {
                if geom::orient2d(&self.stars[tr.tr2].pt, &self.stars[tr.tr0].pt, &x) != -1 {
                    break;
                } else {
                    //-- walk to incident to tr1,tr2
                    // println!("here");
                    let pos = &self.stars[tr.tr2]
                        .link
                        .iter()
                        .position(|&x| x == tr.tr0)
                        .unwrap();
                    let prev = self.prev_vertex_star(&self.stars[tr.tr2].link, *pos);
                    tr.tr1 = tr.tr2;
                    tr.tr2 = prev;
                }
            } else {
                //-- walk to incident to tr1,tr2
                // a.iter().position(|&x| x == 2), Some(1)
                let pos = &self.stars[tr.tr1]
                    .link
                    .iter()
                    .position(|&x| x == tr.tr2)
                    .unwrap();
                let prev = self.prev_vertex_star(&self.stars[tr.tr1].link, *pos);
                tr.tr0 = tr.tr2;
                tr.tr2 = prev;
            }
        }
        return tr;
    }

    fn flip(&mut self, tr: &Triangle, opposite: usize) -> (Triangle, Triangle) {
        //-- step 1.
        let mut pos = self.index_in_star(&self.stars[tr.tr0].link, tr.tr1);
        self.stars[tr.tr0].link.insert(pos + 1, opposite);
        //-- step 2.
        self.delete_in_star(tr.tr1, tr.tr2);
        //-- step 3.
        pos = self.index_in_star(&self.stars[opposite].link, tr.tr2);
        self.stars[opposite].link.insert(pos + 1, tr.tr0);
        //-- step 4.
        self.delete_in_star(tr.tr2, tr.tr1);
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

    fn star_update_infinite_first(&mut self, i: usize) {
        // println!("INFINITE {:?}", self.stars[i]);
        let re = self.stars[i].link.iter().position(|&x| x == 0);
        if re != None {
            let posinf = re.unwrap();
            if posinf == 0 {
                return;
            }
            let mut newstar: Vec<usize> = Vec::new();
            for j in posinf..self.stars[i].link.len() {
                newstar.push(self.stars[i].link[j]);
            }
            for j in 0..posinf {
                newstar.push(self.stars[i].link[j]);
            }
            // println!("newstar: {:?}", newstar);
            self.stars[i].link = newstar;
        }
    }

    fn next_vertex_star(&self, s: &Vec<usize>, i: usize) -> usize {
        //-- get next vertex (its global index) in a star
        if i == (s.len() - 1) {
            s[0]
        } else {
            s[(i + 1)]
        }
    }

    fn prev_vertex_star(&self, s: &Vec<usize>, i: usize) -> usize {
        //-- get prev vertex (its global index) in a star
        if i == 0 {
            s[(s.len() - 1)]
        } else {
            s[(i - 1)]
        }
    }

    fn index_in_star(&self, s: &Vec<usize>, i: usize) -> usize {
        s.iter().position(|&x| x == i).unwrap()
    }

    fn get_opposite_vertex(&self, tr: &Triangle) -> usize {
        let pos = self.index_in_star(&self.stars[tr.tr2].link, tr.tr1);
        self.next_vertex_star(&self.stars[tr.tr2].link, pos)
    }

    fn delete_in_star(&mut self, i: usize, value: usize) {
        let re = self.stars[i].link.iter().position(|&x| x == value);
        if re != None {
            self.stars[i].link.remove(re.unwrap());
        }
    }

    fn nexti(&self, len: usize, i: usize) -> usize {
        if i == (len - 1) {
            0
        } else {
            i + 1
        }
    }

    fn get_triangles(&self) -> Vec<Triangle> {
        let mut trs: Vec<Triangle> = Vec::new();
        for (i, star) in self.stars.iter().enumerate() {
            //-- reconstruct triangles
            for (j, value) in star.link.iter().enumerate() {
                if i < *value {
                    let k = star.link[self.nexti(star.link.len(), j)];
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
                write!(f, "v {} {} {}\n", self.stars[i].pt.x, self.stars[i].pt.y, 0).unwrap();
            } else {
                write!(
                    f,
                    "v {} {} {}\n",
                    self.stars[i].pt.x, self.stars[i].pt.y, self.stars[i].pt.z
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
        // for (i, _p) in self.pts.iter().enumerate() {
        //     fmt.write_str(&format!("{}: {:?}\n", i, self.stars[i]))?;
        // }
        fmt.write_str("===============================\n")?;
        Ok(())
    }
}

impl fmt::Debug for Triangulation {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str("======== DEBUG ========\n")?;
        fmt.write_str(&format!("# vertices: {:19}\n", self.number_of_vertices()))?;
        fmt.write_str(&format!("# triangles: {:18}\n", self.number_of_triangles()))?;
        fmt.write_str(&format!(
            "# convex hull: {:16}\n",
            self.number_of_vertices_on_convex_hull()
        ))?;
        for (i, _p) in self.stars.iter().enumerate() {
            fmt.write_str(&format!("{}: {:?}\n", i, self.stars[i].link))?;
        }
        fmt.write_str("===============================\n")?;
        Ok(())
    }
}
