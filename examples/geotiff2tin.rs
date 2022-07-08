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

    println!("rasterband offset: {:?}", rasterband.offset());
    println!("rasterband x-size: {:?}", rasterband.x_size());
    println!("rasterband y-size: {:?}", rasterband.y_size());
    println!("no_data: {:?}", rasterband.no_data_value());

    let mut pts: Vec<[f64; 3]> = Vec::new();

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
                    pts.push([x, y, *z]);
                }
            }
        }
    }

    let mut dt = startin::Triangulation::new();
    // dt.set_jump_and_walk(true);
    // let bbox = [
    //     crs[0],
    //     crs[3] + (ysize as f64 * crs[5]),
    //     crs[0] + (xsize as f64 * crs[1]),
    //     crs[3],
    // ];
    // dt.insert_with_bbox(&pts, &bbox);
    dt.insert(&pts, startin::InsertionStrategy::BBox);

    println!("Number of points in DT: {}", dt.number_of_vertices());
    println!("Number of triangles in DT: {}", dt.number_of_triangles());
    println!("bbox: {:?}", dt.get_bbox());

    dt.vertical_exaggeration(5.0);
    // let _re = dt.write_geojson("/Users/hugo/temp/g1.geojson".to_string());
    let pathout = "/Users/hugo/temp/out2.obj";
    println!("Writing OBJ file...");
    let re = dt.write_obj(pathout.to_string(), false);
    match re {
        Ok(_x) => println!("--> OBJ output saved to: {}", pathout),
        Err(_x) => println!("ERROR: path {} doesn't exist, abort.", pathout),
    }
}
