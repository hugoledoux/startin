extern crate startin;
use rand::prelude::*;

fn main() {
    let num_pts = 1000;
    let mut pts: Vec<[f64; 3]> = Vec::new();

    let mut rng = rand::thread_rng();
    for _i in 0..num_pts {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        let z: f64 = rng.gen();
        pts.push([x * 55555.0, y * 55555.0, z * 10.0]);
    }

    let mut dt = startin::Triangulation::new();
    dt.insert(&pts, startin::InsertionStrategy::AsIs);
    // println!("{}", dt.printme(false));

    for _i in 0..1000 {
        let mut x: f64 = rng.gen();
        let mut y: f64 = rng.gen();
        x *= 55555.0;
        y *= 55555.0;
        let nni1 = dt.interpolate_nni(x, y);
        let nni2 = dt.interpolate_nni_2(x, y);
        if nni1.is_ok() && nni2.is_ok() {
            let v = (nni1.as_ref().unwrap() - nni2.as_ref().unwrap()).abs();
            // println!("({}, {})", x, y);
            // println!("{} -- {}", nni1.unwrap(), nni2.unwrap());
            println!("{}", v);
            assert!((nni1.unwrap() - nni2.unwrap()).abs() < 0.000001);
        }
    }
}
