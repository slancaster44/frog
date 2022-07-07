use crate::matrix;

pub struct Camera {
    pub height: usize,
    pub width: usize,
    pub field_of_view_radians: f64,
    pub transform: matrix::Matrix4x4
}

pub fn new(h: usize, w: usize, fov: f64) -> Camera {
    return Camera {
        height: h,
        width: w,
        field_of_view_radians: fov,
        transform: matrix::IDENTITY_MATRIX_4X4
    }
}