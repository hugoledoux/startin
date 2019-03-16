mod geom;

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
    snaptol: f64,
    cur: usize,
    is_init: bool,
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
            snaptol: 0.001,
            cur: 0,
            is_init: false,
        }
    }

    fn insert_one_pt_init_phase(&mut self, p: Point3d) -> Result<usize, usize> {
        for (i, pi) in self.pts.iter().enumerate() {
            if pi.square_2d_distance(&p) <= (self.snaptol * self.snaptol) {
                return Err(i);
            }
        }
        //-- add point to Triangulation and create its empty star
        self.pts.push(p);
        self.stars.push([].to_vec());
        //-- form the first triangles (finite + infinite)
        let l = self.pts.len();
        if l >= 4 {
            let a = l - 3;
            let b = l - 2;
            let c = l - 1;
            let re = geom::orient2d(&self.pts[a], &self.pts[b], &self.pts[c]);
            if re == 1 {
                println!("init: ({},{},{})", a, b, c);
                let mut v = vec![a, c, b];
                self.stars[0].append(&mut v);
                v = vec![0, b, c];
                self.stars[a].append(&mut v);
                v = vec![0, c, a];
                self.stars[b].append(&mut v);
                v = vec![0, a, b];
                self.stars[c].append(&mut v);
                self.is_init = true;
            } else if re == -1 {
                println!("init: ({},{},{})", a, c, b);
                let mut v = vec![a, b, c];
                self.stars[0].append(&mut v);
                v = vec![0, c, b];
                self.stars[a].append(&mut v);
                v = vec![0, a, c];
                self.stars[b].append(&mut v);
                v = vec![0, b, a];
                self.stars[c].append(&mut v);
                self.is_init = true;
            }
        }
        self.cur = l - 1;
        if self.is_init == true {
            //-- insert the previous vertices in the dt
            for j in 1..(l - 3) {
                let tr = self.walk(&self.pts[j]);
                println!("found tr: {}", tr);
                self.flip13(j, &tr);
                self.update_dt(j);
            }
        }
        Ok(self.cur)
    }

    //-- insert_one_pt
    pub fn insert_one_pt(&mut self, px: f64, py: f64, pz: f64) -> Result<usize, usize> {
        let p = Point3d {
            x: px,
            y: py,
            z: pz,
        };
        println!("-->{}", p);

        if self.is_init == false {
            return self.insert_one_pt_init_phase(p);
        }

        //-- walk
        println!("Walking");
        let tr = self.walk(&p);
        println!("STARTING TR: {}", tr);
        if p.square_2d_distance(&self.pts[tr.tr0]) < (self.snaptol * self.snaptol) {
            return Err(tr.tr0);
        }
        if p.square_2d_distance(&self.pts[tr.tr1]) < (self.snaptol * self.snaptol) {
            return Err(tr.tr1);
        }
        if p.square_2d_distance(&self.pts[tr.tr2]) < (self.snaptol * self.snaptol) {
            return Err(tr.tr2);
        }
        self.pts.push(p);
        self.stars.push([].to_vec());
        let pi = self.pts.len() - 1;

        //-- flip13()
        self.flip13(pi, &tr);
        //-- update_dt()
        self.update_dt(pi);

        self.cur = self.pts.len() - 1;
        Ok(self.pts.len() - 1)
    }

    fn update_dt(&mut self, pi: usize) {
        println!("--> Update DT");
        let mut mystack: Vec<Triangle> = Vec::new();
        mystack.push(Triangle {
            tr0: pi,
            tr1: self.stars[pi][0],
            tr2: self.stars[pi][1],
        });
        mystack.push(Triangle {
            tr0: pi,
            tr1: self.stars[pi][1],
            tr2: self.stars[pi][2],
        });
        mystack.push(Triangle {
            tr0: pi,
            tr1: self.stars[pi][2],
            tr2: self.stars[pi][0],
        });

        loop {
            let tr = match mystack.pop() {
                None => break,
                Some(x) => x,
            };
            let opposite = self.get_opposite_vertex(&tr);
            println!("stacked: {} {}", tr, opposite);

            if tr.is_infinite() == true {
                let mut a: i8 = 0;
                if tr.tr0 == 0 {
                    a = geom::orient2d(&self.pts[opposite], &self.pts[tr.tr1], &self.pts[tr.tr2]);
                } else if tr.tr1 == 0 {
                    a = geom::orient2d(&self.pts[tr.tr0], &self.pts[opposite], &self.pts[tr.tr2]);
                } else if tr.tr2 == 0 {
                    a = geom::orient2d(&self.pts[tr.tr0], &self.pts[tr.tr1], &self.pts[opposite]);
                }
                println!("TODO: INCIRCLE FOR INFINITY {}", a);
                if a > 0 {
                    println!("FLIPPED0 {} {}", tr, opposite);
                    let (ret0, ret1) = self.flip(&tr, opposite);
                    mystack.push(ret0);
                    mystack.push(ret1);
                }
            } else {
                if opposite == 0 {
                    //- if insertion on CH then break the edge, otherwise do nothing
                    if geom::orient2d(&self.pts[tr.tr0], &self.pts[tr.tr1], &self.pts[tr.tr2]) == 0
                    {
                        println!("FLIPPED1 {} {}", tr, 0);
                        let (ret0, ret1) = self.flip(&tr, 0);
                        mystack.push(ret0);
                        mystack.push(ret1);
                    }
                } else {
                    if geom::incircle(
                        &self.pts[tr.tr0],
                        &self.pts[tr.tr1],
                        &self.pts[tr.tr2],
                        &self.pts[opposite],
                    ) > 0
                    {
                        println!("FLIPPED2 {} {}", tr, opposite);
                        let (ret0, ret1) = self.flip(&tr, opposite);
                        mystack.push(ret0);
                        mystack.push(ret1);
                    }
                }
            }
        }
    }

    fn flip13(&mut self, pi: usize, tr: &Triangle) {
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
    }

    pub fn get_point(&self, i: usize) -> Point3d {
        let p = &self.pts[i];
        Point3d {
            x: p.x,
            y: p.y,
            z: p.z,
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

    // Get convex hull, oriented CCW
    // as a list of vertices (first != last)
    pub fn get_convex_hull(&self) -> Vec<usize> {
        let mut re: Vec<usize> = Vec::new();
        for x in self.stars[0].iter().rev() {
            re.push(*x);
        }
        re
    }

    pub fn number_of_vertices_on_convex_hull(&self) -> usize {
        //-- number of finite vertices on the boundary of the convex hull
        if self.is_init == false {
            return 0;
        }
        return self.stars[0].len();
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
        println!("cur: {}", cur);
        //-- 1. find a finite triangle
        // TODO: necessary?
        tr.tr0 = cur;
        if self.stars[cur].len() < 3 {
            tr.tr1 = self.stars[cur][0];
            tr.tr2 = self.stars[cur][1];
            return tr;
        }
        if self.stars[cur][0] == 0 {
            tr.tr1 = self.stars[cur][1];
            tr.tr2 = self.stars[cur][2];
        } else {
            tr.tr1 = self.stars[cur][0];
            tr.tr2 = self.stars[cur][1];
        }

        //-- 2. order it such that tr0-tr1-x is CCW
        if geom::orient2d(&self.pts[tr.tr0], &self.pts[tr.tr1], &x) == -1 {
            if geom::orient2d(&self.pts[tr.tr1], &self.pts[tr.tr2], &x) != -1 {
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
            if geom::orient2d(&self.pts[tr.tr1], &self.pts[tr.tr2], &x) != -1 {
                if geom::orient2d(&self.pts[tr.tr2], &self.pts[tr.tr0], &x) != -1 {
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
                if geom::incircle(&self.pts[tr.tr0], &self.pts[tr.tr1], &self.pts[tr.tr2], pt) > 0 {
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
        for (i, _p) in self.pts.iter().enumerate() {
            fmt.write_str(&format!("{}: {:?}\n", i, self.stars[i]))?;
        }
        fmt.write_str("===============================\n")?;
        Ok(())
    }
}
