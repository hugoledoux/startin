
extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;
// use std::collections::vec::Vector;


#[derive(Debug, Deserialize)]
pub struct Point3d {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Point3d {
  fn printme(&self)  {
    println!("POINT({:.3}, {:.3}, {:.3})", self.x, self.y, self.z);
  }
  fn distance_squared(&self, p: Point3d) -> f64 {
      (p.x - self.x) * (p.x - self.x) 
    + (p.y - self.y) * (p.y - self.y) 
    + (p.z - self.z) * (p.z - self.z)
  }
}

pub struct Triangulation {
  pts:    Vec<Point3d>,
  stars:  Vec<u32>,
  tol:    f32,
}

impl Triangulation {
  //-- new
  pub fn new() -> Triangulation {
    //-- add point at infinity
    let mut v: Vec<Point3d> = Vec::new();
    v.push(Point3d{x: -1.0, y: -1.0, z: -1.0});
    Triangulation {
      pts:   v,
      stars: Vec::new(),
      tol: 0.001,
    }
  }

  //-- insertpt
  pub fn insertpt(&mut self, p: Point3d) {
    if self.pts.len() <= 2 {
      self.pts.push(p);
      println!("-1: inserted");
    }
    else if self.pts.len() == 3 {
      println!("-2: inserted");
      self.pts.push(p);
      //-- here build first triangle
    } 
    else {
      println!("-3:{}", self.pts.len());
      self.pts.push(p);
    }
    println!("{}", self.pts.len());
  }
  
  //-- number_pts
  pub fn number_pts(self) -> usize {
    (self.pts.len() - 1)
  }
}

//--------------------------------------------------

fn main() {
  let re = read_xyz_file();
  let vec = match re {
    Ok(vec) => vec,
    Err(error) => {
      panic!("Problem with the file {:?}", error)
    },
  };

  // println!("===TOTAL: {} points", re.len());
  // println!("{:?}", vec);
  dosmth(&vec);

  // for p in vec.iter() {
  //   p.printme();
  // }

  let mut tr = Triangulation::new();
  for p in vec.into_iter() {
    tr.insertpt(p);
  }  

  println!("Number of points in DT: {}", tr.number_pts());
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



fn dosmth(vpts: &Vec<Point3d>) {
  println!("===TOTAL: {} points", vpts.len());
  // println!("{:#?}", vpts);
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