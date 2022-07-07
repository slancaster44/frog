use crate::matrix;
use crate::primatives;

pub const PI: f64 = std::f64::consts::PI;

pub fn new_translation_matrix(x: f64, y: f64, z: f64) -> matrix::Matrix4x4 {
    return matrix::Matrix4x4 {
        contents: [
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0]
        ]
    }
}

pub fn new_scaling_matrix(x: f64, y: f64, z: f64) -> matrix::Matrix4x4 {
    return matrix::Matrix4x4 {
        contents: [
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    }
}

pub fn new_rotation_x_matrix(deg_radians: f64) -> matrix::Matrix4x4 {
    return matrix::Matrix4x4 {
        contents: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, deg_radians.cos(), -deg_radians.sin(), 0.0],
            [0.0, deg_radians.sin(), deg_radians.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    }
}

pub fn new_rotation_y_matrix(deg_radians: f64) -> matrix::Matrix4x4 {
    return matrix::Matrix4x4 {
        contents: [
            [deg_radians.cos(), 0.0, deg_radians.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-deg_radians.sin(), 0.0, deg_radians.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    }
}

pub fn new_rotation_z_matrix(deg_radians: f64) -> matrix::Matrix4x4 {
    return matrix::Matrix4x4 {
        contents: [
            [deg_radians.cos(), -deg_radians.sin(), 0.0, 0.0],
            [deg_radians.sin(), deg_radians.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    }
}

pub fn new_shearing_matrix(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> matrix::Matrix4x4 {
    return matrix::Matrix4x4 {
        contents: [
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    }
}

pub fn new_view_transformation_matrix(
    eye_loc: primatives::PointT,
    point_viewed: primatives::PointT,
    up: primatives::Vec3T
) -> matrix::Matrix4x4 {
    eye_loc.check_type(primatives::TYPE_PNT);
    point_viewed.check_type(primatives::TYPE_PNT);
    up.check_type(primatives::TYPE_VEC);

    let up = up.normalized();

    let forwardv = (point_viewed - eye_loc).normalized();
    let leftv = primatives::cross_product(forwardv, up);

    let up = primatives::cross_product(leftv, forwardv);

    let m = matrix::Matrix4x4{
        contents: [
            [leftv.x, leftv.y, leftv.z, 0.0],
            [up.x, up.y, up.z, 0.0],
            [-forwardv.x, -forwardv.y, -forwardv.z, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]
    };

    return m * new_translation_matrix(-eye_loc.x, -eye_loc.y, -eye_loc.z);
}