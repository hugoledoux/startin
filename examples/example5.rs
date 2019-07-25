extern crate startin;

fn main() {
    let mut pts: Vec<Vec<f64>> = Vec::new();
    pts.push(vec![0.0, 0.0, 12.5]);
    pts.push(vec![1.0, 0.0, 7.65]);
    pts.push(vec![1.1, 1.1, 33.0]);
    pts.push(vec![0.0, 1.0, 33.0]);
    pts.push(vec![0.5, 0.9, 33.0]);
    pts.push(vec![0.9, 0.5, 33.0]);
    pts.push(vec![0.67, 0.66, 33.0]);
    let mut dt = startin::Triangulation::new();
    dt.set_jump_and_walk(false);
    dt.insert(&pts);
    println!("{}", dt.printme(true));

    let _re = dt.remove(3);
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

    // let _re = dt.insert_one_pt(1.5, 1.5, 33.0);
    // println!("{}", dt.printme(true));
    // assert!(dt.is_valid());
    // dt.write_obj("/Users/hugo/temp/0.obj".to_string(), true);
}
