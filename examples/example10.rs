// To run:
// $ ./example10 < ../../data/samples2.xyz

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
    dt.set_snap_tolerance(0.00000000000001);
    dt.set_jump_and_walk(false);
    dt.use_robust_predicates(true);

    // let mut duplicates = 0;
    // for p in vec.into_iter() {
    //     let re = dt.insert_one_pt(p[0], p[1], p[2]);
    //     match re {
    //         // Ok(_x) => println!("{:?}", dt),
    //         Ok(_x) => continue,
    //         Err(_e) => duplicates = duplicates + 1,
    //         // Err(e) => println!("Duplicate point! Not inserted {}", dt.get_point(e)),
    //     };
    // }
    // if duplicates > 0 {
    //     println!("Duplicates? {} of them.\n", duplicates);
    // } else {
    //     println!("Duplicates? none.\n");
    // }

    let _re = dt.insert(&vec, startin::InsertionStrategy::AsIs);
    // let _re = dt.insert(&vec, Some(vec![434366.0, 19722.0, 900289.0, 337914.0]));
    println!("{}", dt);
}

fn read_xyz_file() -> Result<Vec<[f64; 3]>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .from_reader(io::stdin());
    let mut vpts: Vec<[f64; 3]> = Vec::new();
    for result in rdr.deserialize() {
        let record: CSVPoint = result?;
        if record.z != -9999.0 {
            vpts.push([record.x, record.y, record.z]);
        }
    }
    Ok(vpts)
}
