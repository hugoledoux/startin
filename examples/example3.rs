extern crate startin;
use rand::prelude::*;

fn main() {
    let mut pts: Vec<Vec<f64>> = Vec::new();

    pts.push(vec![1.1, 1.07, 12.5]);
    pts.push(vec![11.0, 1.02, 7.65]);
    pts.push(vec![11.05, 11.1, 33.0]);
    pts.push(vec![1.0, 11.0, 21.0]);
    pts.push(vec![9.0, 5.0, 21.0]);
    pts.push(vec![12.0, 5.1, 21.0]);
    pts.push(vec![8.0, 8.0, 21.0]);
    pts.push(vec![12.0, 8.1, 21.0]);

    pts.push(vec![4.0, 5.15, 33.0]);

    let mut dt = startin::Triangulation::new();
    dt.set_jump_and_walk(false);
    dt.insert(&pts);
    println!("{}", dt.printme());

    let re = dt.remove(7);
    if re.is_err() == true {
        println!("!!!Error: {:?}", re.unwrap_err());
    }
    println!("{}", dt.printme());

    println!("Is Delaunay?: {}", dt.is_valid());
    println!("# vertices {}", dt.number_of_vertices());
    println!("# triangles {}", dt.number_of_triangles());
}
