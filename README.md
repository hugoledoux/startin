# startin

[![crates.io](https://img.shields.io/crates/v/startin.svg)](https://crates.io/crates/startin)

A Delaunay triangulator where the input are 2.5D points, the DT is computed in 2D but the elevation of the vertices are kept.
This is used mostly for the modelling of terrains.

The construction algorithm used is an incremental insertion based on flips, and the data structure is a cheap implementation of the star-based structure defined in [Blandford et al. (2003)](https://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.9.6823), cheap because the link of each vertex is stored a simple array (`Vec`) and not in an optimised blob like they did.
It results in a pretty fast library (comparison will come at some point), but it uses more space than the optimised one.

The deletion of a vertex is also possible. The algorithm implemented is a modification of the one of [Mostafavi, Gold, and Dakowicz (2003)](https://doi.org/10.1016/S0098-3004(03)00017-7). The ears are filled by flipping, so it's in theory more robust. I have also extended the algorithm to allow the deletion of vertices on the boundary of the convex hull. The algorithm is sub-optimal, but in practice the number of neighbours of a given vertex in a DT is only 6, so it doesn't really matter.

Robust arithmetic for the geometric predicates are used ([Shewchuk's predicates](https://www.cs.cmu.edu/~quake/robust.html)), so the library is robust and shouldn't crash (touch wood). 

I made this in Rust because I wanted to learn Rust.

But if you prefer Python, I made bindings (very beta): [https://github.com/hugoledoux/startin_python/](https://github.com/hugoledoux/startin_python/)


# Usage

```rust
extern crate startin;

fn main() {
    let mut pts: Vec<Vec<f64>> = Vec::new();
    pts.push(vec![20.0, 30.0, 2.0]);
    pts.push(vec![120.0, 33.0, 12.5]);
    pts.push(vec![124.0, 222.0, 7.65]);
    pts.push(vec![20.0, 133.0, 21.0]);
    pts.push(vec![60.0, 60.0, 33.0]);

    let mut dt = startin::Triangulation::new();
    dt.insert(&pts);

    println!("*****");
    println!("Number of points in DT: {}", dt.number_of_vertices());
    println!("Number of triangles in DT: {}", dt.number_of_triangles());

    //-- print all the vertices
    for (i, each) in dt.all_vertices().iter().enumerate() {
        // skip the first one, the infinite vertex
        if i > 0 {
            println!("#{}: ({:.3}, {:.3}, {:.3})", i, each[0], each[1], each[2]);
        }
    }

    //-- insert a new vertex
    let re = dt.insert_one_pt(22.2, 33.3, 4.4);
    match re {
        Ok(_v) => println!("Inserted new point"),
        Err(v) => println!("Duplicate of vertex #{}, not inserted", v),
    }
    //-- remove it
    let re = dt.remove(6);
    if re.is_err() == true {
        println!("!!! Deletion error: {:?}", re.unwrap_err());
    } else {
        println!("Deleted vertex");
    }
    //-- insert it again
    let re = dt.insert_one_pt(22.2, 33.3, 4.4);
    match re {
        Ok(_v) => println!("Inserted new point"),
        Err(v) => println!("Duplicate of vertex #{}, not inserted", v),
    }

    //-- get the convex hull
    let ch = dt.convex_hull();
    println!("Convex hull: {:?}", ch);

    //-- fetch triangle containing (x, y)
    let re = dt.locate(50.0, 50.0);
    if re.is_some() {
        let t = re.unwrap();
        println!("The triangle is {}", t);
        assert!(dt.is_triangle(&t));
    } else {
        println!("Outside convex hull");
    }

    //-- some stats
    println!("Number of points in DT: {}", dt.number_of_vertices());
    println!("Number of triangles in DT: {}", dt.number_of_triangles());
}
```
