extern crate startin;

fn main() {
    let mut dt = startin::Triangulation::new();

    //-- insert 5 points
    dt.insert_one_pt(20.0, 30.0, 2.0).unwrap();
    dt.insert_one_pt(120.0, 33.0, 12.5).unwrap();
    dt.insert_one_pt(124.0, 222.0, 7.65).unwrap();
    dt.insert_one_pt(20.0, 133.0, 21.0).unwrap();
    dt.insert_one_pt(60.0, 60.0, 33.0).unwrap();

    println!("Number of points in DT: {}", dt.number_of_vertices());
    println!("Number of triangles in DT: {}", dt.number_of_triangles());

    //-- print all the vertices
    for (i, each) in dt.all_vertices().iter().enumerate() {
        println!(
            "#{}: ({:.3}, {:.3}, {:.3})",
            (i + 1),
            each[0],
            each[1],
            each[2]
        );
    }

    //-- get the convex hull
    let ch = dt.convex_hull();
    println!("Convex hull: {:?}", ch);

    //-- fetch triangle containing (x, y)
    let re = dt.locate(50.0, 50.0);
    if re.is_some() {
        let t = re.unwrap();
        println!("The triangle is {}", t);
        assert!(dt.is_triangle(&t));
    } else {
        println!("Outside convex hull");
    }
}
