extern crate startin;

fn main() {
    let mut pts: Vec<[f64; 3]> = Vec::new();
    pts.push([0.0, 0.0, 12.5]);
    pts.push([1.0, 0.0, 7.65]);
    pts.push([1.1, 1.1, 33.0]);
    pts.push([0.0, 1.0, 33.0]);
    pts.push([0.5, 0.9, 33.0]);
    pts.push([0.9, 0.5, 33.0]);
    pts.push([0.67, 0.66, 33.0]);
    let mut dt = startin::Triangulation::new();
    dt.set_jump_and_walk(false);
    dt.insert(&pts, startin::InsertionStrategy::AsIs);
    println!("{}", dt.printme(true));

    // let _re = dt.remove(3);
    // println!("{}", dt.printme(true));

    println!("is 3 removed {}", dt.is_vertex_removed(3));
    // dt.insert_one_pt(1.1, 2.2, 3.3);
    // println!("is 3 removed {}", dt.is_vertex_removed(3));
    // println!("is 4 removed {}", dt.is_vertex_removed(4));

    assert!(dt.is_valid());

    let a = dt.get_point(3);
    if a.is_some() == true {
        println!("point {:?}", a.unwrap());
    }

    let re = dt.locate(50.0, 50.0);
    match re {
        Some(x) => println!("Triangle: {}", x),
        None => println!("No triangle found, outside of the CH."),
    }

    let re = dt.closest_point(1.1, 1.11);
    match re {
        Some(x) => println!("Point: {}", x),
        None => println!("Outside of the CH."),
    }

    // let _re = dt.insert_one_pt(1.5, 1.5, 33.0);
    // println!("{}", dt.printme(true));
    // assert!(dt.is_valid());
    // dt.write_obj("/Users/hugo/temp/0.obj".to_string(), true);
}
