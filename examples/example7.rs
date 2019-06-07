extern crate startin;
use rand::prelude::*;

fn main() {
    let mut found = false;
    loop {
        if found {
            break;
        }
        let num_pts = 20;
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
        // println!("{}", dt.printme(false));

        loop {
            let j: usize = rng.gen_range(1, num_pts);
            if dt.is_vertex_convex_hull(j) == true {
                let _re = dt.remove(j);
                if dt.is_valid() == false {
                    for p in pts {
                        println!("{} {} {}", p[0], p[1], p[2]);
                        // s.push_str(&format!("\t{:?}\n", self.stars[i].pt));
                    }
                    println!("vertex === {}", j);
                    found = true;
                }
                break;
            }
        }

        // //-- insert 50 vertices randomly
        // for _i in 0..50 {
        //     let x: f64 = rng.gen();
        //     let y: f64 = rng.gen();
        //     let _re = dt.insert_one_pt(x, y, 1.1);
        // }
        // println!("{}", dt.printme(false));

        // println!("Is Delaunay?: {}", dt.is_valid());
        // println!("# vertices {}", dt.number_of_vertices());
        // println!("# triangles {}", dt.number_of_triangles());
    }
}
