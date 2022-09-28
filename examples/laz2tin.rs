extern crate las;
extern crate startin;

use las::{Read, Reader};
use rand::{thread_rng, Rng};

fn main() {
    let path = std::env::args()
        .skip(1)
        .next()
        .expect("Must provide a path to a LAS/LAZ file");
    let mut reader = Reader::from_path(path).expect("Wrong file name");

    let header = reader.header();
    println!("Reading LAS file version: {}", header.version());
    println!("{} points.", header.number_of_points());

    // let b = header.bounds();
    // println!(
    // "({}, {}, {}) --> ({}, {}, {})",
    // b.min.x, b.min.y, b.min.z, b.max.x, b.max.y, b.max.z
    // );

    let mut dt = startin::Triangulation::new();
    // dt.set_jump_and_walk(true);

    let mut rng = thread_rng();
    let thin_factor = 100;
    for laspt in reader.points() {
        if rng.gen_ratio(1, thin_factor) == true {
            let p = laspt.unwrap();
            let _re = dt.insert_one_pt(p.x, p.y, p.z);
        }
    }

    println!("Number of points in DT: {}", dt.number_of_vertices());
    println!("Number of triangles in DT: {}", dt.number_of_triangles());
    println!("bbox: {:?}", dt.get_bbox());

    for i in 100..150 {
        // println!("{}", i);
        let _re = dt.remove(i);
    }

    dt.collect_garbage();
    assert_eq!(dt.is_valid(), true);
    assert_eq!(dt.has_garbage(), false);

    println!("=={}", dt.number_of_removed_vertices());
    // dt.vertical_exaggeration(2.0);
    // let pathout = "/Users/hugo/temp/t1.ply";
    // println!("Writing PLY file...");
    // let re = dt.write_ply(pathout.to_string());
    // match re {
    //     Ok(_x) => println!("--> PLY output saved to: {}", pathout),
    //     Err(_x) => println!("ERROR: path {} doesn't exist, abort.", pathout),
    // }
}
