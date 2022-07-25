// To run:
// $ ./example1 < ../../data/samples2.xyz

#![allow(dead_code)]

extern crate csv;
extern crate serde;
extern crate startin;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;

#[derive(Debug, Deserialize)]
pub struct CSVPoint {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

fn main() {
    let re = read_xyz_file();
    let vec = match re {
        Ok(vec) => vec,
        Err(error) => panic!("Problem with the file {:?}", error),
    };

    let mut dt = startin::Triangulation::new();
    // dt.set_snap_tolerance(0.1);
    // dt.set_jump_and_walk(false);
    dt.use_robust_predicates(true);

    for p in &vec {
        let _re = dt.insert_one_pt(p[0], p[1], p[2]);
    }

    // println!("****** is Delaunay? ******");
    // println!("{}", dt.is_valid());
    // println!("**************************");

    // println!("Number of points in DT: {}", dt.number_of_vertices());
    // println!("Number of triangles in DT: {}", dt.number_of_triangles());
    // println!("{:?}", dt);
    println!("{}", dt);
    // let ch = dt.convex_hull();
    // println!("{:?}", ch);
}

fn read_xyz_file() -> Result<Vec<[f64; 3]>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .from_reader(io::stdin());
    let mut vpts: Vec<[f64; 3]> = Vec::new();
    for result in rdr.deserialize() {
        let record: CSVPoint = result?;
        vpts.push([record.x, record.y, record.z]);
    }
    Ok(vpts)
}
