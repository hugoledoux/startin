use serde_json::json;

extern crate startin;

fn main() {
    let mut dt = startin::Triangulation::new(true); //-- true means 'extra_attributes'
    dt.set_duplicates_handling(startin::DuplicateHandling::Lowest);
    let _ = dt.insert_one_pt_with_attribute(20 as f64, 30.0, 1.1, json!(1.1));
    let _ = dt.insert_one_pt_with_attribute(20 as f64, 30.0, 0.0, json!(1.0));
    let _ = dt.insert_one_pt_with_attribute(120.0, 33.0, 12.5, json!(1.2));
    let _ = dt.insert_one_pt_with_attribute(124.0, 222.0, 7.65, json!(1.3));
    let _ = dt.insert_one_pt_with_attribute(20.0, 133.0, 21.0, json!(1.4));
    let _ = dt.insert_one_pt(23.0, 13.0, 11.0);
    let _ = dt.insert_one_pt_with_attribute(60.0, 60.0, 33.0, json!(1.5));

    println!("{}", dt);
    // let _ = dt.remove(3);
    // println!("{}", dt);
    // dt.collect_garbage();
    // println!("{}", dt);
}
