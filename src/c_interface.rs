use crate::Triangulation;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::os::raw::c_double;
use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn new_triangulation() -> *mut Triangulation {
    let x = Box::new(Triangulation::new());
    let ptr = Box::into_raw(x);
    return ptr;
}

#[no_mangle]
pub extern "C" fn destroy(ptr: *mut Triangulation) -> c_int {
    unsafe { drop(Box::from_raw(ptr)) };
    return 0;
}

#[no_mangle]
pub extern "C" fn insert_one_pt(
    ptr: *mut Triangulation,
    px: c_double,
    py: c_double,
    pz: c_double,
) -> usize {
    let t = unsafe { ptr.as_mut().unwrap() };
    let re = Triangulation::insert_one_pt(t, px, py, pz);
    match re {
        Ok(pointid) => return pointid,
        Err(_) => return 0,
    };
}

#[no_mangle]
pub extern "C" fn remove(
    ptr: *mut Triangulation,
    pointid: usize,
) -> c_int {
    let t = unsafe { ptr.as_mut().unwrap() };
    let re = Triangulation::remove(t, pointid);
    match re {
        Ok(_) => return 0,
        Err(_) => return 1,
    };
}

#[no_mangle]
pub extern "C" fn insert(ptr: *mut Triangulation, length: c_int, arr: *mut c_double) -> c_int {
    let mut duplicates = 0;
    let t = unsafe { ptr.as_mut().unwrap() };
    let pts = unsafe { std::slice::from_raw_parts_mut(arr, length as usize) };
    for i in (0..length as usize).step_by(3) {
        let re = Triangulation::insert_one_pt(t, pts[i], pts[i + 1], pts[i + 2]);
        match re {
            Ok(_) => continue,
            Err(_) => duplicates = duplicates + 1,
        }
    }
    return duplicates;
}

#[no_mangle]
pub extern "C" fn interpolate_nn(ptr: *mut Triangulation, px: c_double, py: c_double) -> c_double {
    let t = unsafe { ptr.as_mut().unwrap() };
    let re = Triangulation::interpolate_nn(t, px, py);
    return re.unwrap_or_else(|| std::f64::NAN);
}

#[no_mangle]
pub extern "C" fn interpolate_linear(
    ptr: *mut Triangulation,
    px: c_double,
    py: c_double,
) -> c_double {
    let t = unsafe { ptr.as_mut().unwrap() };
    let re = Triangulation::interpolate_tin_linear(t, px, py);
    return re.unwrap_or_else(|| std::f64::NAN);
}

#[no_mangle]
pub extern "C" fn interpolate_nni(ptr: *mut Triangulation, px: c_double, py: c_double) -> c_double {
    let t = unsafe { ptr.as_mut().unwrap() };
    let re = Triangulation::interpolate_nni(t, px, py);
    return re.unwrap_or_else(|| std::f64::NAN);
}

#[no_mangle]
pub extern "C" fn interpolate_laplace(
    ptr: *mut Triangulation,
    px: c_double,
    py: c_double,
) -> c_double {
    let t = unsafe { ptr.as_mut().unwrap() };
    let re = Triangulation::interpolate_laplace(t, px, py);
    return re.unwrap_or_else(|| std::f64::NAN);
}

#[no_mangle]
pub extern "C" fn write_obj(ptr: *mut Triangulation, s: *const c_char) -> c_int {
    let t = unsafe { ptr.as_mut().unwrap() };
    let path = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };
    let re = Triangulation::write_obj(t, path.to_str().unwrap().to_string());
    if re.is_err() {
        return 1;
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn write_ply(ptr: *mut Triangulation, s: *const c_char) -> c_int {
    let t = unsafe { ptr.as_mut().unwrap() };
    let path = unsafe {
        assert!(!s.is_null());
        CStr::from_ptr(s)
    };
    let re = Triangulation::write_ply(t, path.to_str().unwrap().to_string());
    if re.is_err() {
        return 1;
    }
    return 0;
}

#[no_mangle]
pub extern "C" fn get_snap_tolerance(ptr: *mut Triangulation) -> c_double {
    let t = unsafe { ptr.as_mut().unwrap() };
    let re = Triangulation::get_snap_tolerance(t);
    return re;
}

#[no_mangle]
pub extern "C" fn set_snap_tolerance(ptr: *mut Triangulation, tolerance: c_double) -> c_double {
    let t = unsafe { ptr.as_mut().unwrap() };
    let re = Triangulation::set_snap_tolerance(t, tolerance);
    return re;
}

#[no_mangle]
pub extern "C" fn debug(ptr: *mut Triangulation) -> c_int {
    let t = unsafe { ptr.as_mut().unwrap() };
    println!("Vertices: {}", t.number_of_vertices());
    println!("Triangles: {}", t.number_of_triangles());
    println!("Convex points: {}", t.number_of_vertices_on_convex_hull());
    println!("Robust?: {}", t.robust_predicates);
    return 0;
}
