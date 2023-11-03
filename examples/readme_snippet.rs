extern crate startin;

fn main() {
    let mut pts: Vec<(f64, f64, f64)> = Vec::new();
    pts.push((20.0, 30.0, 2.0));
    pts.push((120.0, 33.0, 12.5));
    pts.push((124.0, 222.0, 7.65));
    pts.push((20.0, 133.0, 21.0));
    pts.push((60.0, 60.0, 33.0));
    let mut dt = startin::Triangulation::new();
    dt.insert(pts, startin::InsertionStrategy::AsIs);
    println!("{}", dt);
    //-- print all the vertices
    for (i, each) in dt.all_vertices().iter().enumerate() {
        // skip the first one, the infinite vertex
        if i > 0 {
            println!("#{}: ({:.3}, {:.3}, {:.3})", i, each.0, each.1, each.2);
        }
    }
    //-- insert a new vertex
    let re = dt.insert_one_pt(22.2, 33.3, 4.4);
    match re {
        Ok(_v) => println!(
            "Inserted new point, now the DT has {} vertices",
            dt.number_of_vertices()
        ),
        Err(v) => println!("Duplicate of vertex #{}, not inserted", v),
    }
    //-- remove it
    match dt.remove(6) {
        Ok(num) => println!("Vertex deleted, now the DT has {} vertices", num),
        Err(why) => println!("!!! Deletion error: {:?}", why),
    }
    //-- get the convex hull
    let ch = dt.convex_hull();
    println!("Convex hull: {:?}", ch);
    //-- fetch triangle containing (x, y)
    match dt.locate(50.0, 50.0) {
        Ok(tr) => println!("The triangle is {}", tr),
        Err(why) => println!("Error: {:?}", why),
    }
    //-- interpolate with Laplace interpolation at 2 locations
    let locs = vec![[51.0, 22.0], [50.3, 19.9]];
    let interpolant = startin::interpolation::Laplace {};
    let zs = startin::interpolation::interpolate(&interpolant, &mut dt, &locs);
    for z in &zs {
        match z {
            Ok(value) => println!("z = {}", value),
            Err(why) => println!("Interplation impossible: {:?}", why),
        }
    }
    //-- save the triangulation in OBJ for debug purposes
    let _re = dt.write_obj("/home/elvis/tr.obj".to_string());
}
