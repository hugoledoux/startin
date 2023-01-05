use crate::startin::Triangulation;
use rand::prelude::*;
use startin;

fn four_points() -> Triangulation {
    let mut pts: Vec<[f64; 3]> = Vec::new();
    pts.push([0.0, 0.0, 1.0]);
    pts.push([10.0, 0.0, 2.0]);
    pts.push([10.0, 10.0, 3.0]);
    pts.push([0.0, 10.0, 4.0]);
    let mut dt = startin::Triangulation::new();
    dt.insert(&pts, startin::InsertionStrategy::AsIs);
    dt
}

fn random_points_500() -> Triangulation {
    let mut dt = startin::Triangulation::new();
    let mut rng = rand::thread_rng();
    for _i in 0..500 {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        let z: f64 = rng.gen();
        let _re = dt.insert_one_pt(x * 100.0, y * 100.0, z * 10.0);
    }
    dt
}

#[test]
fn empty() {
    let mut dt = startin::Triangulation::new();
    assert_eq!(
        Err(startin::StartinError::NoTriangleinTIN),
        dt.interpolate_nn(&vec![[51.0, 42.0]])[0]
    );
    assert_eq!(
        Err(startin::StartinError::NoTriangleinTIN),
        dt.interpolate_nni(&vec![[51.0, 42.0]], true)[0]
    );
    assert_eq!(
        Err(startin::StartinError::NoTriangleinTIN),
        dt.interpolate_laplace(&vec![[51.0, 42.0]])[0]
    );
    assert_eq!(
        Err(startin::StartinError::NoTriangleinTIN),
        dt.interpolate_tin_linear(&vec![[51.0, 42.0]])[0]
    );
}

#[test]
fn outside_ch() {
    let mut dt = four_points();
    assert_eq!(
        Err(startin::StartinError::OutsideConvexHull),
        dt.interpolate_nn(&vec![[5.0, -0.1]])[0]
    );
    assert_eq!(
        Err(startin::StartinError::OutsideConvexHull),
        dt.interpolate_nni(&vec![[5.0, -0.1]], false)[0]
    );
    assert_eq!(
        Err(startin::StartinError::OutsideConvexHull),
        dt.interpolate_laplace(&vec![[5.0, -0.1]])[0]
    );
    assert_eq!(
        Err(startin::StartinError::OutsideConvexHull),
        dt.interpolate_tin_linear(&vec![[5.0, -0.1]])[0]
    );
    assert_eq!(
        Err(startin::StartinError::OutsideConvexHull),
        dt.interpolate_nni(&vec![[5.0, 0.0]], true)[0]
    );
}

#[test]
fn existing_point() {
    let mut dt = four_points();
    let _re = dt.insert_one_pt(5.0, 5.0, 11.1);
    assert_eq!(Ok(11.1), dt.interpolate_nn(&vec![[5.0, 5.0]])[0]);
    assert_eq!(Ok(11.1), dt.interpolate_nni(&vec![[5.0, 5.0]], false)[0]);
    assert_eq!(Ok(11.1), dt.interpolate_laplace(&vec![[5.0, 5.0]])[0]);
    assert_eq!(Ok(11.1), dt.interpolate_tin_linear(&vec![[5.0, 5.0]])[0]);
    assert_eq!(Ok(1.0), dt.interpolate_nn(&vec![[0.0, 0.0]])[0]);
    assert_eq!(Ok(1.0), dt.interpolate_nni(&vec![[0.0, 0.0]], false)[0]);
    assert_eq!(Ok(1.0), dt.interpolate_laplace(&vec![[0.0, 0.0]])[0]);
    assert_eq!(Ok(1.0), dt.interpolate_tin_linear(&vec![[0.0, 0.0]])[0]);
}

#[test]
fn middle() {
    let mut dt = four_points();
    assert_eq!(Ok(2.5), dt.interpolate_nni(&vec![[5.0, 5.0]], true)[0]);
    assert_eq!(Ok(2.5), dt.interpolate_laplace(&vec![[5.0, 5.0]])[0]);
}

#[test]
fn nn() {
    let mut dt = four_points();
    let _re = dt.insert_one_pt(5.0, 5.0, 11.1);
    assert_eq!(Ok(11.1), dt.interpolate_nn(&vec![[5.1, 5.1]])[0]);
}

#[test]
fn tin_linear() {
    let dt = four_points();
    assert_eq!(Ok(1.5), dt.interpolate_tin_linear(&vec![[5.0, 0.0]])[0]);
}

#[test]
fn tin_linear_random() {
    let dt = random_points_500();
    assert_eq!(
        Err(startin::StartinError::OutsideConvexHull),
        dt.interpolate_tin_linear(&vec![[144.0, 48.0]])[0]
    );
    assert_eq!(
        true,
        dt.interpolate_tin_linear(&vec![[44.0, 48.0]])[0].is_ok()
    );
}

#[test]
fn nni_boundary_ch() {
    let mut dt = four_points();
    let re = dt.interpolate_nni(&vec![[10.0, 5.0]], false);
    assert_eq!(false, re[0].is_ok());
}
