use crate::startin::interpolation::interpolate;
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
    let i_nn = startin::interpolation::NN {};
    let i_tin = startin::interpolation::TIN {};
    let i_lap = startin::interpolation::Laplace {};
    let i_nni = startin::interpolation::NNI { precompute: false };
    let i_idw = startin::interpolation::IDW {
        radius: 1.0,
        power: 2.0,
    };
    assert_eq!(
        Err(startin::StartinError::EmptyTriangulation),
        startin::interpolation::interpolate(&i_nn, &mut dt, &vec![[51.0, 42.0]])[0]
    );
    assert_eq!(
        Err(startin::StartinError::EmptyTriangulation),
        interpolate(&i_tin, &mut dt, &vec![[51.0, 42.0]])[0]
    );
    assert_eq!(
        Err(startin::StartinError::EmptyTriangulation),
        interpolate(&i_lap, &mut dt, &vec![[51.0, 42.0]])[0]
    );
    assert_eq!(
        Err(startin::StartinError::EmptyTriangulation),
        interpolate(&i_nni, &mut dt, &vec![[51.0, 42.0]])[0]
    );
    assert_eq!(
        Err(startin::StartinError::SearchCircleEmpty),
        interpolate(&i_idw, &mut dt, &vec![[51.0, 42.0]])[0]
    );
}

#[test]
fn idw() {
    let mut dt = four_points();
    let i_idw = startin::interpolation::IDW {
        radius: 3.0,
        power: 2.0,
    };
    assert_eq!(
        Err(startin::StartinError::SearchCircleEmpty),
        startin::interpolation::interpolate(&i_idw, &mut dt, &vec![[5.0, 5.0]])[0]
    );
    assert_eq!(
        Ok(3.0),
        startin::interpolation::interpolate(&i_idw, &mut dt, &vec![[9.0, 9.0]])[0]
    );
}

#[test]
fn outside_ch() {
    let mut dt = four_points();
    let i_nn = startin::interpolation::NN {};
    let i_tin = startin::interpolation::TIN {};
    let i_lap = startin::interpolation::Laplace {};
    let i_nni = startin::interpolation::NNI { precompute: false };
    assert_eq!(
        Err(startin::StartinError::OutsideConvexHull),
        startin::interpolation::interpolate(&i_nn, &mut dt, &vec![[5.0, -0.1]])[0]
    );
    assert_eq!(
        Err(startin::StartinError::OutsideConvexHull),
        interpolate(&i_tin, &mut dt, &vec![[5.0, -0.1]])[0]
    );
    assert_eq!(
        Err(startin::StartinError::OutsideConvexHull),
        interpolate(&i_lap, &mut dt, &vec![[5.0, -0.1]])[0]
    );
    assert_eq!(
        Err(startin::StartinError::OutsideConvexHull),
        interpolate(&i_nni, &mut dt, &vec![[5.0, -0.1]])[0]
    );
    assert_eq!(
        Err(startin::StartinError::OutsideConvexHull),
        interpolate(&i_nni, &mut dt, &vec![[5.0, 0.0]])[0]
    );
}

#[test]
fn existing_point() {
    let mut dt = four_points();
    let _re = dt.insert_one_pt(5.0, 5.0, 11.1);

    let i_nn = startin::interpolation::NN {};
    let i_tin = startin::interpolation::TIN {};
    let i_lap = startin::interpolation::Laplace {};
    let i_nni = startin::interpolation::NNI { precompute: false };
    let i_idw = startin::interpolation::IDW {
        radius: 1.0,
        power: 2.0,
    };
    assert_eq!(
        Ok(11.1),
        startin::interpolation::interpolate(&i_nn, &mut dt, &vec![[5.0, 5.0]])[0]
    );
    assert_eq!(Ok(11.1), interpolate(&i_tin, &mut dt, &vec![[5.0, 5.0]])[0]);
    assert_eq!(Ok(11.1), interpolate(&i_lap, &mut dt, &vec![[5.0, 5.0]])[0]);
    assert_eq!(Ok(11.1), interpolate(&i_nni, &mut dt, &vec![[5.0, 5.0]])[0]);
    assert_eq!(Ok(11.1), interpolate(&i_idw, &mut dt, &vec![[5.0, 5.0]])[0]);
}

#[test]
fn existing_point_lowest() {
    let mut pts: Vec<[f64; 3]> = Vec::new();
    pts.push([0.0, 0.0, 1.0]);
    pts.push([10.0, 0.0, 2.0]);
    pts.push([10.0, 10.0, 3.0]);
    pts.push([0.0, 10.0, 4.0]);
    let mut dt = startin::Triangulation::new();
    dt.set_duplicates_handling(startin::DuplicateHandling::Lowest);
    dt.insert(&pts, startin::InsertionStrategy::AsIs);
    let _re = dt.insert_one_pt(5.0, 5.0, 11.1);

    let i_lap = startin::interpolation::Laplace {};
    assert_eq!(Ok(11.1), interpolate(&i_lap, &mut dt, &vec![[5.0, 5.0]])[0]);

    let i_nni = startin::interpolation::NNI { precompute: false };
    assert_eq!(Ok(11.1), interpolate(&i_nni, &mut dt, &vec![[5.0, 5.0]])[0]);
}

#[test]
fn middle() {
    let mut dt = four_points();
    let i_lap = startin::interpolation::Laplace {};
    let i_nni = startin::interpolation::NNI { precompute: false };
    let i_tin = startin::interpolation::TIN {};
    assert_eq!(Ok(2.5), interpolate(&i_lap, &mut dt, &vec![[5.0, 5.0]])[0]);
    assert_eq!(Ok(2.5), interpolate(&i_nni, &mut dt, &vec![[5.0, 5.0]])[0]);
    assert_eq!(Ok(3.0), interpolate(&i_tin, &mut dt, &vec![[5.0, 5.0]])[0]);
}

#[test]
fn nn() {
    let mut dt = four_points();
    let _re = dt.insert_one_pt(5.0, 5.0, 11.1);
    let i_nn = startin::interpolation::NN {};
    assert_eq!(
        Ok(11.1),
        startin::interpolation::interpolate(&i_nn, &mut dt, &vec![[5.1, 5.1]])[0]
    );
}

#[test]
fn tin_linear_random() {
    let mut dt = random_points_500();
    let i_tin = startin::interpolation::TIN {};
    assert_eq!(
        Err(startin::StartinError::OutsideConvexHull),
        startin::interpolation::interpolate(&i_tin, &mut dt, &vec![[144.0, 48.0]])[0]
    );
    assert_eq!(
        true,
        startin::interpolation::interpolate(&i_tin, &mut dt, &vec![[44.0, 48.0]])[0].is_ok()
    );
}
