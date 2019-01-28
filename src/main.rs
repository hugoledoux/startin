
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;


struct Point3d {
    x: f32,
    y: f32,
    z: f32,
}


fn main() {   
  let f = File::open("/Users/hugo/teaching/geo1015_material/hw/01/code_hw01/samples.xyz").unwrap();
  let file = BufReader::new(&f);

  let mut vpts: Vec<Point3d> = Vec::new();

  for (num, line) in file.lines().enumerate() {
    if num != 0 {
      let l = line.unwrap();
      let v: Vec<f32> = l
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
      // println!("{:?}", v);
      let p = Point3d{x: v[0], y: v[1], z: v[2]};
      vpts.push(p);
      // println!("{}", l);
    } 
  }

  println!("{}", vpts.len());
  let mut i = 0;
  for each in vpts.iter() {
    println!("#{} ({}, {}, {})", 
      i, 
      each.x, 
      each.y, 
      each.z
    );
    i += 1;
  }

  // for (num, line) in file.lines().enumerate() {
  //   if (num != 0) && (num % 100 == 0)  {
  //     let l = line.unwrap();
  //     println!("{}", num);
  //     // println!("{}", l);
  //     let split = l.split_whitespace();
  //     // std::io::
  //     for each in split {
  //       print!("{} ", each);
  //     }
  //     println!();
  //   }
  // }           
}