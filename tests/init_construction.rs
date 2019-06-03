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

// #[test]
// fn duplicates() {
//     let mut dt = startin::Triangulation::new();
//     let re = dt.insert_one_pt(2.2, 2.3, 4.5);
//     re = dt.insert_one_pt(2.2, 2.3, 4.5);
//     assert!(re.is_ok());
//     assert_eq!(1, re.unwrap());
//     assert_eq!(1, dt.number_of_vertices());
//     assert_eq!(0, dt.number_of_triangles());
// }
