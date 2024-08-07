use serde::Deserialize;
use serde::Serialize;
use serde_json::json;

extern crate startin;

#[derive(Serialize, Deserialize)]
struct MyAttribute {
    intensity: f64,
    visited: bool,
    classification: u64,
}

fn main() {
    let mut dt = startin::Triangulation::new();
    dt.set_duplicates_handling(startin::DuplicateHandling::Highest);
    let att_schema: Vec<(String, String)> = vec![
        ("intensity".to_string(), "f64".to_string()),
        ("classification".to_string(), "u64".to_string()),
        ("visited".to_string(), "bool".to_string()),
    ];
    let _ = dt.set_attributes_schema(att_schema);

    let a = MyAttribute {
        intensity: 44.0,
        visited: false,
        classification: 1,
    };

    let mut vi = dt.insert_one_pt(20 as f64, 30.0, 1.1);
    let _ = dt.add_vertex_attributes(vi.unwrap(), serde_json::to_value(a).unwrap());
    //-- no json object, nothing will be added
    _ = dt.insert_one_pt(120.0, 33.0, 12.5);
    _ = dt.insert_one_pt(124.0, 222.0, 7.65);
    vi = dt.insert_one_pt(20.0, 133.0, 21.0);
    let _ = dt.add_vertex_attributes(vi.unwrap(), json!({"classification": 33, "intensity": 133}));
    _ = dt.insert_one_pt(23.0, 13.0, 11.0);
    vi = dt.insert_one_pt(60.0, 60.0, 33.0);
    let _ = dt.add_vertex_attributes(vi.unwrap(), json!({"classification": 1, "visited": true}));

    println!("{}", dt);
    println!("{:?}\n", dt.all_attributes());
    println!("{:?}", dt.all_attributes());

    println!("{:?}", dt.get_vertex_attributes(4));
}
