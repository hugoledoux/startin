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

    let mut dt = rustin::Triangulation::new();
    // dt.set_snap_tolerance(0.1);
    dt.set_jump_and_walk(false);
    // dt.set_robust_predicates(false);

    let mut duplicates = 0;
    for p in vec.into_iter() {
        let re = dt.insert_one_pt(p.x, p.y, p.z);
        match re {
            // Ok(_x) => println!("{:?}", dt),
            Ok(_x) => continue,
            Err(_e) => duplicates = duplicates + 1,
            // Err(e) => println!("Duplicate point! Not inserted {}", dt.get_point(e)),
        };
    }

    if duplicates > 0 {
        println!("Duplicates? {} of them.\n", duplicates);
    } else {
        println!("Duplicates? none.\n");
    }

    // println!("****** is Delaunay? ******");
    // println!("{}", dt.is_delaunay());
    // println!("**************************");

    // println!("Number of points in DT: {}", dt.number_of_vertices());
    // println!("Number of trianges in DT: {}", dt.number_of_triangles());
    // println!("{:?}", dt);
    println!("{}", dt);
    // let ch = dt.get_convex_hull();
    // println!("{:?}", ch);

    // let pts = dt.get_vertices();
    // println!("Size pts: {}", pts.len());

    // //-- fetch triangle containing (x, y)
    // let re = dt.locate(50.0, 50.0);
    // if re.is_some() {
    //     let mut t = re.unwrap();
    //     println!("{}", t);
    //     // println!("({:?})", dt.get_point(t.tr0));
    //     // println!("({:?})", dt.get_point(t.tr1));
    //     // println!("({:?})", dt.get_point(t.tr2));
    //     println!("is_triangle(): {}", dt.is_triangle(&t));
    //     t.tr1 = 999;
    //     println!("is_triangle(): {}", dt.is_triangle(&t));
    // } else {
    //     println!("Outside convex hull");
    // }

    let stats = dt.stats_degree();
    println!("stats: {}--{}--{}", stats.0, stats.1, stats.2);

    // dt.write_obj("/Users/hugo/temp/out.obj".to_string(), false)
    //     .unwrap();
    // println!("--> OBJ output saved to: '/Users/hugo/temp/out.obj'");
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
