extern crate startin;
use rand::prelude::*;

fn main() {
    let num_pts = 1000;
    let mut pts: Vec<Vec<f64>> = Vec::new();

    let mut rng = rand::thread_rng();
    for _i in 0..num_pts {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        pts.push(vec![x * 100.0, y * 100.0, 2.0]);
    }

    let mut dt = startin::Triangulation::new();
    dt.set_jump_and_walk(false);
    dt.insert(&pts);
    println!("{}", dt.printme(false));

    //-- delete 5 vertices on convex hull
    let mut total: usize = 0;
    loop {
        let j: usize = rng.gen_range(1, num_pts);
        if dt.is_vertex_convex_hull(j) == true {
            let re = dt.remove(j);
            if re.is_err() == true {
                println!("!!!Error: {:?}", re.unwrap_err());
            }
            total = total + 1;
            if total == 10 {
                break;
            }
        }
    }

    // //-- insert 50 vertices randomly
    // for _i in 0..50 {
    //     let x: f64 = rng.gen();
    //     let y: f64 = rng.gen();
    //     let _re = dt.insert_one_pt(x, y, 1.1);
    // }
    // println!("{}", dt.printme(false));

    println!("Is Delaunay?: {}", dt.is_valid());
    println!("# vertices {}", dt.number_of_vertices());
    println!("# triangles {}", dt.number_of_triangles());
}
