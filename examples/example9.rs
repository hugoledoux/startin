extern crate startin;

fn main() {
    let mut pts: Vec<Vec<f64>> = Vec::new();
    pts.push(vec![0.0, 0.0, 12.5]);
    pts.push(vec![10.0, 0.0, 7.0]);
    pts.push(vec![12.0, 12.0, 21.0]);
    pts.push(vec![0.0, 10.0, 30.0]);

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
}
