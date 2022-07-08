use crate::shapes::{Shape};
use crate::shapes::intersection;
use crate::light;
use crate::color;
use crate::primatives;
use crate::ray;
use crate::shading;
use crate::camera;
use crate::canvas;

pub struct World<'a> {
    pub objects: Vec<&'a (dyn Shape + 'a)>,
    pub light: light::Light,
    pub camera: camera::Camera
}

pub fn new<'a>(c: camera::Camera) -> World<'a> {
    return World {
        objects: vec![],
        light: light::new(color::new(1.0, 1.0, 1.0), primatives::point(-10.0, 10.0, -10.0)),
        camera: c
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

    pub fn color_at_ray<'a>(&'a self, r: ray::Ray) -> color::Color {
        let intersections = self.intersect(r);
        if intersections.len() == 0 {
            return color::new(0.0, 0.0, 0.0)
        } 
            
        return shading::shade_intersection(intersections[0], self.light)
    }

    pub fn render_to_canvas(&self) -> canvas::Canvas {
        let mut c = canvas::new(self.camera.width, self.camera.height);
        c.origin = (0, 0);

        for canvas_x_coord in 0..c.width {
            for canvas_y_coord in 0..c.height {
                let r = self.camera.ray_at_pixel(canvas_x_coord, canvas_y_coord);
                let color = self.color_at_ray(r);
                c.plot(canvas_x_coord as i32, canvas_y_coord as i32, color);
            }
        }

        return c;
    }
}