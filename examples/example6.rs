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
    let thin_factor = 1;
    for laspt in reader.points() {
        if rng.gen_ratio(1, thin_factor) == true {
            let p = laspt.unwrap();
            let _re = dt.insert_one_pt(p.x, p.y, p.z);
        }
    }

    println!("Number of points in DT: {}", dt.number_of_vertices());
    println!("Number of triangles in DT: {}", dt.number_of_triangles());
    println!("bbox: {:?}", dt.get_bbox());

    //-- interpolations

    let rx = Uniform::from(b.min.x..b.max.x);
    let ry = Uniform::from(b.min.y..b.max.y);
    let mut locs = Vec::new();
    for _ in 0..10000 {
        locs.push([rx.sample(&mut rng), ry.sample(&mut rng)]);
    }

    // {
    //     let re_laplace = dt.interpolate_laplace_2(&locs);
    //     let re_nni = dt.interpolate_nni_2(&locs, true);
    //     for (i, _each) in re_laplace.iter().enumerate() {
    //         if re_laplace[i].is_ok() && re_nni[i].is_ok() {
    //             println!(
    //                 "{}",
    //                 (re_laplace[i].as_ref().unwrap() - re_nni[i].as_ref().unwrap()).abs()
    //             );
    //         }
    //     }
    // }

    // {
    //     let now = Instant::now();
    //     {
    //         let mut re = Vec::new();
    //         for loc in &locs {
    //             re.push(dt.interpolate_nn(loc[0], loc[1]));
    //         }
    //         let re2 = dt.interpolate_nn_2(&locs);
    //         for (i, _each) in re.iter().enumerate() {
    //             if re[i].is_ok() && re2[i].is_ok() {
    //                 assert_approx_eq!(re[i].as_ref().unwrap(), re2[i].as_ref().unwrap(), 1e-3f64);
    //             } else {
    //                 assert_eq!(re[i].as_ref(), re2[i].as_ref());
    //             }
    //         }
    //     }
    //     let elapsed = now.elapsed();
    //     println!("nni: {:.2?}", elapsed);
    // }
    // {
    //     let now = Instant::now();
    //     {
    //         let re = dt.interpolate_nni_2(&locs, true);
    //     }
    //     let elapsed = now.elapsed();
    //     println!("nni_2: {:.2?}", elapsed);
    // }

    // {
    //     let now = Instant::now();
    //     {
    //         let re = dt.interpolate_nni_2(&locs, false);
    //     }
    //     let elapsed = now.elapsed();
    //     println!("w/o: {:.2?}", elapsed);
    // }
    // {
    //     let now = Instant::now();
    //     {
    //         let re = dt.interpolate_nni_2(&locs, true);
    //     }
    //     let elapsed = now.elapsed();
    //     println!("with: {:.2?}", elapsed);
    // }

    // println!("{:?}", re);

    //-- interpolate
    // let locs = vec![[51.0, 42.0]];
    // let re = dt.interpolate_nn_2(&locs);
    // let re = dt.interpolate_nn_2(&vec![[51.0, 42.0]]);
    // for each in &re {
    //     match each {
    //         Ok(z) => println!("z = {}", z),
    //         Err(why) => println!("Interplation impossible: {:?}", why),
    //     }
    // }

    // let re = dt.interpolate_tin_linear_2(&vec![[51.0, 52.0]]);
    // for each in &re {
    //     match each {
    //         Ok(z) => println!("z = {}", z),
    //         Err(why) => println!("Interplation impossible: {:?}", why),
    //     }
    // }

    // let a = dt.interpolate_laplace(51.0, 48.0);
    // println!("{:?}", a);
    // let b = dt.interpolate_laplace_2(&vec![[51.0, 48.0]]);
    // println!("{:?}", b);

    //-- save the triangulation in geojson for debug purposes
    //-- do not attempt on large DT
    // let _re = dt.write_geojson("/home/elvis/tr.geojson".to_string());
}
