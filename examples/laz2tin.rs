extern crate las;
extern crate startin;

use las::{Read, Reader};

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
    dt.set_jump_and_walk(true);

    for laspt in reader.points() {
        let p = laspt.unwrap();
        let _re = dt.insert_one_pt(p.x, p.y, p.z);
    }

    println!("Number of points in DT: {}", dt.number_of_vertices());
    println!("Number of triangles in DT: {}", dt.number_of_triangles());
    println!("bbox: {:?}", dt.get_bbox());

    // let pathout = "/Users/hugo/temp/out.obj";
    // println!("Writing OBJ file...");
    // let re = dt.write_obj(pathout.to_string(), false);
    // match re {
    //     Ok(_x) => println!("--> OBJ output saved to: {}", pathout),
    //     Err(_x) => println!("ERROR: path {} doesn't exist, abort.", pathout),
    // }
}
