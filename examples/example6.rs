extern crate startin;

fn main() {
    let mut pts: Vec<[f64; 3]> = Vec::new();

    pts.push([57.42265069953759, 11.302054173605036, 2.0]);
    pts.push([92.84366030693992, 43.6916136057666, 2.0]);
    pts.push([1.4236330964329302, 64.90700146602387, 2.0]);
    pts.push([13.285710267579498, 50.189815660581175, 2.0]);
    pts.push([15.111123388743696, 78.78533829573448, 2.0]);

    let mut dt = startin::Triangulation::new();
    dt.insert(&pts, startin::InsertionStrategy::BBox);
    println!("{}", dt);

    for (i, each) in dt.all_vertices().iter().enumerate() {
        println!("#{}: ({:.3}, {:.3}, {:.3})", i, each[0], each[1], each[2]);
    }

    for (i, each) in dt.all_triangles().iter().enumerate() {
        println!("#{}: ({})", i, each);
    }

    println!("no removed: {}", dt.number_of_removed_vertices());

    // let _re = dt.write_obj("/Users/hugo/temp/0.obj".to_string(), true);

    // let _re = dt.remove(11);
    // println!("{}", dt.printme(true));

    // assert!(dt.is_valid());
    // let _re = dt.write_obj("/Users/hugo/temp/1.obj".to_string(), true);

    // let _re = dt.insert_one_pt(1.5, 1.5, 33.0);
    // println!("{}", dt.printme(true));
    // assert!(dt.is_valid());
}
