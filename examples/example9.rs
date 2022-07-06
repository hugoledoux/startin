extern crate startin;

fn main() {
    let mut pts: Vec<Vec<f64>> = Vec::new();

    pts.push(vec![0.0, 0.0, 0.125]);
    pts.push(vec![0.0, 0.0, 0.1111]);
    pts.push(vec![1.0, 0.0, 0.2222]);
    pts.push(vec![1.0, 1.0, 0.3333]);
    pts.push(vec![0.0, 1.0, 0.440]);
    pts.push(vec![0.5, 0.49, 0.440]);
    pts.push(vec![0.45, 0.69, 0.440]);
    pts.push(vec![0.65, 0.49, 0.440]);
    pts.push(vec![0.75, 0.29, 0.440]);
    pts.push(vec![1.5, 1.49, 0.440]);
    pts.push(vec![0.6, 0.2, 0.440]);
    pts.push(vec![0.45, 0.4, 0.440]);
    pts.push(vec![0.1, 0.8, 0.440]);

    let mut dt = startin::Triangulation::new();
    // dt.insert(&pts, None);
    dt.insert(&pts, Some(vec![-10.0, -10.0, 10.0, 10.0]));

    // let re = dt.interpolate_nn(2., 1.);
    // let re = dt.interpolate_nn(11., 11.);
    // let re = dt.interpolate_tin_linear(2., 1.);
    // let re = dt.interpolate_laplace(1., 1.);
    // println!("{:?}", re);

    // let re = dt.interpolate_laplace(111., 111.);
    // println!("{:?}", re);

    //-- some stats
    println!("Number of points in DT: {}", dt.number_of_vertices());
    println!("Number of triangles in DT: {}", dt.number_of_triangles());
    println!("bbox: {:?}", dt.get_bbox());

    let _re = dt.write_geojson("/Users/hugo/temp/g1.geojson".to_string());

    let pathout = "/Users/hugo/temp/out.obj";
    println!("Writing OBJ file...");
    let re = dt.write_obj(pathout.to_string(), false);
    match re {
        Ok(_x) => println!("--> OBJ output saved to: {}", pathout),
        Err(_x) => println!("ERROR: path {} doesn't exist, abort.", pathout),
    }
}
