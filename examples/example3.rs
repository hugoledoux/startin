extern crate startin;

fn main() {
    let mut pts: Vec<Vec<f64>> = Vec::new();

    let mut dt = startin::Triangulation::new();
    dt.set_jump_and_walk(false);
    
    dt.insert_one_pt(1.1, 1.07, 12.5);
    dt.insert_one_pt(11.0, 1.02, 7.65);
    dt.insert_one_pt(11.05, 11.1, 33.0);
    dt.insert_one_pt(1.0, 11.0, 21.0);

    println!("{}", dt.printme(true));
    println!("Is Delaunay?: {}", dt.is_valid());
    println!("# vertices {}", dt.number_of_vertices());
    println!("# triangles {}", dt.number_of_triangles());
    
    // let c = dt.closest_point(11.0, 11.0);
    // let re = dt.remove(c.unwrap());
    let re = dt.remove(3);
    if re.is_err() == true {
        println!("!!!Error: {:?}", re.unwrap_err());
    }

    println!("{}", dt.printme(true));
    println!("Is Delaunay?: {}", dt.is_valid());
    println!("# vertices {}", dt.number_of_vertices());
    println!("# triangles {}", dt.number_of_triangles());
    
    dt.insert_one_pt(11.0, 11.0, 22.2);

    println!("{}", dt.printme(true));
    println!("Is Delaunay?: {}", dt.is_valid());
    println!("# vertices {}", dt.number_of_vertices());
    println!("# triangles {}", dt.number_of_triangles());
}
