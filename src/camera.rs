use crate::matrix;
use crate::ray;
use crate::primatives;

pub struct Camera {
    pub height: usize,
    pub width: usize,
    pub field_of_view_radians: f64,
    pub transformation: matrix::Matrix4x4,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64
}

pub fn new(h: usize, w: usize, fov: f64) -> Camera {

    //We have to shink or scale the pixel size according
    //to how wide the FOV is vs how much space we have on 
    //the canvas to cram all that information into

    let half_view = (fov / 2.0).tan();
    let aspect_ratio = h as f64 / w as f64;

    let (half_width, half_height) =
        if aspect_ratio >= 1.0 {
            (half_view, half_view / aspect_ratio)
        } else {
            (half_view * aspect_ratio, half_view)
        };

    return Camera {
        height: h,
        width: w,
        field_of_view_radians: fov,
        transformation: matrix::IDENTITY_MATRIX_4X4,
        pixel_size: (half_width * 2.0) / h as f64,
        half_width: half_width,
        half_height: half_height
    }
}

impl Camera {
    pub fn ray_at_pixel(&self, x: usize, y: usize) -> ray::Ray {
        //(x as f64 + *MAGIC CONSTANT*) for good luck
        //TODO: Remove magic constant?
        let x_coord_ray_intersects = self.half_width - ((x as f64 + 0.5) * self.pixel_size);
        let y_coord_ray_intersects = self.half_height - ((y as f64 + 0.5) * self.pixel_size);

        let point_ray_intersects = 
            self.transformation.inverse() * primatives::point(x_coord_ray_intersects, y_coord_ray_intersects, -1.0);
        let ray_origin = 
            self.transformation.inverse() * primatives::point(0.0, 0.0, 0.0);

        let ray_direction = (point_ray_intersects - ray_origin).normalized();

        return ray::new(ray_origin, ray_direction);
    }
}