pub mod intersection;
pub mod sphere;

use crate::ray;
use crate::primatives;

pub trait Shape {
    fn intersect(&self, r_input: ray::Ray) -> Vec<intersection::Intersection> ;
    fn normal_at(&self, p: primatives::PointT) -> primatives::Vec3T;
}