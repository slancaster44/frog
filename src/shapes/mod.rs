pub mod intersection;
pub mod sphere;

use crate::ray;
use crate::primatives;
use crate::material;

pub trait Shape {
    fn intersect(&self, r_input: ray::Ray) -> Vec<intersection::Intersection> ;
    fn normal_at(&self, p: primatives::PointT) -> primatives::Vec3T;
    fn get_material(&self) -> material::Material;
}