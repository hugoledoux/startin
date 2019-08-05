// To run:
// $ ./example1 < ../../data/samples2.xyz

#![allow(dead_code)]

extern crate startin;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let re = read_xyz_file();
    // let vec = match re {
    //     Ok(vec) => vec,
    //     Err(error) => panic!("Problem with the file {:?}", error),
    // };

    let mut dt = startin::Triangulation::new();
    // dt.set_snap_tolerance(0.1);
    // dt.set_jump_and_walk(true);
    // dt.use_robust_predicates(true);

    let mut duplicates = 0;
    for (i, p) in re.into_iter().enumerate() {
        // if i == 1149569 {
        //     println!("checking validity");
        //     println!("is valid: {}", dt.is_valid());
        // }
        if i == 1149570 {
            println!("Oh!");
            // dt.is_valid_flat();
            // dt.is_valid_double_zero();
            // let ch = dt.convex_hull();
            // println!("{:?}", ch);
        }
        let re = dt.insert_one_pt(p[0], p[1], p[2]);
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

    println!("Number of points in DT: {}", dt.number_of_vertices());
    println!("Number of triangles in DT: {}", dt.number_of_triangles());
    // println!("{:?}", dt);
    println!("{}", dt);
}

fn read_xyz_file() -> Vec<Vec<f64>> {
    let tmpf = File::open("/Users/hugo/Dropbox/data/ahn3/o3.txt").unwrap();
    // let tmpf = File::open("/Users/hugo/Dropbox/data/ahn3/test.txt").unwrap();
    let file = BufReader::new(&tmpf);

    let mut pts: Vec<Vec<f64>> = Vec::new();

    for (num, line) in file.lines().enumerate() {
        if num != 0 {
            let l = line.unwrap();
            let v: Vec<f64> = l.split(' ').map(|s| s.parse().unwrap()).collect();
            // println!("{:?}", v);
            pts.push(v);
        }
    }
    pts
}
