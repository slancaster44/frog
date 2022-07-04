use crate::primatives;
use crate::ray;
use crate::shapes::{Shape};

pub struct Intersection<'a> {
    pub time: f64,
    pub location: primatives::PointT,
    pub shape: &'a (dyn Shape + 'a),
    pub inside: bool, //True if the ray origin is inside the sphere
    pub ray: ray::Ray,
    pub eyev: primatives::Vec3T,
    pub normalv: primatives::Vec3T
}

pub fn new<'a>(t: f64, l: primatives::PointT, s: &'a dyn Shape, r: ray::Ray) -> Intersection {
    let mut nv = s.normal_at(l);
    let ev = -r.direction;

    let is_inside =
        if primatives::dot_product(nv, ev) < 0.0 {
            nv = -nv;
            true 
        } else {
            false
        };

    return Intersection {
        time: t,
        location: l,
        shape: s,
        inside: is_inside,
        ray: r,
        eyev: ev,
        normalv: nv,
    }
}