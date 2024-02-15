use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

extern crate startin;

#[derive(Serialize, Deserialize)]
struct MyAttribute {
    intensity: f64,
    number_returns: u16,
    classification: u16,
}

fn main() {
    let mut dt = startin::Triangulation::new();
    let _ = dt.use_extra_attributes(); //-- store extra attributes for each vertex

    let a = MyAttribute {
        intensity: 44.0,
        number_returns: 5,
        classification: 1,
    };
    let attr = serde_json::to_value(a).unwrap();

    let _ = dt.insert_one_pt_with_attribute(20 as f64, 30.0, 1.1, json!(1.1));
    let _ = dt.insert_one_pt_with_attribute(20 as f64, 30.0, 0.0, json!(1.0));
    let _ = dt.insert_one_pt_with_attribute(120.0, 33.0, 12.5, attr);
    let _ = dt.insert_one_pt_with_attribute(124.0, 222.0, 7.65, json!(1.3));
    let _ = dt.insert_one_pt_with_attribute(20.0, 133.0, 21.0, json!(1.4));
    let _ = dt.insert_one_pt(23.0, 13.0, 11.0);
    let _ = dt.insert_one_pt_with_attribute(60.0, 60.0, 33.0, json!(1.5));

    println!("{}", dt);

    println!("{:?}", dt.all_attributes());

    //     let _ = dt.remove(3);
    //     // println!("{}", dt);
    //     dt.collect_garbage();
    //     println!("{}", dt);
}
