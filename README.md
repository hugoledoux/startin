# startin

A Delaunay triangulator where the input are 2.5D points, the DT is computed in 2D but the elevation of the vertices are kept.
This is used mostly for the modelling of terrains.

The algorithm used is an incremental insertion based on flips, and the data structure is a cheap implementation of the star-based structure defined in [Blandford et al. (2003)](https://citeseerx.ist.psu.edu/viewdoc/summary?doi=10.1.1.9.6823), cheap because the link of each vertex is stored a simple array (`Vec`) and not in an optimised blob like they did.
Still, it results is a pretty fast library it seems.

Robust arithmetic for the geometric predicates are used ([Shewchuk's predicates](https://www.cs.cmu.edu/~quake/robust.html)), so the library is robust and shouldn't crash (touch wood). 

I made this in Rust because I wanted to learn Rust.

# Usage

```rust
extern crate startin;

fn main() {
    let mut dt = startin::Triangulation::new();

    //-- insert 5 points
    dt.insert_one_pt(20.0,  30.0,  2.0).unwrap();
    dt.insert_one_pt(120.0, 33.0,  12.5).unwrap();
    dt.insert_one_pt(124.0, 222.0, 7.65).unwrap();
    dt.insert_one_pt(20.0,  133.0, 21.0).unwrap();
    dt.insert_one_pt(60.0,  60.0,  33.0).unwrap();

    println!("Number of points in DT: {}", dt.number_of_vertices());
    println!("Number of triangles in DT: {}", dt.number_of_triangles());

    //-- print all the vertices
    for (i, each) in dt.all_vertices().iter().enumerate() {
        println!(
            "#{}: ({:.3}, {:.3}, {:.3})",
            (i + 1),
            each[0],
            each[1],
            each[2]
        );
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
}
```

