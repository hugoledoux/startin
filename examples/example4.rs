extern crate startin;
use rand::prelude::*;

fn main() {
    let mut pts: Vec<Vec<f64>> = Vec::new();

    let mut rng = rand::thread_rng();
    for _i in 0..100 {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        pts.push(vec![x, y, 2.0]);
    }

    let mut dt = startin::Triangulation::new();
    dt.set_jump_and_walk(false);
    dt.insert(&pts);
    println!("{}", dt.printme(false));

    //-- delete 50 vertices randomly
    for _i in 0..50 {
        let j: usize = rng.gen_range(1, 99);
        let re = dt.remove(j);
        if re.is_err() == true {
            println!("!!!Error: {:?}", re.unwrap_err());
        }
    }

    //-- insert 50 vertices randomly
    for _i in 0..50 {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        let _re = dt.insert_one_pt(x, y, 1.1);
    }
    println!("{}", dt.printme(false));

    println!("Is Delaunay?: {}", dt.is_valid());
    println!("# vertices {}", dt.number_of_vertices());
    println!("# triangles {}", dt.number_of_triangles());
}
