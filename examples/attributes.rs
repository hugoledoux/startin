use serde_json::json;

extern crate startin;

fn main() {
    // let mut pts: Vec<[f64; 3]> = Vec::new();
    // pts.push([20 as f64, 30.0, 0.0]);
    // pts.push([20 as f64, 30.0, 1.1]);
    // pts.push([120.0, 33.0, 12.5]);
    // pts.push([124.0, 222.0, 7.65]);
    // pts.push([20.0, 133.0, 21.0]);
    // pts.push([60.0, 60.0, 33.0]);

    let mut dt = startin::Triangulation::new();
    // dt.insert(&pts, startin::InsertionStrategy::AsIs);
    // dt.initialise_attributes();
    let _ = dt.insert_one_pt_with_attribute(20 as f64, 30.0, 0.0, json!(1.1));
    // let _ = dt.insert_one_pt_with_attribute(20 as f64, 30.0, 1.1, json!(1.1));
    // let _ = dt.insert_one_pt_with_attribute(120.0, 33.0, 12.5, json!(1.1));
    // let _ = dt.insert_one_pt_with_attribute(124.0, 222.0, 7.65, json!(1.1));
    // let _ = dt.insert_one_pt_with_attribute(20.0, 133.0, 21.0, json!(1.1));
    // let _ = dt.insert_one_pt_with_attribute(60.0, 60.0, 33.0, json!(1.1));

    println!("{}", dt);
    // println!("Number of points in DT: {}", dt.number_of_vertices());
    // println!("Number of triangles in DT: {}", dt.number_of_triangles());

    // //-- print all the vertices
    // for (i, each) in dt.all_vertices().iter().enumerate() {
    //     // skip the first one, the infinite vertex
    //     if i > 0 {
    //         println!("#{}: ({:.3}, {:.3}, {:.3})", i, each[0], each[1], each[2]);
    //     }
    // }

    // //-- insert a new vertex
    // let re = dt.insert_one_pt(22.2, 33.3, 4.4);
    // match re {
    //     Ok(_v) => println!("Inserted new point"),
    //     Err(v) => println!("Duplicate of vertex #{}, not inserted", v),
    // }

    // //-- some stats
    // println!("Number of points in DT: {}", dt.number_of_vertices());
    // println!("Number of triangles in DT: {}", dt.number_of_triangles());
}
