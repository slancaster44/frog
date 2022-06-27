use crate::matrix;

pub const PI: f64 = std::f64::consts::PI;

pub fn new_translation(x: f64, y: f64, z: f64) -> matrix::Matrix4x4 {
    return matrix::Matrix4x4 {
        contents: [
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0]
        ]
    }
}

pub fn new_scaling(x: f64, y: f64, z: f64) -> matrix::Matrix4x4 {
    return matrix::Matrix4x4 {
        contents: [
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    }
}

pub fn new_rotation_x(deg_radians: f64) -> matrix::Matrix4x4 {
    return matrix::Matrix4x4 {
        contents: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, deg_radians.cos(), -deg_radians.sin(), 0.0],
            [0.0, deg_radians.sin(), deg_radians.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    }
}