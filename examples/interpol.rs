extern crate startin;

fn main() {
    let mut pts: Vec<[f64; 3]> = Vec::new();
    pts.push([20 as f64, 30.0, 0.0]);
    pts.push([20 as f64, 30.0, 1.1]);
    pts.push([120.0, 33.0, 12.5]);
    pts.push([124.0, 222.0, 7.65]);
    pts.push([20.0, 133.0, 21.0]);
    pts.push([60.0, 60.0, 33.0]);

    let mut dt = startin::Triangulation::new();
    dt.insert(&pts, startin::InsertionStrategy::AsIs);

    println!("{}", dt);

    let re = startin::interpolation::nni(&mut dt, &vec![[54., 54.]], true);
    println!("{:?}", re);
    let re = startin::interpolation::laplace(&mut dt, &vec![[54., 54.]]);
    println!("{:?}", re);

    // println!("{:?}", dt.stars.len());
}
