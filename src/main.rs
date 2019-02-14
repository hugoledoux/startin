// $ ./rustri < ../../samples2.xyz

// #![allow(dead_code)]

extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::fmt;
use std::io;

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
            println!("WALK TO TRIANGLE");
            let tr = self.walk(&p);
            println!("TEST FOR DISTANCE");
            println!("INSERT+FLIP");
            self.pts.push(p);
            self.stars.push([].to_vec());
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
        //-- find the first tr
        let mut tr = Triangle {
            tr0: 0,
            tr1: 0,
            tr2: 0,
        };
        let cur = self.cur;
        for i in self.stars[cur].iter() {
            if orient2d(&self.pts[cur], &self.pts[*i], &x) == -1 {
                tr.tr0 = cur;
                tr.tr1 = self.get_previous_star(&self.stars[cur], *i);
                tr.tr1 = *i;
            }
        }
        println!("startr: {}", tr);

        Triangle {
            tr0: 22,
            tr1: 8,
            tr2: 9,
        }
    }

    fn get_next_star(&self, s: &Vec<usize>, i: usize) -> usize {
        //-- get next vertex in a star
        if i == (s.len() - 1) {
            0
        } else {
            (i + 1)
        }
    }

    fn get_previous_star(&self, s: &Vec<usize>, i: usize) -> usize {
        //-- get previous vertex in a star
        if i == 0 {
            (s.len() - 1)
        } else {
            (i - 1)
        }
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
    }

    // println!("Number of points in DT: {}", tr.number_pts());
    println!("{}", tr);
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
    if re > 0.0 {
        return 1;
    } else if re == 0.0 {
        return 0;
    } else {
        return -1;
    }
}

fn dosmth(vpts: &Vec<Point3d>) {
    println!("===TOTAL: {} points", vpts.len());
    // println!("{:#?}", vpts); //-- to format with \n
    // println!("{:?}", vpts);
}

// fn main() {
//   let f = File::open("/Users/hugo/teaching/geo1015_material/hw/01/code_hw01/samples.xyz").unwrap();
//   let file = BufReader::new(&f);

//   let mut vpts: Vec<Point3d> = Vec::new();

//   for (num, line) in file.lines().enumerate() {
//     if num != 0 {
//       let l = line.unwrap();
//       let v: Vec<f32> = l
//         .split(' ')
//         .map(|s| s.parse().unwrap())
//         .collect();
//       // println!("{:?}", v);
//       let p = Point3d{x: v[0], y: v[1], z: v[2]};
//       vpts.push(p);
//       // println!("{}", l);
//     }
//   }

//   println!("{}", vpts.len());
//   let mut i = 0;
//   for each in vpts.iter() {
//     println!("#{} ({}, {}, {})",
//       i,
//       each.x,
//       each.y,
//       each.z
//     );
//     i += 1;
//   }

//   // for (num, line) in file.lines().enumerate() {
//   //   if (num != 0) && (num % 100 == 0)  {
//   //     let l = line.unwrap();
//   //     println!("{}", num);
//   //     // println!("{}", l);
//   //     let split = l.split_whitespace();
//   //     // std::io::
//   //     for each in split {
//   //       print!("{} ", each);
//   //     }
//   //     println!();
//   //   }
//   // }
// }
