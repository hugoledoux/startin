//! # geom
//!
//! Geometric predicates functions are declared here.
//! Notice that [Shewchuk's predicates](https://www.cs.cmu.edu/~quake/robust.html)
//! are used (activated by default by startin), but also possible to rely on floating-point
//! arithmetic (not recommended).

extern crate robust;

pub fn det3x3t(a: &[f64], b: &[f64], c: &[f64]) -> f64 {
    ((a[0] - c[0]) * (b[1] - c[1])) - ((a[1] - c[1]) * (b[0] - c[0]))
}

pub fn crossproduct(a: &[f64], b: &[f64]) -> Vec<f64> {
    let i = (a[1] * b[2]) - (a[2] * b[1]);
    let j = (a[0] * b[2]) - (a[2] * b[0]);
    let k = (a[0] * b[1]) - (a[1] * b[0]);
    return vec![i, -j, k];
}

pub fn area2d_triangle(a: &[f64], b: &[f64], c: &[f64]) -> f64 {
    det3x3t(a, b, c) / 2.0
}

pub fn area3d_triangle(a: &[f64], b: &[f64], c: &[f64]) -> f64 {
    let v1 = vec![b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let v2 = vec![c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let i = (v1[1] * v2[2]) - (v1[2] * v2[1]);
    let j = (v1[0] * v2[2]) - (v1[2] * v2[0]);
    let k = (v1[0] * v2[1]) - (v1[1] * v2[0]);
    return 0.5 * (i * i + j * j + k * k).sqrt();
}

pub fn volume_triangle_from_base(a: &[f64], b: &[f64], c: &[f64], basez: f64) -> f64 {
    //-- volume of the prism from the basez to minz
    let minz = a[2].min(b[2]).min(c[2]);
    let vol0 = area2d_triangle(&a, &b, &c) * (minz - basez);
    //-- construct a polyhedron, 1+ of the vertices will be at minz
    let a_ = vec![a[0], a[1], minz];
    let b_ = vec![b[0], b[1], minz];
    let c_ = vec![c[0], c[1], minz];
    //-- split this polyhedron into 8 faces and sum their signed volume
    let mut vol = 0.0;
    vol += signed_volume(&a, &b, &c);
    vol += signed_volume(&a_, &c_, &b_);
    vol += signed_volume(&a, &a_, &b);
    vol += signed_volume(&a_, &b_, &b);
    vol += signed_volume(&b, &b_, &c);
    vol += signed_volume(&b_, &c_, &c);
    vol += signed_volume(&c, &c_, &a);
    vol += signed_volume(&c_, &a_, &a);
    return vol0 + vol.abs();
}

pub fn signed_volume(a: &[f64], b: &[f64], c: &[f64]) -> f64 {
    let xp = crossproduct(&b, &c);
    let dp = (a[0] * xp[0]) + (a[1] * xp[1]) + (a[2] * xp[2]);
    return dp / 6.0;
}

pub fn normal_triangle(a: &[f64], b: &[f64], c: &[f64], normalise: bool) -> Vec<f64> {
    let v0: Vec<f64> = vec![a[0] - b[0], a[1] - b[1], a[2] - b[2]];
    let v1: Vec<f64> = vec![a[0] - c[0], a[1] - c[1], a[2] - c[2]];
    let nx = (v0[1] * v1[2]) - (v0[2] * v1[1]);
    let ny = (v0[2] * v1[0]) - (v0[0] * v1[2]);
    let nz = (v0[0] * v1[1]) - (v0[1] * v1[0]);
    if normalise {
        let norm = ((nx * nx) + (ny * ny) + (nz * nz)).sqrt();
        vec![nx / norm, ny / norm, nz / norm]
    } else {
        vec![nx, ny, nz]
    }
}

pub fn circle_centre(a: &[f64], b: &[f64], c: &[f64]) -> Vec<f64> {
    //-- nicked from http://www.ambrsoft.com/trigocalc/circle3d.htm
    let val_a = det3x3t(&[a[0], a[1], 1.0], &[b[0], b[1], 1.0], &[c[0], c[1], 1.0]);
    let val_b = det3x3t(
        &[a[0] * a[0] + a[1] * a[1], a[1], 1.0],
        &[b[0] * b[0] + b[1] * b[1], b[1], 1.0],
        &[c[0] * c[0] + c[1] * c[1], c[1], 1.0],
    );
    let val_c = det3x3t(
        &[a[0] * a[0] + a[1] * a[1], a[0], 1.0],
        &[b[0] * b[0] + b[1] * b[1], b[0], 1.0],
        &[c[0] * c[0] + c[1] * c[1], c[0], 1.0],
    );
    let x = val_b / (2.0 * val_a);
    let y = -val_c / (2.0 * val_a);
    vec![x, y, 0.0]
}

pub fn distance2d_squared(a: &[f64], b: &[f64]) -> f64 {
    (b[0] - a[0]) * (b[0] - a[0]) + (b[1] - a[1]) * (b[1] - a[1])
}

pub fn distance2d(a: &[f64], b: &[f64]) -> f64 {
    let d2 = (b[0] - a[0]) * (b[0] - a[0]) + (b[1] - a[1]) * (b[1] - a[1]);
    d2.sqrt()
}

pub fn orient2d(a: &[f64], b: &[f64], c: &[f64], robust_predicates: bool) -> i8 {
    //-- CCW    = +1
    //-- CW     = -1
    //-- colinear = 0
    if robust_predicates {
        orient2d_robust(&a, &b, &c)
    } else {
        orient2d_fast(&a, &b, &c)
    }
}

pub fn orient2d_robust(a: &[f64], b: &[f64], c: &[f64]) -> i8 {
    //-- CCW    = +1
    //-- CW     = -1
    //-- colinear = 0
    let re = robust::orient2d(
        robust::Coord { x: a[0], y: a[1] },
        robust::Coord { x: b[0], y: b[1] },
        robust::Coord { x: c[0], y: c[1] },
    );
    if re == 0.0_f64 {
        0
    } else if re.is_sign_positive() {
        return 1;
    } else {
        return -1;
    }
}

pub fn orient2d_fast(a: &[f64], b: &[f64], c: &[f64]) -> i8 {
    //-- CCW    = +1
    //-- CW     = -1
    //-- colinear = 0
    let re: f64 = ((a[0] - c[0]) * (b[1] - c[1])) - ((a[1] - c[1]) * (b[0] - c[0]));
    if re.abs() < 1e-12 {
        0
    } else if re > 0.0 {
        return 1;
    } else {
        return -1;
    }
}

pub fn incircle(a: &[f64], b: &[f64], c: &[f64], p: &[f64], robust_predicates: bool) -> i8 {
    //-- p is INSIDE   == +1
    //-- p is OUTSIDE  == -1
    //-- p is ONCIRCLE == 0
    if robust_predicates {
        incircle_robust(&a, &b, &c, &p)
    } else {
        incircle_fast(&a, &b, &c, &p)
    }
}

pub fn incircle_robust(a: &[f64], b: &[f64], c: &[f64], p: &[f64]) -> i8 {
    //-- p is INSIDE   == +1
    //-- p is OUTSIDE  == -1
    //-- p is ONCIRCLE == 0
    let re = robust::incircle(
        robust::Coord { x: a[0], y: a[1] },
        robust::Coord { x: b[0], y: b[1] },
        robust::Coord { x: c[0], y: c[1] },
        robust::Coord { x: p[0], y: p[1] },
    );
    if re == 0.0_f64 {
        0
    } else if re.is_sign_positive() {
        return 1;
    } else {
        return -1;
    }
}

pub fn incircle_fast(a: &[f64], b: &[f64], c: &[f64], p: &[f64]) -> i8 {
    //-- p is INSIDE   == +1
    //-- p is OUTSIDE  == -1
    //-- p is ONCIRCLE == 0
    let at = (
        a[0] - p[0],
        a[1] - p[1],
        (a[0] * a[0] + a[1] * a[1]) - (p[0] * p[0] + p[1] * p[1]),
    );
    let bt = (
        b[0] - p[0],
        b[1] - p[1],
        (b[0] * b[0] + b[1] * b[1]) - (p[0] * p[0] + p[1] * p[1]),
    );
    let ct = (
        c[0] - p[0],
        c[1] - p[1],
        (c[0] * c[0] + c[1] * c[1]) - (p[0] * p[0] + p[1] * p[1]),
    );
    let i = at.0 * (bt.1 * ct.2 - bt.2 * ct.1);
    let j = at.1 * (bt.0 * ct.2 - bt.2 * ct.0);
    let k = at.2 * (bt.0 * ct.1 - bt.1 * ct.0);
    let re = i - j + k;
    // println!("INCIRCLE TEST: {}", re);
    if re.abs() < 1e-12 {
        0
    } else if re > 0.0 {
        return 1;
    } else {
        return -1;
    }
}

pub fn bbox2d(pts: &Vec<[f64; 3]>) -> [f64; 4] {
    let mut re: [f64; 4] = [std::f64::MAX, std::f64::MAX, std::f64::MIN, std::f64::MIN];
    for each in pts {
        if each[0] < re[0] {
            re[0] = each[0];
        }
        if each[1] < re[1] {
            re[1] = each[1];
        }
        if each[0] > re[2] {
            re[2] = each[0];
        }
        if each[1] > re[3] {
            re[3] = each[1];
        }
    }
    re
}
