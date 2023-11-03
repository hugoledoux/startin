use startin;

#[test]
fn empty() {
    let dt = startin::Triangulation::<f64>::new();
    assert_eq!(0, dt.number_of_vertices());
    assert_eq!(0, dt.number_of_triangles());
}

#[test]
fn one_pt() {
    let mut dt = startin::Triangulation::new();
    let re = dt.insert_one_pt(2.2, 2.3, 4.5);
    assert!(re.is_ok());
    assert_eq!(1, re.unwrap());
    assert_eq!(1, dt.number_of_vertices());
    assert_eq!(0, dt.number_of_triangles());
    assert_eq!(
        Err(startin::StartinError::EmptyTriangulation),
        dt.locate(0.5, 0.5)
    );
}

#[test]
fn infinite() {
    let mut dt = startin::Triangulation::new();
    let _re = dt.insert_one_pt(1., 1., 1.);
    let _re = dt.insert_one_pt(2., 1., 2.);
    let _re = dt.insert_one_pt(2., 2., 3.);
    let _re = dt.insert_one_pt(1., 2., 4.);
    let _re = dt.insert_one_pt(1.5, 1.5, 5.);

    assert_eq!(5, dt.number_of_vertices());
    assert_eq!(4, dt.number_of_triangles());
    assert_eq!(8, dt.all_triangles().len());
    assert_eq!(4, dt.all_finite_triangles().len());
}

#[test]
fn duplicates() {
    let mut dt = startin::Triangulation::new();
    let _re = dt.insert_one_pt(2.5, 6.3, 4.5);
    let _re = dt.insert_one_pt(2.2, 2.3, 4.5);
    let re2 = dt.insert_one_pt(2.2, 2.3, 4.5);
    assert!(re2.is_err());
    assert_eq!(2, re2.unwrap_err());
    assert_eq!(2, dt.number_of_vertices());
    assert_eq!(0, dt.number_of_triangles());
}

#[test]
fn grid() {
    let mut dt = startin::Triangulation::new();
    for i in 0..10 {
        for j in 0..10 {
            let _re = dt.insert_one_pt(i as f64, j as f64, 1.0);
        }
    }
    assert!(dt.is_valid());
}
