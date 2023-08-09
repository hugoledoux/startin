use crate::interpolation;
use crate::Triangulation;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::os::raw::c_double;
use std::os::raw::c_int;
use std::os::raw::c_ulong;

#[no_mangle]
pub extern "C" fn new_triangulation() -> *mut Triangulation {
    let x = Box::new(Triangulation::new());
    let ptr = Box::into_raw(x);
    return ptr;
}

#[no_mangle]
pub extern "C" fn destroy(ptr: *mut Triangulation) {
    unsafe { drop(Box::from_raw(ptr)) };
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
pub extern "C" fn remove(ptr: *mut Triangulation, vi: c_ulong) -> c_int {
    let t = unsafe { ptr.as_mut().unwrap() };
    let re = Triangulation::remove(t, vi as usize);
    match re {
        Ok(_) => return 0,
        Err(_) => return 1,
    };
}

#[no_mangle]
pub extern "C" fn adjacent_vertices_to_vertex(ptr: *mut Triangulation, vi: c_ulong) -> *mut u64 {
    let t = unsafe { ptr.as_mut().unwrap() };
    let re = Triangulation::adjacent_vertices_to_vertex(t, vi as usize);
    let mut adjs: Vec<u64> = Vec::new();
    if re.is_ok() {
        let a = re.unwrap();
        adjs.push(a.len() as u64);
        for each in a {
            adjs.push(each as u64);
        }
    } else {
        adjs.push(0);
    }
    let ptr = adjs.as_mut_ptr();
    std::mem::forget(adjs); // so that it is not destructed at the end of the scope
    ptr
}

#[no_mangle]
pub extern "C" fn insert(
    ptr: *mut Triangulation,
    length: c_int,
    arr: *mut c_double,
    strategy: *const c_char,
) {
    let istrategy = unsafe {
        assert!(!strategy.is_null());
        CStr::from_ptr(strategy)
    };
    let dt = unsafe { ptr.as_mut().unwrap() };
    let slice = unsafe { std::slice::from_raw_parts(arr, length as usize) };
    let iter = slice.chunks(3);
    match istrategy.to_str().unwrap() {
        "AsIs" => {
            for p in iter {
                let _re = Triangulation::insert_one_pt(dt, p[0], p[1], p[2]);
            }
        }
        "BBox" => {
            //-- find the bbox
            let mut bbox = Triangulation::get_bbox(dt);
            //-- "padding" of the bbox to avoid conflicts
            bbox[0] = bbox[0] - 10.0;
            bbox[1] = bbox[1] - 10.0;
            bbox[2] = bbox[2] + 10.0;
            bbox[3] = bbox[3] + 10.0;
            let mut c4: Vec<usize> = Vec::new();
            c4.push(Triangulation::insert_one_pt(dt, bbox[0], bbox[1], 0.0).unwrap());
            c4.push(Triangulation::insert_one_pt(dt, bbox[2], bbox[1], 0.0).unwrap());
            c4.push(Triangulation::insert_one_pt(dt, bbox[2], bbox[3], 0.0).unwrap());
            c4.push(Triangulation::insert_one_pt(dt, bbox[0], bbox[3], 0.0).unwrap());
            for p in iter {
                let _re = Triangulation::insert_one_pt(dt, p[0], p[1], p[2]);
            }
            //-- remove the 4 corners
            for each in &c4 {
                let _re = Triangulation::remove(dt, *each);
            }
        }
        _ => {
            for p in iter {
                let _re = Triangulation::insert_one_pt(dt, p[0], p[1], p[2]);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn interpolate_nn(ptr: *mut Triangulation, px: c_double, py: c_double) -> c_double {
    let t = unsafe { ptr.as_mut().unwrap() };
    let interpol = interpolation::NN {};
    let mut re = interpolation::interpolate(&interpol, t, &vec![[px, py]]);
    let re1 = re.pop().expect("no results");
    return re1.unwrap_or(std::f64::NAN);
}

#[no_mangle]
pub extern "C" fn interpolate_linear(
    ptr: *mut Triangulation,
    px: c_double,
    py: c_double,
) -> c_double {
    let t = unsafe { ptr.as_mut().unwrap() };
    let interpol = interpolation::TIN {};
    let mut re = interpolation::interpolate(&interpol, t, &vec![[px, py]]);
    let re1 = re.pop().expect("no results");
    return re1.unwrap_or(std::f64::NAN);
}

#[no_mangle]
pub extern "C" fn interpolate_nni(ptr: *mut Triangulation, px: c_double, py: c_double) -> c_double {
    let t = unsafe { ptr.as_mut().unwrap() };
    let interpol = interpolation::NNI { precompute: false };
    let mut re = interpolation::interpolate(&interpol, t, &vec![[px, py]]);
    let re1 = re.pop().expect("no results");
    return re1.unwrap_or(std::f64::NAN);
}

#[no_mangle]
pub extern "C" fn interpolate_laplace(
    ptr: *mut Triangulation,
    px: c_double,
    py: c_double,
) -> c_double {
    let t = unsafe { ptr.as_mut().unwrap() };
    let interpol = interpolation::Laplace {};
    let mut re = interpolation::interpolate(&interpol, t, &vec![[px, py]]);
    let re1 = re.pop().expect("no results");
    return re1.unwrap_or(std::f64::NAN);
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
pub extern "C" fn printme(ptr: *mut Triangulation) -> c_int {
    let t = unsafe { ptr.as_mut().unwrap() };
    println!("Vertices: {}", t.number_of_vertices());
    println!("Triangles: {}", t.number_of_triangles());
    println!("Convex points: {}", t.number_of_vertices_on_convex_hull());
    println!("Robust?: {}", t.robust_predicates);
    return 0;
}
