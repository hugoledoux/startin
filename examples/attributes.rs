use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

extern crate startin;

#[derive(Serialize, Deserialize)]
struct MyAttribute {
    intensity: f64,
    number_returns: u64,
    classification: u64,
}

fn main() {
    let mut dt = startin::Triangulation::new();
    dt.set_duplicates_handling(startin::DuplicateHandling::Highest);
    dt.add_attribute_map("intensity".to_string(), "f64".to_string());
    dt.add_attribute_map("classification".to_string(), "u64".to_string());

    let a = MyAttribute {
        intensity: 44.0,
        number_returns: 5,
        classification: 1,
    };
    let attr = serde_json::to_value(a).unwrap();

    let mut vi = dt.insert_one_pt(20 as f64, 30.0, 1.1);
    let _ = dt.add_vertex_attributes(vi.unwrap(), attr);
    vi = dt.insert_one_pt(120.0, 33.0, 12.5);
    //-- no json object, nothing will be added
    let _ = dt.add_vertex_attributes(vi.unwrap(), json!(45));
    _ = dt.insert_one_pt(124.0, 222.0, 7.65);
    vi = dt.insert_one_pt(20.0, 133.0, 21.0);
    let _ = dt.add_vertex_attributes(vi.unwrap(), json!({"classification": 33.3}));
    _ = dt.insert_one_pt(23.0, 13.0, 11.0);
    _ = dt.insert_one_pt(60.0, 60.0, 33.0);

    println!("{}", dt);
    println!("{:?}", dt.list_all_attributes());
    println!("{:?}", dt.all_attributes());
}
