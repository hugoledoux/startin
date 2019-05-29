extern crate startin;

fn main() {
    let mut pts: Vec<Vec<f64>> = Vec::new();
    pts.push(vec![2.0, 2.0, 2.0]);
    pts.push(vec![4.0, 2.0, 12.5]);
    pts.push(vec![4.0, 5.0, 7.65]);
    pts.push(vec![2.0, 4.0, 21.0]);
    pts.push(vec![2.5, 2.5, 33.0]);
    pts.push(vec![3.0, 3.0, 33.0]);

    let mut dt = startin::Triangulation::new();
    dt.insert(&pts);

    println!("{}", dt);
    dt.remove(5);
    println!("{}", dt);

    // let re = dt.insert_one_pt(20.0, 30.0, 2.0);
    // match re {
    //     Ok(_v) => println!("Inserted new point"),
    //     Err(v) => println!("Duplicate of vertex #{}, not inserted", v),
    // }

    // println!("*****");
    // println!("Number of points in DT: {}", dt.number_of_vertices());
    // println!("Number of triangles in DT: {}", dt.number_of_triangles());

    // //-- print all the vertices
    // for (i, each) in dt.all_vertices().iter().enumerate() {
    //     // skip the first one, the infinite vertex
    //     if i > 0 {
    //         println!("#{}: ({:.3}, {:.3}, {:.3})", i, each[0], each[1], each[2]);
    //     }
    // }

    // //-- get the convex hull
    // let ch = dt.convex_hull();
    // println!("Convex hull: {:?}", ch);

    // //-- fetch triangle containing (x, y)
    // let re = dt.locate(50.0, 50.0);
    // if re.is_some() {
    //     let t = re.unwrap();
    //     println!("The triangle is {}", t);
    //     assert!(dt.is_triangle(&t));
    // } else {
    //     println!("Outside convex hull");
    // }
}
