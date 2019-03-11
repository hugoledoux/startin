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
    let mut duplicates = 0;
    for p in vec.into_iter() {
        let re = tr.insert_one_pt(p.x, p.y, p.z);
        match re {
            Ok(_x) => continue,
            Err(_e) => duplicates = duplicates + 1,
            // Err(e) => println!("Duplicate point! Not inserted {}", tr.get_point(e)),
        };
    }
    println!("Duplicates? yup: {} of them", duplicates);

    println!("****** is Delaunay? ******");
    println!("{}", tr.is_delaunay());
    println!("**************************");

    // println!("Number of points in DT: {}", tr.number_of_vertices());
    // println!("Number of trianges in DT: {}", tr.number_of_triangles());
    // println!("{:?}", tr);
    // println!("{}", tr);
    // let ch = tr.get_convex_hull();
    // println!("{:?}", ch);

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
