extern crate startin;

// fn main() {
//     let mut pts: Vec<[f64; 3]> = Vec::new();
//
//     pts.push([0.0, 0.0, 1.0]);
//     pts.push([10.0, 0.0, 1.0]);
//     pts.push([10.0, 10.0, 1.0]);
//     pts.push([0.0, 10.0, 1.0]);
//     pts.push([10.0, 5.0, 1.0]);
//     pts.push([5.0, 5.0, 1.0]);
//
//     let mut dt = startin::Triangulation::new();
//     dt.insert(&pts, startin::InsertionStrategy::AsIs);
//     print!("0{}", dt);
//
//     // let new_pt = dt.insert_one_pt(1.0, 1.0, 1.0);
//     // print!("1{}", dt);
//
//     let _re = dt.remove(5);
//     print!("2{}", dt);
//
//     let _ = dt.insert_one_pt(9.0, 9.0, 0.0);
//     print!("3{}", dt);
// }

//-- case of the student rust panic in python
fn main() {
    let mut pts: Vec<[f64; 3]> = Vec::new();

    pts.push([0.0, 0.0, 1.0]);
    pts.push([0.0, 1.0, 1.0]);
    pts.push([0.0, 2.0, 1.0]);
    // pts.push([0.0, 3.0, 1.0]);

    let mut dt = startin::Triangulation::new();
    dt.insert(&pts, startin::InsertionStrategy::AsIs);
    // println!("{}", dt.printme(true));
    // print!("0{}", dt);

    let new_pt = dt.insert_one_pt(1.0, 1.0, 1.0);
    // print!("1{}", dt);

    let _re = dt.remove(new_pt.unwrap());
    assert!(dt.is_valid());
    // print!("2{}", dt);

    let _ = dt.insert_one_pt(1.0, 1.0, 0.0);
    assert!(dt.is_valid());
    // print!("3{}", dt);


    let _ = dt.insert_one_pt(11.0, 11.0, 11.0);
    // print!("3{}", dt);
    let _ = dt.insert_one_pt(1.0, 2.0, 11.0);
    assert!(dt.is_valid());
    // print!("3{}", dt);

    assert!(dt.is_valid());
}
