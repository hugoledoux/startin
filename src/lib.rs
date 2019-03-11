mod predicates;

use std::fmt;
use std::fs::File;
use std::io::Write;
use std::mem;

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
    pub fn insert_one_pt(&mut self, px: f64, py: f64, pz: f64) -> (bool, usize) {
        let p = Point3d {
            x: px,
            y: py,
            z: pz,
        };
        // println!("-->{:?}", p);
        if self.pts.len() <= 3 {
            for (i, pi) in self.pts.iter().enumerate() {
                if pi.square_2d_distance(&p) <= (self.tol * self.tol) {
                    return (false, i);
                }
            }
            self.pts.push(p);
            self.stars.push([].to_vec());
            if self.pts.len() == 4 {
                if predicates::orient2d(&self.pts[1], &self.pts[2], &self.pts[3]) == 1 {
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
            return (true, self.pts.len() - 1);
        } else {
            let tr = self.walk(&p);
            if p.square_2d_distance(&self.pts[tr.tr0]) < (self.tol * self.tol) {
                return (false, tr.tr0);
            }
            if p.square_2d_distance(&self.pts[tr.tr1]) < (self.tol * self.tol) {
                return (false, tr.tr1);
            }
            if p.square_2d_distance(&self.pts[tr.tr2]) < (self.tol * self.tol) {
                return (false, tr.tr2);
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
                            a = predicates::orient2d(
                                &self.pts[opposite],
                                &self.pts[tr.tr1],
                                &self.pts[tr.tr2],
                            );
                        } else if tr.tr1 == 0 {
                            a = predicates::orient2d(
                                &self.pts[tr.tr0],
                                &self.pts[opposite],
                                &self.pts[tr.tr2],
                            );
                        } else if tr.tr2 == 0 {
                            a = predicates::orient2d(
                                &self.pts[tr.tr0],
                                &self.pts[tr.tr1],
                                &self.pts[opposite],
                            );
                        }
                        if a > 0 {
                            let (ret0, ret1) = self.flip(&tr, opposite);
                            mystack.push(ret0);
                            mystack.push(ret1);
                        }
                    } else {
                        if predicates::incircle(
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

    pub fn number_of_vertices(&self) -> usize {
        //-- number of finite vertices
        (self.pts.len() - 1)
    }

    pub fn number_of_triangles(&self) -> usize {
        //-- number of finite triangles
        let mut count: usize = 0;
        for (i, star) in self.stars.iter().enumerate() {
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
                            count = count + 1;
                        }
                    }
                }
            }
        }
        count
    }

    pub fn number_of_vertices_on_ch(&self) -> usize {
        //-- number of finite vertices on the boundary of the convex hull
        self.stars[0].len()
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
        if predicates::orient2d(&self.pts[tr.tr0], &self.pts[tr.tr1], &x) == -1 {
            if predicates::orient2d(&self.pts[tr.tr1], &self.pts[tr.tr2], &x) != -1 {
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
            if predicates::orient2d(&self.pts[tr.tr1], &self.pts[tr.tr2], &x) != -1 {
                if predicates::orient2d(&self.pts[tr.tr2], &self.pts[tr.tr0], &x) != -1 {
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
        self.delete_in_star(tr.tr1, tr.tr2);
        //-- step 3.
        pos = self.index_in_star(&self.stars[opposite], tr.tr2);
        self.stars[opposite].insert(pos + 1, tr.tr0);
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

    pub fn is_delaunay(&self) -> bool {
        let mut re = true;
        let trs = self.get_triangles();
        for tr in trs.iter() {
            for (i, pt) in self.pts.iter().enumerate() {
                if i == 0 {
                    continue;
                }
                if predicates::incircle(&self.pts[tr.tr0], &self.pts[tr.tr1], &self.pts[tr.tr2], pt)
                    > 0
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
        fmt.write_str("======== TRIANGULATION ========\n")?;
        fmt.write_str(&format!("# vertices: {:19}\n", self.number_of_vertices()))?;
        fmt.write_str(&format!("# triangles: {:18}\n", self.number_of_triangles()))?;
        fmt.write_str(&format!(
            "# on convex hull: {:13}\n",
            self.number_of_vertices_on_ch()
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
            "# on convex hull: {:13}\n",
            self.number_of_vertices_on_ch()
        ))?;
        for (i, _p) in self.pts.iter().enumerate() {
            fmt.write_str(&format!("{}: {:?}\n", i, self.stars[i]))?;
        }
        fmt.write_str("===============================\n")?;
        Ok(())
    }
}
