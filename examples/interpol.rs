extern crate las;
extern crate startin;

use las::{Read, Reader};
use rand::distributions::{Distribution, Uniform};
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

    let b = header.bounds();
    println!(
        "({}, {}, {}) --> ({}, {}, {})",
        b.min.x, b.min.y, b.min.z, b.max.x, b.max.y, b.max.z
    );

    let mut dt = startin::Triangulation::new();

    let mut rng = thread_rng();
    let thin_factor = 10;
    for laspt in reader.points() {
        if rng.gen_ratio(1, thin_factor) == true {
            let p = laspt.unwrap();
            let _re = dt.insert_one_pt(p.x, p.y, p.z);
        }
    }

    println!("Number of points in DT: {}", dt.number_of_vertices());
    println!("Number of triangles in DT: {}", dt.number_of_triangles());
    println!("bbox: {:?}", dt.get_bbox());

    let rx = Uniform::from(b.min.x..b.max.x);
    let ry = Uniform::from(b.min.y..b.max.y);
    let mut locs = Vec::new();
    for _ in 0..10 {
        locs.push([rx.sample(&mut rng), ry.sample(&mut rng)]);
    }

    let idw = startin::interpolation::IDW {
        radius: 1.0,
        power: 2.0,
    };
    let re1 = startin::interpolation::interpolate(&idw, &mut dt, &locs);
    // println!("{:?}", re.len());

    let lap = startin::interpolation::Laplace {};
    let re2 = startin::interpolation::interpolate(&lap, &mut dt, &locs);
    // println!("{:?}", re.len());

    for i in 0..re1.len() {
        println!("{:?}--{:?}", re1[i], re2[i]);
    }
}
