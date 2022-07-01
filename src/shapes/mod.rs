use crate::primatives;
use crate::ray;
use crate::matrix;

pub trait Shape {
    fn intersect(&self, r: ray::Ray) -> Vec<primatives::Tuple>;
}

pub struct Sphere {
    pub radius: f64,
    pub origin: primatives::Tuple,
    pub transformation: matrix::Matrix4x4
}

pub fn new_sphere(r: f64, o: primatives::Tuple) -> Sphere {
    o.check_type(primatives::TYPE_PNT);
    return Sphere {
        radius: r,
        origin: o,
        transformation: matrix::IDENTITY_MATRIX_4X4
    }
}

impl Shape for Sphere {

    /* (x * x) + (y * y) + (z * z) = (radius * radius) is true for all 
     * points (x, y, z) on a sphere. You can reformulate this into
     * (ray_origin + ray_time * ray_direction) ^ 2 = (radius * radius), which
     * is true for all points that are both on the ray and on the sphere.
     * When you develop this formula, it becomes a quadratic equation.
     * this function solves that quadratic to find the points on the ray
     * that intersect the sphere.
     */
    fn intersect(&self, r_input: ray::Ray) -> Vec<primatives::Tuple> {
        let r = r_input.transform(self.transformation.inverse());

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
            //Note: I only really have a hunch that it actually works this way
            ret_val.push(r_input.position(i));
        }

        return ret_val
    }
}