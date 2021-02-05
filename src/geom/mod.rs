//! # geom
//!
//! Geometric predicates functions are declared here.
//! Notice that [Shewchuk's predicates](https://www.cs.cmu.edu/~quake/robust.html)
//! are used (activated by default by startin), but also possible to rely on floating-point
//! arithmetic (not recommended).

mod exactpred;

pub fn det3x3t(a: &[f64], b: &[f64], c: &[f64]) -> f64 {
    ((a[0] - c[0]) * (b[1] - c[1])) - ((a[1] - c[1]) * (b[0] - c[0]))
}

pub fn area_triangle(a: &[f64], b: &[f64], c: &[f64]) -> f64 {
    det3x3t(a, b, c) / 2.0
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
    if robust_predicates == true {
        return orient2d_robust(&a, &b, &c);
    } else {
        return orient2d_fast(&a, &b, &c);
    }
}

pub fn orient2d_robust(a: &[f64], b: &[f64], c: &[f64]) -> i8 {
    //-- CCW    = +1
    //-- CW     = -1
    //-- colinear = 0
    // let re = unsafe { shewchuk::orient2d(a.as_ptr(), b.as_ptr(), c.as_ptr()) };
    let re = exactpred::orient2d(a, b, c);
    if re == 0.0 {
        return 0;
    } else if re > 0.0 {
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
        return 0;
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
    if robust_predicates == true {
        return incircle_robust(&a, &b, &c, &p);
    } else {
        return incircle_fast(&a, &b, &c, &p);
    }
}

pub fn incircle_robust(a: &[f64], b: &[f64], c: &[f64], p: &[f64]) -> i8 {
    //-- CCW    = +1
    //-- CW     = -1
    //-- colinear = 0
    // let re = unsafe { shewchuk::incircle(a.as_ptr(), b.as_ptr(), c.as_ptr(), p.as_ptr()) };
    let re = exactpred::incircle(a, b, c, p);
    if re == 0.0 {
        return 0;
    } else if re > 0.0 {
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
        return 0;
    } else if re > 0.0 {
        return 1;
    } else {
        return -1;
    }
}
