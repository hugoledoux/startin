#[allow(dead_code)]
extern crate csv;
extern crate rustin;
extern crate serde;

use rustin::Point3d;
use std::error::Error;
use std::io;

// To run:
// $ ./rustin < ../../data/samples2.xyz
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

    let mut tr = rustin::Triangulation::new();
    for p in vec.into_iter() {
        // println!("{}", p);
        let (i, b) = tr.insert_one_pt(p);
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
