extern crate startin;

fn main() {
    let mut pts: Vec<Vec<f64>> = Vec::new();

    pts.push(vec![0.0, 0.0, 12.5]);
    pts.push(vec![0.0, 0.0, 11.11]);
    pts.push(vec![1.0, 0.0, 22.22]);
    pts.push(vec![1.0, 1.0, 33.33]);
    pts.push(vec![0.0, 1.0, 44.0]);
    pts.push(vec![0.5, 0.49, 44.0]);
    pts.push(vec![0.45, 0.69, 44.0]);
    pts.push(vec![0.65, 0.49, 44.0]);
    pts.push(vec![0.75, 0.29, 44.0]);
    pts.push(vec![1.5, 1.49, 44.0]);
    pts.push(vec![0.6, 0.2, 44.0]);
    pts.push(vec![0.45, 0.4, 44.0]);
    pts.push(vec![0.1, 0.8, 44.0]);

    let mut dt = startin::Triangulation::new();
    dt.insert(&pts);

    // let re = dt.interpolate_nn(2., 1.);
    // let re = dt.interpolate_nn(11., 11.);
    // let re = dt.interpolate_tin_linear(2., 1.);
    let re = dt.interpolate_laplace(1., 1.);
    println!("{:?}", re);

    let re = dt.interpolate_laplace(111., 111.);
    println!("{:?}", re);

    //-- some stats
    println!("Number of points in DT: {}", dt.number_of_vertices());
    println!("Number of triangles in DT: {}", dt.number_of_triangles());
    println!("bbox: {:?}", dt.get_bbox());
}
