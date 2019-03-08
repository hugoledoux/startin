// $ ./rustin < ../../samples2.xyz

#![allow(dead_code)]

extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

// use serde_json::json;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io;
use std::io::Write;
use std::mem;

#[derive(Debug, Deserialize)]
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
pub struct Triangulation {
    pts: Vec<Point3d>,
    stars: Vec<Vec<usize>>,
    tol: f64,
    cur: usize,
}

impl Triangulation {
    //-- new
    pub fn new() -> Triangulation {
        //-- add point at infinity
        let mut v: Vec<Point3d> = Vec::new();
        v.push(Point3d {
            x: 9999999.0,
            y: 9999999.0,
            z: 9999999.0,
        });
        let mut s: Vec<Vec<usize>> = Vec::new();
        s.push([].to_vec());
        Triangulation {
            pts: v,
            stars: s,
            tol: 0.001,
            cur: 0,
        }
    }

    //-- insert_one_pt
    pub fn insert_one_pt(&mut self, p: Point3d) -> (usize, bool) {
        // println!("-->{:?}", p);
        if self.pts.len() <= 3 {
            for (i, pi) in self.pts.iter().enumerate() {
                if pi.square_2d_distance(&p) <= (self.tol * self.tol) {
                    return (i, false);
                }
            }
            self.pts.push(p);
            self.stars.push([].to_vec());
            if self.pts.len() == 4 {
                if orient2d(&self.pts[1], &self.pts[2], &self.pts[3]) == 1 {
                    self.stars[0].push(1);
                    self.stars[0].push(2);
                    self.stars[0].push(3);
                    self.stars[1].push(0);
                    self.stars[1].push(2);
                    self.stars[1].push(3);
                    self.stars[2].push(0);
                    self.stars[2].push(3);
                    self.stars[2].push(1);
                    self.stars[3].push(0);
                    self.stars[3].push(1);
                    self.stars[3].push(2);
                } else {
                    self.stars[0].push(1);
                    self.stars[0].push(2);
                    self.stars[0].push(3);
                    self.stars[1].push(0);
                    self.stars[1].push(3);
                    self.stars[1].push(2);
                    self.stars[2].push(0);
                    self.stars[2].push(1);
                    self.stars[2].push(3);
                    self.stars[3].push(0);
                    self.stars[3].push(2);
                    self.stars[3].push(1);
                }
            }
            self.cur = self.pts.len() - 1;
            return (self.pts.len() - 1, true);
        } else {
            let tr = self.walk(&p);
            if p.square_2d_distance(&self.pts[tr.tr0]) < (self.tol * self.tol) {
                return (tr.tr0, false);
            }
            if p.square_2d_distance(&self.pts[tr.tr1]) < (self.tol * self.tol) {
                return (tr.tr1, false);
            }
            if p.square_2d_distance(&self.pts[tr.tr2]) < (self.tol * self.tol) {
                return (tr.tr2, false);
            }
            self.pts.push(p);
            self.stars.push([].to_vec());
            let pi = self.pts.len() - 1;
            self.stars[pi].push(tr.tr0);
            self.stars[pi].push(tr.tr1);
            self.stars[pi].push(tr.tr2);

            let mut i = self.index_in_star(&self.stars[tr.tr0], tr.tr1);
            self.stars[tr.tr0].insert(i + 1, pi);
            i = self.index_in_star(&self.stars[tr.tr1], tr.tr2);
            self.stars[tr.tr1].insert(i + 1, pi);
            i = self.index_in_star(&self.stars[tr.tr2], tr.tr0);
            self.stars[tr.tr2].insert(i + 1, pi);

            //-- put infinite vertex first in list
            self.star_update_infinite_first(pi);

            // println!("-->FLIP");
            let mut mystack: Vec<Triangle> = Vec::new();
            mystack.push(Triangle {
                tr0: pi,
                tr1: tr.tr0,
                tr2: tr.tr1,
            });
            mystack.push(Triangle {
                tr0: pi,
                tr1: tr.tr1,
                tr2: tr.tr2,
            });
            mystack.push(Triangle {
                tr0: pi,
                tr1: tr.tr2,
                tr2: tr.tr0,
            });

            loop {
                let tr = match mystack.pop() {
                    None => break,
                    Some(x) => x,
                };
                let opposite = self.get_opposite_vertex(&tr);
                if opposite != 0 {
                    if tr.is_infinite() == true {
                        let mut a: i8 = 0;
                        if tr.tr0 == 0 {
                            a = orient2d(&self.pts[opposite], &self.pts[tr.tr1], &self.pts[tr.tr2]);
                        } else if tr.tr1 == 0 {
                            a = orient2d(&self.pts[tr.tr0], &self.pts[opposite], &self.pts[tr.tr2]);
                        } else if tr.tr2 == 0 {
                            a = orient2d(&self.pts[tr.tr0], &self.pts[tr.tr1], &self.pts[opposite]);
                        }
                        if a > 0 {
                            let (ret0, ret1) = self.flip(&tr, opposite);
                            mystack.push(ret0);
                            mystack.push(ret1);
                        }
                    } else {
                        if incircle(
                            &self.pts[tr.tr0],
                            &self.pts[tr.tr1],
                            &self.pts[tr.tr2],
                            &self.pts[opposite],
                        ) > 0
                        {
                            let (ret0, ret1) = self.flip(&tr, opposite);
                            mystack.push(ret0);
                            mystack.push(ret1);
                        }
                    }
                }
            }

            self.cur = self.pts.len() - 1;
            return (self.pts.len() - 1, true);
        }
    }

