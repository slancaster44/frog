use std::ops;

use crate::primatives;
use crate::ray;
use crate::matrix;
use crate::material;
use crate::shapes;
use crate::shapes::intersection;

#[derive(Debug, Copy, Clone)]
pub struct Sphere {
    pub radius: f64,
    pub origin: primatives::Tuple,
    pub transformation: matrix::Matrix4x4,
    pub material: material::Material
}

pub fn new(r: f64, o: primatives::Tuple) -> Sphere {
    o.check_type(primatives::TYPE_PNT);
    return Sphere {
        radius: r,
        origin: o,
        transformation: matrix::IDENTITY_MATRIX_4X4,
        material: material::new_default()
    }
}

impl ops::Mul<Sphere> for matrix::Matrix4x4 {
    type Output = Sphere;
    fn mul(self, s: Sphere) -> Sphere {
        return Sphere {
            radius: s.radius,
            origin: s.origin,
            transformation: s.transformation * self,
            material: s.material
        }
    }
}

impl shapes::Shape for Sphere {

    /* (x * x) + (y * y) + (z * z) = (radius * radius) is true for all 
     * points (x, y, z) on a sphere. You can reformulate this into
     * (ray_origin + ray_time * ray_direction) ^ 2 = (radius * radius), which
     * is true for all points that are both on the ray and on the sphere.
     * When you develop this formula, it becomes a quadratic equation.
     * this function solves that quadratic to find the points on the ray
     * that intersect the sphere.
     */
    fn intersect(&self, r_input: ray::Ray) -> Vec<intersection::Intersection> {
        let r = self.transformation.inverse() * r_input;

        let sphere_to_ray = r.origin - self.origin;

        let a = primatives::dot_product(r.direction, r.direction);
        let b = 2.0 * primatives::dot_product(r.direction, sphere_to_ray);
        let c = primatives::dot_product(sphere_to_ray, sphere_to_ray) - (self.radius * self.radius);
        let d = (b * b) - 4.0 * a * c; 

        let t_values: Vec<f64> = 
            if d > 0.0 {
                vec![ (-b + d.sqrt()) / (2.0 * a), (-b - d.sqrt()) / (2.0 * a)]
            } else if d == 0.0 {
                vec![ -b / (2.0 * a) ]

            } else {
                vec![]
            };

        let mut ret_val = vec![];
        for i in t_values {
            let intersect = intersection::new(i, r_input.position(i), self);
            ret_val.push(intersect);
        }

        return ret_val
    }

    fn normal_at(&self, p: primatives::PointT) -> primatives::Vec3T {
        p.check_type(primatives::TYPE_PNT);

        //Remove transformations from input point
        let point = self.transformation.inverse() * p;

        //Find what the normal vector would be if the sphere
        //had no transformations
        let obj_normal = point - self.origin;

        //Reapply the transformations to the normal vector to get actual normal
        let mut world_normal = self.transformation.inverse().transposed() * obj_normal;
        world_normal.w = 0.0;

        return world_normal.normalized()
    }
}