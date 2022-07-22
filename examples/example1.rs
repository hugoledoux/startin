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
    dt.set_jump_and_walk(false);
    dt.use_robust_predicates(true);

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
    // println!("{}", dt.is_valid());
    // println!("**************************");

    // println!("Number of points in DT: {}", dt.number_of_vertices());
    // println!("Number of triangles in DT: {}", dt.number_of_triangles());
    // println!("{:?}", dt);
    println!("{}", dt);
    // let ch = dt.convex_hull();
    // println!("{:?}", ch);

    let pts = dt.all_vertices();
    println!("Size pts: {}", pts.len());

    println!("Vertex CH: {}", dt.is_vertex_convex_hull(0));

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

    let re = dt.adjacent_vertices_to_vertex(66);
    if re.is_ok() == true {
        for each in re.unwrap() {
            println!("Adjacent vertex {}", each);
        }
    } else {
        println!("Vertex does not exists.");
    }

    let trs = dt.incident_triangles_to_vertex(6).unwrap();

    let re = dt.adjacent_triangles_to_triangle(&trs[0]);
    if re.is_ok() == true {
        println!("Adjacent to: {}", &trs[0]);
        for tr in re.unwrap().iter() {
            println!("adj: {}", tr);
            // println!("is_triangle? {}", dt.is_triangle(&tr));
        }
    }

    // let stats = dt.statistics_degree();
    // println!("stats: {}--{}--{}", stats.0, stats.1, stats.2);

    let pathout = "/Users/hugo/temp/out.obj";
    println!("Writing OBJ file...");
    let re = dt.write_obj(pathout.to_string());
    match re {
        Ok(_x) => println!("--> OBJ output saved to: {}", pathout),
        Err(_x) => println!("ERROR: path {} doesn't exist, abort.", pathout),
    }

    // dt.write_obj(pathout.to_string(), false)
    //     .expect(&format!("Path {} doesn't exist, abort.", pathout));
    // println!("--> OBJ output saved to: {}", pathout);
}

fn read_xyz_file() -> Result<Vec<CSVPoint>, Box<dyn Error>> {
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