    pub fn number_vertices(&self) -> usize {
        //-- number of finite vertices
        (self.pts.len() - 1)
    }

    fn walk(&self, x: &Point3d) -> Triangle {
        //-- TODO: random sample some and pick closest?
        //-- find the starting tr
        let mut tr = Triangle {
            tr0: 0,
            tr1: 0,
            tr2: 0,
        };
        let cur = self.cur;
        //-- 1. find a finite triangle
        tr.tr0 = cur;
        if self.stars[cur][0] == 0 {
            tr.tr1 = self.stars[cur][1];
            tr.tr2 = self.stars[cur][2];
        } else {
            tr.tr1 = self.stars[cur][0];
            tr.tr2 = self.stars[cur][1];
        }
        //-- 2. order it such that tr0-tr1-x is CCW
        if orient2d(&self.pts[tr.tr0], &self.pts[tr.tr1], &x) == -1 {
            if orient2d(&self.pts[tr.tr1], &self.pts[tr.tr2], &x) != -1 {
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
            if orient2d(&self.pts[tr.tr1], &self.pts[tr.tr2], &x) != -1 {
                if orient2d(&self.pts[tr.tr2], &self.pts[tr.tr0], &x) != -1 {
                    break;
                } else {
                    //-- walk to incident to tr1,tr2
                    // println!("here");
                    let pos = &self.stars[tr.tr2]
                        .iter()
                        .position(|&x| x == tr.tr0)
                        .unwrap();
                    let prev = self.prev_vertex_star(&self.stars[tr.tr2], *pos);
                    tr.tr1 = tr.tr2;
                    tr.tr2 = prev;
                }
            } else {
                //-- walk to incident to tr1,tr2
                // a.iter().position(|&x| x == 2), Some(1)
                let pos = &self.stars[tr.tr1]
                    .iter()
                    .position(|&x| x == tr.tr2)
                    .unwrap();
                let prev = self.prev_vertex_star(&self.stars[tr.tr1], *pos);
                tr.tr0 = tr.tr2;
                tr.tr2 = prev;
            }
        }
        return tr;
    }

    fn flip(&mut self, tr: &Triangle, opposite: usize) -> (Triangle, Triangle) {
        //-- step 1.
        let mut pos = self.index_in_star(&self.stars[tr.tr0], tr.tr1);
        self.stars[tr.tr0].insert(pos + 1, opposite);
        //-- step 2.
        pos = self.index_in_star(&self.stars[tr.tr1], tr.tr2);
        self.stars[tr.tr1].remove(pos);
        //-- step 3.
        pos = self.index_in_star(&self.stars[opposite], tr.tr2);
        self.stars[opposite].insert(pos + 1, tr.tr0);
        //-- step 4.
        pos = self.index_in_star(&self.stars[tr.tr2], tr.tr1);
        self.stars[tr.tr2].remove(pos);
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
        let re = self.stars[i].iter().position(|&x| x == 0);
        if re != None {
            let posinf = re.unwrap();
            if posinf == 0 {
                return;
            }
            let mut newstar: Vec<usize> = Vec::new();
            for j in posinf..self.stars[i].len() {
                newstar.push(self.stars[i][j]);
            }
            for j in 0..posinf {
                newstar.push(self.stars[i][j]);
            }
            // println!("newstar: {:?}", newstar);
            self.stars[i] = newstar;
        }
    }

    fn next_pos_star(&self, s: &Vec<usize>, i: usize) -> usize {
        //-- get next position/index in the star
        //-- helper function not have a circular star
        if i == (s.len() - 1) {
            0
        } else {
            (i + 1)
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

    fn prev_pos_star(&self, s: &Vec<usize>, i: usize) -> usize {
        //-- get next position/index in the star
        //-- helper function not have a circular star
        if i == 0 {
            (s.len() - 1)
        } else {
            (i - 1)
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
        let pos = self.index_in_star(&self.stars[tr.tr2], tr.tr1);
        self.next_vertex_star(&self.stars[tr.tr2], pos)
    }

    fn delete_in_star(&mut self, i: usize, value: usize) {
        let re = self.stars[i].iter().position(|&x| x == value);
        if re != None {
            self.stars[i].remove(re.unwrap());
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
            for (j, value) in star.iter().enumerate() {
                if i < *value {
                    let k = star[self.nexti(star.len(), j)];
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

    fn is_delaunay(&self) -> bool {
        let mut re = true;
        let trs = self.get_triangles();
        for tr in trs.iter() {
            for (i, pt) in self.pts.iter().enumerate() {
                if i == 0 {
                    continue;
                }
                if incircle(&self.pts[tr.tr0], &self.pts[tr.tr1], &self.pts[tr.tr2], pt) > 0 {
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
        for (i, v) in self.pts.iter().enumerate() {
            if i != 0 {
                if twod == true {
                    write!(f, "v {} {} {}\n", v.x, v.y, 0).unwrap();
                } else {
                    write!(f, "v {} {} {}\n", v.x, v.y, v.z).unwrap();
                }
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
        fmt.write_str("=== TRIANGULATION ===\n")?;
        fmt.write_str(&format!("#pts: {}\n", self.number_vertices()))?;
        for (i, _p) in self.pts.iter().enumerate() {
            fmt.write_str(&format!("{}: {:?}\n", i, self.stars[i]))?;
        }
        fmt.write_str("===============\n")?;
        Ok(())
    }
}

//--------------------------------------------------

fn main() {
    let re = read_xyz_file();
    let vec = match re {
        Ok(vec) => vec,
        Err(error) => panic!("Problem with the file {:?}", error),
    };

    // println!("===TOTAL: {} points", re.len());
    // println!("{:?}", vec);
    // dosmth(&vec);

    // for (i, p) in vec.iter().enumerate() {
    //   println!("#{}: {}", i, p.printme());
    // }

    let mut tr = Triangulation::new();
    for p in vec.into_iter() {
        // println!("{}", p);
        let (i, b) = tr.insert_one_pt(p);
        if b == false {
            println!("Duplicate point ({})", i);
        }
        // else {
        //     println!("{}", tr);
        // }
    }

    println!("****** is Delaunay? ******");
    println!("{}", tr.is_delaunay());
    println!("**************************");

    // println!("Number of points in DT: {}", tr.number_pts());
    // println!("{}", tr);
    tr.write_obj("/Users/hugo/temp/out.obj".to_string(), false)
        .unwrap();
}

fn read_xyz_file() -> Result<Vec<Point3d>, Box<Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .from_reader(io::stdin());
    let mut vpts: Vec<Point3d> = Vec::new();
    for result in rdr.deserialize() {
        let record: Point3d = result?;
        vpts.push(record);
    }
    Ok(vpts)
}

fn orient2d(a: &Point3d, b: &Point3d, c: &Point3d) -> i8 {
    //-- CCW    = +1
    //-- CW     = -1
    //-- linear = 0
    let re: f64 = ((a.x - c.x) * (b.y - c.y)) - ((a.y - c.y) * (b.x - c.x));
    if re.abs() < 1e-12 {
        return 0;
    } else if re > 0.0 {
        return 1;
    } else {
        return -1;
    }
}

fn incircle(a: &Point3d, b: &Point3d, c: &Point3d, p: &Point3d) -> i8 {
    //-- INSIDE   == +1
    //-- OUTSIDE  == -1
    //-- ONCIRCLE == 0
    let at = (
        a.x - p.x,
        a.y - p.y,
        (a.x * a.x + a.y * a.y) - (p.x * p.x + p.y * p.y),
    );
    let bt = (
        b.x - p.x,
        b.y - p.y,
        (b.x * b.x + b.y * b.y) - (p.x * p.x + p.y * p.y),
    );
    let ct = (
        c.x - p.x,
        c.y - p.y,
        (c.x * c.x + c.y * c.y) - (p.x * p.x + p.y * p.y),
    );
    let i = at.0 * (bt.1 * ct.2 - bt.2 * ct.1);
    let j = at.1 * (bt.0 * ct.2 - bt.2 * ct.0);
    let k = at.2 * (bt.0 * ct.1 - bt.1 * ct.0);
    let re = i - j + k;
    // println!("INCIRCLE TEST: {}", re);
    if re.abs() < 1e-12 {
        return 0;
    } else if re > 0.0 {
        return 1;
    } else {
        return -1;
    }
}
