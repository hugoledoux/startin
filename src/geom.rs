extern crate libc;

use super::Point3d;

pub mod shewchuk {
    extern "C" {
        pub fn exactinit();
        pub fn orient2d(
            pa: *mut libc::c_double,
            pb: *mut libc::c_double,
            pc: *mut libc::c_double,
        ) -> libc::c_double;
        pub fn incircle(
            pa: *mut libc::c_double,
            pb: *mut libc::c_double,
            pc: *mut libc::c_double,
            pp: *mut libc::c_double,
        ) -> libc::c_double;
    }
}

pub fn orient2d(a: &Point3d, b: &Point3d, c: &Point3d) -> i8 {
    //-- CCW    = +1
    //-- CW     = -1
    //-- colinear = 0
    return orient2d_fast(&a, &b, &c);
}

pub fn test_shewchuk() {
    let mut a: Vec<f64> = vec![1.1, 1.110001];
    let mut b: Vec<f64> = vec![5.1, 1.110001];
    let mut c: Vec<f64> = vec![10.1, 1.110001];
    // let mut d: Vec<f64> = vec![1.1, 1.110001, 5.1, 1.110001, 10.1, 1.110001];
    let pa = a.as_mut_ptr();
    let pb = b.as_mut_ptr();
    let pc = c.as_mut_ptr();
    // let pd = d.as_mut_ptr();
    // let re = unsafe { orient2d(pa, pc, pb) };
    let re = unsafe { shewchuk::orient2d(pa, pb, pc) };
    if re == 0.0 {
        println!("COLLINEAR");
    } else if re > 0.0 {
        println!("CCW");
    } else {
        println!("CW");
    }
}

pub fn orient2d_fast(a: &Point3d, b: &Point3d, c: &Point3d) -> i8 {
    //-- CCW    = +1
    //-- CW     = -1
    //-- colinear = 0
    let re: f64 = ((a.x - c.x) * (b.y - c.y)) - ((a.y - c.y) * (b.x - c.x));
    if re.abs() < 1e-12 {
        return 0;
    } else if re > 0.0 {
        return 1;
    } else {
        return -1;
    }
}

pub fn incircle(a: &Point3d, b: &Point3d, c: &Point3d, p: &Point3d) -> i8 {
    //-- p is INSIDE   == +1
    //-- p is OUTSIDE  == -1
    //-- p is ONCIRCLE == 0
    let at = (
        a.x - p.x,
        a.y - p.y,
        (a.x * a.x + a.y * a.y) - (p.x * p.x + p.y * p.y),
    );
    let bt = (
        b.x - p.x,
        b.y - p.y,
        (b.x * b.x + b.y * b.y) - (p.x * p.x + p.y * p.y),
    );
    let ct = (
        c.x - p.x,
        c.y - p.y,
        (c.x * c.x + c.y * c.y) - (p.x * p.x + p.y * p.y),
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
