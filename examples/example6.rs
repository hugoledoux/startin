extern crate startin;

fn main() {
    let mut pts: Vec<Vec<f64>> = Vec::new();

    pts.push(vec![57.42265069953759, 11.302054173605036, 2.0]);
    pts.push(vec![92.84366030693992, 43.6916136057666, 2.0]);
    pts.push(vec![1.4236330964329302, 64.90700146602387, 2.0]);
    pts.push(vec![13.285710267579498, 50.189815660581175, 2.0]);
    pts.push(vec![15.111123388743696, 78.78533829573448, 2.0]);
    pts.push(vec![36.63646690982746, 6.91987548989339, 2.0]);
    pts.push(vec![29.637182771593974, 68.65833069674996, 2.0]);
    pts.push(vec![11.833562381185224, 17.179817935796258, 2.0]);
    pts.push(vec![76.1403374462718, 85.06661188634638, 2.0]);
    pts.push(vec![97.68197803600579, 86.72138786481214, 2.0]);
    pts.push(vec![57.14429336094553, 85.64704227817887, 2.0]);
    pts.push(vec![39.00833936370296, 38.120561196977896, 2.0]);
    pts.push(vec![72.92102098683782, 29.345682339583913, 2.0]);
    pts.push(vec![11.294274255141302, 3.9130782784767892, 2.0]);
    pts.push(vec![0.22779172434063133, 28.09424506294478, 2.0]);
    pts.push(vec![90.87939595311902, 69.60923683981981, 2.0]);
    pts.push(vec![13.196743373706909, 37.43824255825546, 2.0]);
    pts.push(vec![31.851227544325145, 51.80273049754918, 2.0]);
    pts.push(vec![0.6726961194112979, 43.93675937945696, 2.0]);
    pts.push(vec![30.932824361776735, 66.6221847617953, 2.0]);

    let mut dt = startin::Triangulation::new();
    dt.set_jump_and_walk(false);
    dt.insert(&pts, None);
    println!("{}", dt.printme(true));
    // let _re = dt.write_obj("/Users/hugo/temp/0.obj".to_string(), true);

    let _re = dt.remove(11);
    // println!("{}", dt.printme(true));

    assert!(dt.is_valid());
    // let _re = dt.write_obj("/Users/hugo/temp/1.obj".to_string(), true);

    // let _re = dt.insert_one_pt(1.5, 1.5, 33.0);
    // println!("{}", dt.printme(true));
    // assert!(dt.is_valid());
}
