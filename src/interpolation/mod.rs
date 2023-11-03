use crate::Attr;
use crate::StartinError;
use crate::Triangulation;
use kdbush::KDBush;

use crate::geom;

pub trait Interpolant<T> {
    fn interpolate(
        &self,
        dt: &mut Triangulation<T>,
        locations: &Vec<[f64; 2]>,
    ) -> Vec<Result<T, StartinError>>;
}

pub fn interpolate<T>(
    interpolant: &impl Interpolant<T>,
    dt: &mut Triangulation<T>,
    locs: &Vec<[f64; 2]>,
) -> Vec<Result<T, StartinError>> {
    interpolant.interpolate(dt, locs)
}

/// Estimation of z-value with interpolation: IDW
/// (this function doesn't use the TIN at all, added here for
/// convenience and teaching purposes)
pub struct IDW {
    pub radius: f64,
    pub power: f64,
}
impl<T: Attr> Interpolant<T> for IDW {
    fn interpolate(
        &self,
        dt: &mut Triangulation<T>,
        locs: &Vec<[f64; 2]>,
    ) -> Vec<Result<T, StartinError>> {
        //-- build a kd-tree
        let mut allpts: Vec<(f64, f64)> = Vec::new();
        for i in 0..dt.stars.len() {
            allpts.push((dt.stars[i].pt[0], dt.stars[i].pt[1]));
        }
        let index = KDBush::create(allpts, kdbush::DEFAULT_NODE_SIZE);
        //-- perform interpolations
        let mut re = Vec::new();
        for p in locs {
            let mut ns: Vec<usize> = Vec::new();
            index.within(p[0], p[1], self.radius, |id| ns.push(id));
            if ns.is_empty() {
                re.push(Err(StartinError::SearchCircleEmpty));
            } else {
                let mut weights: Vec<f64> = Vec::new();
                let mut exisiting = false;
                let mut value: T = T::default();
                for each in &ns {
                    let d = geom::distance2d(p, &dt.stars[*each].pt);
                    if d <= dt.get_snap_tolerance() {
                        exisiting = true;
                        value = dt.stars[*each].attr.clone();
                        break;
                    }
                    weights.push(d.powf(-self.power));
                }
                if exisiting {
                    re.push(Ok(value));
                } else {
                    let mut z = T::default();
                    for (i, w) in weights.iter().enumerate() {
                        z += dt.stars[ns[i]].attr.clone() * *w;
                    }
                    re.push(Ok(z / weights.iter().sum::<f64>()));
                }
            }
        }
        re
    }
}

/// Estimation of z-value with interpolation: Laplace interpolation
///
/// Details about Laplace: <http://dilbert.engr.ucdavis.edu/~suku/nem/index.html>, which
/// is a variation of nni with distances instead of stolen areas, which yields a much
/// faster implementation.
pub struct Laplace {}
impl<T: Attr> Interpolant<T> for Laplace {
    fn interpolate(
        &self,
        dt: &mut Triangulation<T>,
        locs: &Vec<[f64; 2]>,
    ) -> Vec<Result<T, StartinError>> {
        let mut re: Vec<Result<T, StartinError>> = Vec::new();
        for p in locs {
            //-- cannot interpolate if no TIN
            if dt.is_init == false {
                re.push(Err(StartinError::EmptyTriangulation));
                continue;
            }
            //-- no extrapolation
            let loc = dt.locate(p[0], p[1]);
            match loc {
                Ok(_tr) => {
                    match dt.insert_one_pt(p[0], p[1], T::default()) {
                        Ok(pi) => {
                            //-- no extrapolation
                            if dt.is_vertex_convex_hull(pi) {
                                //-- interpolation point was added on boundary of CH
                                //-- nothing to be done, Voronoi cell is unbounded
                                let _rr = dt.remove(pi);
                                re.push(Err(StartinError::OutsideConvexHull));
                            } else {
                                let l = &dt.stars[pi].link;
                                let mut centres: Vec<Vec<f64>> = Vec::new();
                                for (i, v) in l.iter().enumerate() {
                                    let j = l.next_index(i);
                                    centres.push(geom::circle_centre(
                                        &dt.stars[pi].pt,
                                        &dt.stars[*v].pt,
                                        &dt.stars[l[j]].pt,
                                    ));
                                }
                                let mut weights: Vec<f64> = Vec::new();
                                for (i, v) in l.iter().enumerate() {
                                    // fetch 2 voronoi centres
                                    let e =
                                        geom::distance2d(&centres[i], &centres[l.prev_index(i)]);
                                    let w = geom::distance2d(&dt.stars[pi].pt, &dt.stars[*v].pt);
                                    weights.push(e / w);
                                }
                                let mut z = T::default();
                                for (i, v) in l.iter().enumerate() {
                                    z += dt.stars[*v].attr.clone() * weights[i];
                                }
                                let sumweights: f64 = weights.iter().sum();
                                //-- delete the interpolation location point
                                let _rr = dt.remove(pi);
                                re.push(Ok(z / sumweights));
                            }
                        }
                        Err(e) => re.push(Ok(dt.stars[e].attr.clone())),
                    }
                }
                Err(_e) => re.push(Err(StartinError::OutsideConvexHull)),
            }
        }
        re
    }
}

