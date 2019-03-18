// #[allow(dead_code)]
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

    let mut tr = rustin::Triangulation::new();
    // tr.set_snap_tolerance(0.1);
    tr.set_jump_and_walk(false);

    let mut duplicates = 0;
    for p in vec.into_iter() {
        let re = tr.insert_one_pt(p.x, p.y, p.z);
        match re {
            // Ok(_x) => println!("{:?}", tr),
            Ok(_x) => continue,
            Err(_e) => duplicates = duplicates + 1,
            // Err(e) => println!("Duplicate point! Not inserted {}", tr.get_point(e)),
        };
    }

    if duplicates > 0 {
        println!("Duplicates? {} of them.\n", duplicates);
    } else {
        println!("Duplicates? none.\n");
    }

    // println!("****** is Delaunay? ******");
    // println!("{}", tr.is_delaunay());
    // println!("**************************");

    // println!("Number of points in DT: {}", tr.number_of_vertices());
    // println!("Number of trianges in DT: {}", tr.number_of_triangles());
    // println!("{:?}", tr);
    println!("{}", tr);
    // let ch = tr.get_convex_hull();
    // println!("{:?}", ch);

    let pts = tr.get_vertices();
    println!("Size pts: {}", pts.len());

    //-- fetch triangle containing (x, y)
    let re = tr.locate(50.0, 50.0);
    if re.is_some() {
        let mut t = re.unwrap();
        println!("{}", t);
        println!("({})", tr.get_point(t.tr0));
        println!("({})", tr.get_point(t.tr1));
        println!("({})", tr.get_point(t.tr2));
        println!("is_triangle(): {}", tr.is_triangle(&t));
        t.tr2 = 999;
        println!("is_triangle(): {}", tr.is_triangle(&t));
    } else {
        println!("Outside convex hull");
    }

    // let stats = tr.stats_degree();
    // println!("stats: {}--{}--{}", stats.0, stats.1, stats.2);

    // tr.write_obj("/Users/hugo/temp/out.obj".to_string(), true)
    //     .unwrap();
}

fn read_xyz_file() -> Result<Vec<CSVPoint>, Box<Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .from_reader(io::stdin());
    let mut vpts: Vec<CSVPoint> = Vec::new();
    for result in rdr.deserialize() {
        let record: CSVPoint = result?;
        vpts.push(record);
    }
    Ok(vpts)
}
