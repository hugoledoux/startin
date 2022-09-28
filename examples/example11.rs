extern crate startin;

fn main() {
    let mut pts: Vec<[f64; 3]> = Vec::new();
    pts.push([1.1, 1.07, 12.5]);
    pts.push([11.0, 1.02, 7.65]);
    pts.push([11.05, 11.1, 33.0]);
    pts.push([1.0, 11.0, 21.0]);
    pts.push([9.0, 5.0, 21.0]);
    pts.push([12.0, 5.1, 21.0]);
    pts.push([8.0, 8.0, 21.0]);
    pts.push([12.0, 8.1, 21.0]);
    pts.push([4.0, 5.15, 33.0]);

    let mut dt = startin::Triangulation::new();
    dt.insert(&pts, startin::InsertionStrategy::AsIs);

    let mut _re = dt.remove(7);
    _re = dt.remove(2);

    // assert_eq!(8, dt.number_of_vertices());
    // assert_eq!(8, dt.number_of_triangles());

    println!("{}", dt.printme(false));
    dt.collect_garbage();
    println!("{}", dt.printme(false));

    assert_eq!(dt.is_valid(), true);
    let _re = dt.write_ply("/Users/hugo/temp/t.ply".to_string());
}
