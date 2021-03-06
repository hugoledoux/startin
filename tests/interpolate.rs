use crate::startin::Triangulation;
use startin;

fn four_points() -> Triangulation {
    let mut pts: Vec<Vec<f64>> = Vec::new();
    pts.push(vec![0.0, 0.0, 1.0]);
    pts.push(vec![10.0, 0.0, 2.0]);
    pts.push(vec![10.0, 10.0, 3.0]);
    pts.push(vec![0.0, 10.0, 4.0]);
    let mut dt = startin::Triangulation::new();
    dt.insert(&pts);
    dt
}

#[test]
fn empty() {
    let mut dt = startin::Triangulation::new();
    assert_eq!(None, dt.interpolate_nn(1.1, 1.1));
    assert_eq!(None, dt.interpolate_nni(1.1, 1.1));
    assert_eq!(None, dt.interpolate_laplace(1.1, 1.1));
    assert_eq!(None, dt.interpolate_tin_linear(1.1, 1.1));
}

#[test]
fn outside_ch() {
    let mut dt = four_points();
    assert_eq!(None, dt.interpolate_nn(5.0, -0.1));
    assert_eq!(None, dt.interpolate_nni(5.0, -0.1));
    assert_eq!(None, dt.interpolate_laplace(5.0, -0.1));
    assert_eq!(None, dt.interpolate_tin_linear(5.0, -0.1));
    assert_eq!(None, dt.interpolate_nni(5.0, 0.0));
}

#[test]
fn existing_point() {
    let mut dt = four_points();
    let _re = dt.insert_one_pt(5.0, 5.0, 11.1);
    assert_eq!(Some(11.1), dt.interpolate_nn(5.0, 5.0));
    assert_eq!(Some(11.1), dt.interpolate_nni(5.0, 5.0));
    assert_eq!(Some(11.1), dt.interpolate_laplace(5.0, 5.0));
    assert_eq!(Some(11.1), dt.interpolate_tin_linear(5.0, 5.0));
    assert_eq!(Some(1.0), dt.interpolate_nn(0.0, 0.0));
    assert_eq!(Some(1.0), dt.interpolate_nni(0.0, 0.0));
    assert_eq!(Some(1.0), dt.interpolate_laplace(0.0, 0.0));
    assert_eq!(Some(1.0), dt.interpolate_tin_linear(0.0, 0.0));
}

#[test]
fn middle() {
    let mut dt = four_points();
    assert_eq!(Some(2.5), dt.interpolate_nni(5.0, 5.0));
    assert_eq!(Some(2.5), dt.interpolate_laplace(5.0, 5.0));
}

#[test]
fn nn() {
    let mut dt = four_points();
    let _re = dt.insert_one_pt(5.0, 5.0, 11.1);
    assert_eq!(Some(11.1), dt.interpolate_nn(5.1, 5.1));
}

#[test]
fn tin_linear() {
    let dt = four_points();
    assert_eq!(Some(1.5), dt.interpolate_tin_linear(5.0, 0.0));
}
