use startin;

#[test]
fn empty() {
    let dt = startin::Triangulation::new();
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