/// Estimation of z-value with interpolation: nearest/closest neighbour
pub struct NN {}
impl<T: Attr> Interpolant<T> for NN {
    fn interpolate(
        &self,
        dt: &mut Triangulation<T>,
        locs: &Vec<[f64; 2]>,
    ) -> Vec<Result<T, StartinError>> {
        let mut re = Vec::new();
        for p in locs {
            //-- cannot interpolation if no TIN
            if dt.is_init == false {
                re.push(Err(StartinError::EmptyTriangulation));
                continue;
            }
            //-- TODO: should interpolate_nn() extrapolate?
            match dt.closest_point(p[0], p[1]) {
                Ok(vi) => re.push(Ok(dt.stars[vi].attr.clone())),
                Err(why) => re.push(Err(why)),
            }
        }
        re
    }
}

/// Estimation of z-value with interpolation: linear in TIN
pub struct TIN {}
impl<T: Attr> Interpolant<T> for TIN {
    fn interpolate(
        &self,
        dt: &mut Triangulation<T>,
        locs: &Vec<[f64; 2]>,
    ) -> Vec<Result<T, StartinError>> {
        let mut re = Vec::new();
        for p in locs {
            //-- cannot interpolate if no TIN
            if dt.is_init == false {
                re.push(Err(StartinError::EmptyTriangulation));
                continue;
            }
            //-- no extrapolation
            let loc = dt.locate(p[0], p[1]);
            match loc {
                Ok(tr) => {
                    let q: [f64; 3] = [p[0], p[1], 0.0];
                    let a0: f64 =
                        geom::area_triangle(&q, &dt.stars[tr.v[1]].pt, &dt.stars[tr.v[2]].pt);
                    let a1: f64 =
                        geom::area_triangle(&q, &dt.stars[tr.v[2]].pt, &dt.stars[tr.v[0]].pt);
                    let a2: f64 =
                        geom::area_triangle(&q, &dt.stars[tr.v[0]].pt, &dt.stars[tr.v[1]].pt);
                    let mut total = T::default();
                    total += dt.stars[tr.v[0]].attr.clone() * a0;
                    total += dt.stars[tr.v[1]].attr.clone() * a1;
                    total += dt.stars[tr.v[2]].attr.clone() * a2;
                    re.push(Ok(total / (a0 + a1 + a2)));
                }
                Err(_e) => re.push(Err(StartinError::OutsideConvexHull)),
            }
        }
        re
    }
}

/// Estimation of z-value with interpolation: natural neighbour interpolation (nni),
/// also called Sibson's interpolation
pub struct NNI {
    pub precompute: bool,
}
impl<T: Attr> Interpolant<T> for NNI {
    fn interpolate(
        &self,
        dt: &mut Triangulation<T>,
        locs: &Vec<[f64; 2]>,
    ) -> Vec<Result<T, StartinError>> {
        //-- store temporarily all the Voronoi cells areas
        let mut vorareas: Vec<f64> = Vec::new();
        if self.precompute {
            vorareas.reserve_exact(dt.stars.len());
            vorareas.push(0.);
            for vi in 1..dt.stars.len() {
                if dt.stars[vi].is_deleted() == false {
                    vorareas.push(dt.voronoi_cell_area(vi, true).unwrap());
                } else {
                    vorareas.push(0.);
                }
            }
        }
        let mut re = Vec::new();
        for p in locs {
            //-- cannot interpolate if no TIN
            if dt.is_init == false {
                re.push(Err(StartinError::EmptyTriangulation));
                continue;
            }
            //-- no extrapolation
            let loc = dt.locate(p[0], p[1]);
            match loc {
                Ok(_tr) => {
                    match dt.insert_one_pt(p[0], p[1], T::default()) {
                        Ok(pi) => {
                            //-- no extrapolation
                            if dt.is_vertex_convex_hull(pi) {
                                //-- interpolation point was added on boundary of CH
                                //-- nothing to be done, Voronoi cell is unbounded
                                let _rr = dt.remove(pi);
                                re.push(Err(StartinError::OutsideConvexHull));
                            } else {
                                let nns = dt.adjacent_vertices_to_vertex(pi).unwrap();
                                let mut weights: Vec<f64> = Vec::new();
                                for nn in &nns {
                                    let a = dt.voronoi_cell_area(*nn, true).unwrap();
                                    weights.push(a);
                                }
                                let newarea = dt.voronoi_cell_area(pi, true).unwrap();
                                let _rr = dt.remove(pi);
                                for (i, nn) in nns.iter().enumerate() {
                                    if self.precompute {
                                        weights[i] = vorareas[*nn] - weights[i];
                                    } else {
                                        //-- TODO : is it faster to save them?!
                                        weights[i] =
                                            dt.voronoi_cell_area(*nn, true).unwrap() - weights[i];
                                    }
                                }
                                let mut z: T = T::default();
                                for (i, nn) in nns.iter().enumerate() {
                                    z += dt.stars[*nn].attr.clone() * weights[i];
                                }
                                re.push(Ok(z / newarea));
                            }
                        }
                        Err(e) => re.push(Ok(dt.stars[e].attr.clone())),
                    }
                }
                Err(_e) => re.push(Err(StartinError::OutsideConvexHull)),
            }
        }
        re
    }
}
