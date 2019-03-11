#[allow(dead_code)]
extern crate csv;
extern crate rustin;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use std::error::Error;
use std::io;

// To run:
// $ ./rustin < ../../data/samples2.xyz

#[derive(Debug, Deserialize)]
pub struct Point {
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

    let mut tr = rustin::Triangulation::new();
    for p in vec.into_iter() {
        // println!("{}", p);
        let (i, b) = tr.insert_one_pt(p.x, p.y, p.z);
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

fn read_xyz_file() -> Result<Vec<Point>, Box<Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .from_reader(io::stdin());
    let mut vpts: Vec<Point> = Vec::new();
    for result in rdr.deserialize() {
        let record: Point = result?;
        vpts.push(record);
    }
    Ok(vpts)
}
