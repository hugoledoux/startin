extern crate gdal;
extern crate startin;

use gdal::raster::RasterBand;
use gdal::{Dataset, Metadata};

fn main() {
    let path = std::env::args()
        .skip(1)
        .next()
        .expect("Must provide a path to a GeoTIFF file");
    let dataset = Dataset::open(path).unwrap();

    println!("Reading GeoTIFF file: {:?}", dataset.description());
    println!("CRS: {:?}", dataset.geo_transform());
    let crs = dataset.geo_transform().unwrap();

    let rasterband: RasterBand = dataset.rasterband(1).unwrap();

    // println!("rasterband description: {:?}", rasterband.description());
    println!("rasterband no_data_value: {:?}", rasterband.no_data_value());
    // println!("rasterband type: {:?}", rasterband.band_type());
    // println!("rasterband scale: {:?}", rasterband.scale());
    println!("rasterband offset: {:?}", rasterband.offset());
    println!("rasterband x-size: {:?}", rasterband.x_size());
    println!("rasterband y-size: {:?}", rasterband.y_size());
    println!("no_data: {:?}", rasterband.no_data_value());

    let mut pts: Vec<Vec<f64>> = Vec::new();

    let nodatavalue = rasterband.no_data_value().unwrap();
    let xsize = rasterband.x_size();
    let ysize = rasterband.y_size();
    //-- for each line, starting from the top-left
    for j in 0..ysize {
        if let Ok(rv) =
            rasterband.read_as::<f64>((0, j.try_into().unwrap()), (xsize, 1), (xsize, 1), None)
        {
            for (i, each) in rv.data.iter().enumerate() {
                let x = crs[0] + (i as f64 * crs[1]) + crs[1];
                let y = crs[3] + (j as f64 * crs[5]) + crs[5];
                let z = each;
                if *z != nodatavalue {
                    pts.push(vec![x, y, *z]);
                }
            }
            // println!("{:?}", rv.data);
            // println!("{:?}", rv.data.len());
        }
    }
    // println!("size: {:?}", pts.len());

    let mut dt = startin::Triangulation::new();
    dt.set_jump_and_walk(true);
    let bbox = vec![
        crs[0],
        crs[3] + (ysize as f64 * crs[5]),
        crs[0] + (xsize as f64 * crs[1]),
        crs[3],
    ];
    dt.insert(&pts, Some(bbox));

    println!("Number of points in DT: {}", dt.number_of_vertices());
    println!("Number of triangles in DT: {}", dt.number_of_triangles());
    println!("bbox: {:?}", dt.get_bbox());

    let _re = dt.write_geojson("/Users/hugo/temp/g1.geojson".to_string());
    // let pathout = "/Users/hugo/temp/out.obj";
    // println!("Writing OBJ file...");
    // let re = dt.write_obj(pathout.to_string(), false);
    // match re {
    //     Ok(_x) => println!("--> OBJ output saved to: {}", pathout),
    //     Err(_x) => println!("ERROR: path {} doesn't exist, abort.", pathout),
    // }
}

// use las::{Read, Reader};

// fn main() {
//     let path = std::env::args()
//         .skip(1)
//         .next()
//         .expect("Must provide a path to a LAS/LAZ file");
//     let mut reader = Reader::from_path(path).expect("Wrong file name");

//     let header = reader.header();
//     println!("Reading LAS file version: {}", header.version());
//     println!("{} points.", header.number_of_points());

//     // let b = header.bounds();
//     // println!(
//     // "({}, {}, {}) --> ({}, {}, {})",
//     // b.min.x, b.min.y, b.min.z, b.max.x, b.max.y, b.max.z
//     // );

//     let mut dt = startin::Triangulation::new();
//     dt.set_jump_and_walk(true);

//     for laspt in reader.points() {
//         let p = laspt.unwrap();
//         let _re = dt.insert_one_pt(p.x, p.y, p.z);
//     }

//     println!("Number of points in DT: {}", dt.number_of_vertices());
//     println!("Number of triangles in DT: {}", dt.number_of_triangles());
//     println!("bbox: {:?}", dt.get_bbox());

//     // let pathout = "/Users/hugo/temp/out.obj";
//     // println!("Writing OBJ file...");
//     // let re = dt.write_obj(pathout.to_string(), false);
//     // match re {
//     //     Ok(_x) => println!("--> OBJ output saved to: {}", pathout),
//     //     Err(_x) => println!("ERROR: path {} doesn't exist, abort.", pathout),
//     // }
// }
