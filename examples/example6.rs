extern crate startin;

fn main() {
    let mut pts: Vec<Vec<f64>> = Vec::new();
    pts.push(vec![43.280539542478635, 95.35863890738987, 2.0]);
    pts.push(vec![66.14032927311288, 82.85271227835442, 2.0]);
    pts.push(vec![1.4093428832859134, 92.76988963342919, 2.0]);
    pts.push(vec![62.96333660608633, 93.33531186396561, 2.0]);
    pts.push(vec![63.71391632861801, 65.66157231281619, 2.0]);
    pts.push(vec![16.08822729811472, 47.5730496187353, 2.0]);
    pts.push(vec![5.6848363041893, 90.4851664268796, 2.0]);
    pts.push(vec![57.695476669379765, 79.64865421377347, 2.0]);
    pts.push(vec![62.39148990455298, 76.62176198691277, 2.0]);
    pts.push(vec![92.6915212773203, 57.578081306256536, 2.0]);
    pts.push(vec![25.74780275464058, 41.80687651869954, 2.0]);
    pts.push(vec![64.37821714868092, 74.50228382385613, 2.0]);
    pts.push(vec![98.29621924152339, 51.76577004224608, 2.0]);
    pts.push(vec![45.64272880746103, 74.3903035504656, 2.0]);
    pts.push(vec![69.51393660452635, 30.488118771201044, 2.0]);
    pts.push(vec![32.54039941272271, 26.85552324494419, 2.0]);
    pts.push(vec![95.18836194411536, 61.63777146538612, 2.0]);
    pts.push(vec![91.20955564995384, 16.55393588914914, 2.0]);
    pts.push(vec![78.24620342206315, 98.5235761039577, 2.0]);
    pts.push(vec![58.26477349490131, 85.8722669976183, 2.0]);

    let mut dt = startin::Triangulation::new();
    dt.set_jump_and_walk(false);
    dt.insert(&pts);
    println!("{}", dt.printme(true));
    let _re = dt.write_obj("/Users/hugo/temp/0.obj".to_string(), true);

    let _re = dt.remove(6);
    println!("{}", dt.printme(true));

    // assert!(dt.is_valid());
    let _re = dt.write_obj("/Users/hugo/temp/1.obj".to_string(), true);

    // let _re = dt.insert_one_pt(1.5, 1.5, 33.0);
    // println!("{}", dt.printme(true));
    // assert!(dt.is_valid());
}
