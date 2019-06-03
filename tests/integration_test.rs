use startin;

#[test]
fn empty() {
    let dt = startin::Triangulation::new();
    assert_eq!(0, dt.number_of_vertices());
}
