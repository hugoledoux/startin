extern crate startin;

fn main() {
    let mut pts: Vec<[f64; 3]> = Vec::new();
    pts.push([0.0, 0.0, 1.0]);
    pts.push([10.0, 1.0, 2.0]);
    pts.push([10.0, 12.0, 3.0]);
    pts.push([0.0, 10.0, 4.0]);
    pts.push([5.0, 5.0, 4.]);

    let mut dt = startin::Triangulation::new();
    dt.insert(&pts, startin::InsertionStrategy::AsIs);

    println!("*****");
    println!("Number of points in DT: {}", dt.number_of_vertices());
    println!("Number of triangles in DT: {}", dt.number_of_triangles());

    println!("{:?}", dt.normal_vertex(2));
    println!("{:?}", dt.normal_vertex(5));
}
