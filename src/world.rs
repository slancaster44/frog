use crate::shapes::{Shape};
use crate::shapes::intersection;
use crate::light;
use crate::color;
use crate::primatives;
use crate::ray;

pub struct World<'a> {
    pub objects: Vec<&'a (dyn Shape + 'a)>,
    pub light: light::Light
}

pub fn new<'a>() -> World<'a> {
    return World {
        objects: vec![],
        light: light::new(color::new(1.0, 1.0, 1.0), primatives::point(-10.0, 10.0, -10.0))
    }
}

impl World<'_> {
    pub fn intersect<'a>(&'a self, r: ray::Ray) -> Vec<intersection::Intersection<'a>> {
        let mut ret_val = vec![];

        for obj in &self.objects {
            ret_val.extend(obj.intersect(r));
        }

        ret_val.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        return ret_val
    }
}