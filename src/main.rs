// $ ./rustri < ../../samples2.xyz

// #![allow(dead_code)]

extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;


#[derive(Debug, Deserialize)]
pub struct Point3d {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}


impl Point3d {
  fn printme(&self) -> String {
    format!("POINT({:.3}, {:.3}, {:.3})", self.x, self.y, self.z)
  }
  fn square_2d_distance(&self, p: &Point3d) -> f64 {
      (p.x - self.x) * (p.x - self.x) 
    + (p.y - self.y) * (p.y - self.y) 
  }
}

pub struct Triangulation {
  pts:    Vec<Point3d>,
  stars:  Vec<Vec<usize>>,
  tol:    f64,
}

impl Triangulation {
  //-- new
  pub fn new() -> Triangulation {
    //-- add point at infinity
    let mut v: Vec<Point3d> = Vec::new();
    v.push(Point3d{x: 9999999.0, y: 9999999.0, z: 9999999.0});
    let mut s: Vec<Vec<usize>> = Vec::new();
    s.push([].to_vec());
    Triangulation {
      pts:   v,
      stars: s,
      tol: 0.001,
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
      println!("insert point");
      if self.pts.len() == 4 {
        println!("CREATE First triangle here");
        if orient2d(&self.pts[1], &self.pts[2], &self.pts[3]) == 1 {
          self.stars[1].push(0);
        }
        else {
          println!("CW");
        }

      }
      return (self.pts.len() - 1, true);
    }
    else {
      println!("WALK TO TRIANGLE");
      println!("TEST FOR DISTANCE");
      println!("INSERT+FLIP");
      self.pts.push(p);
      self.stars.push([].to_vec());
    }
    // println!("{}", self.pts.len());
    return (self.pts.len(), true);
  }
  
  //-- number_pts
  pub fn number_pts(&self) -> usize {
    (self.pts.len() - 1)
  }

  pub fn printme(&self) -> String {
    let mut s = String::from("=== TRIANGULATION ===\n");
    s.push_str(&format!("#pts: {}\n", self.number_pts()));
    for (i, p) in self.pts.iter().enumerate() {
      s.push_str(&format!("{}: {:?}\n", i, self.stars[i]));
      // s.push_str(&p.printme());
      // s.push_str("\n");
    }
    s.push_str("===============\n");
    s
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
  // dosmth(&vec);

  // for (i, p) in vec.iter().enumerate() {
  //   println!("#{}: {}", i, p.printme());
  // }

  let mut tr = Triangulation::new();
  for p in vec.into_iter() {
    let (i, b) = tr.insert_one_pt(p);
    if b == false {
      println!("Duplicate point ({})", i);
    }
    // println!("{}", tr.printme());
  }  

  // println!("Number of points in DT: {}", tr.number_pts());
  println!("{}", tr.printme());
  // println!("{:?}", tr);
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
  let re:  f64 = ((a.x-c.x) * (b.y - c.y)) - ((a.y - c.y) * (b.x - c.x));
  if re > 0.0 {
    return 1;
  }
  else if re == 0.0 {
    return 0
  }
  else {
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