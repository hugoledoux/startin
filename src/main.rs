
extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;
// use std::collections::vec::Vector;


#[derive(Debug, Deserialize)]
pub struct Point3d {
  x: f64,
  y: f64,
  z: f64,
}

impl Point3d {
  fn printme(&self)  {
    println!("POINT({:.3}, {:.3}, {:.3})", self.x, self.y, self.z);
  }
}

pub struct Triangulation {
  pts:   Vec<Point3d>,
  stars: Vec<u32>,
}

impl Triangulation {
  //-- new
  pub fn new() -> Triangulation {
    Triangulation {
      pts:   Vec::new(),
      stars: Vec::new(),
    }
  }
  //-- insertpt
  pub fn insertpt(&mut self, p: Point3d) {
    self.pts.push(p);
  }
  //-- number_pts
  pub fn number_pts(self) -> usize {
    self.pts.len()
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

  println!("{}", tr.number_pts());
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