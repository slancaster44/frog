use crate::primatives;
use crate::shapes::{Shape};

pub struct Intersection<'a> {
    pub time: f64,
    pub location: primatives::PointT,
    pub shape: &'a (dyn Shape + 'a)
}

pub fn new<'a>(t: f64, l: primatives::PointT, s: &'a dyn Shape) -> Intersection {
    return Intersection {
        time: t,
        location: l,
        shape: s
    }
}