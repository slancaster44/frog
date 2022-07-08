#[cfg(test)]

use frog::camera;
use frog::matrix;
use frog::primatives;
use frog::shapes::sphere;
use frog::world;
use frog::color;
use frog::matrix::transformations;

#[test]
fn camera_creation() {
    let c = camera::new(160, 120, 3.14/2.0);

    assert_eq!(c.height, 160);
    assert_eq!(c.width, 120);
    assert_eq!(c.field_of_view_radians, 3.14/2.0);
    assert_eq!(c.transformation, matrix::IDENTITY_MATRIX_4X4);
}

#[test]
fn pixel_size_test() {
    let pi = std::f64::consts::PI;

    let c = camera::new(200, 125, pi/2.0);
    assert_eq!(c.pixel_size, 0.009999999999999998);

    let c = camera::new(125, 200, pi/2.0);
    assert_eq!(c.pixel_size, 0.009999999999999998);
}

#[test]
fn camera_ray_creation() {
    let pi = std::f64::consts::PI;

    let c = camera::new(201, 101, pi / 2.0);
    let r = c.ray_at_pixel(100, 50);

    let expected_r_origin = primatives::point(0.0, 0.0, 0.0);
    let expected_r_vec = primatives::vec3(0.0, 0.0, -1.0);

    assert_eq!(r.origin, expected_r_origin);
    assert_eq!(r.direction, expected_r_vec);

    let c = camera::new(201, 101, pi / 2.0);
    let r = c.ray_at_pixel(0, 0);

    let expected_r_origin = primatives::point(0.0, 0.0, 0.0);
    let expected_r_vec = primatives::vec3(0.66519, 0.33259, -0.66851);

    assert_eq!(r.origin, expected_r_origin);
    assert_eq!(r.direction, expected_r_vec);

    let mut c = camera::new(201, 101, pi / 2.0);
    c.transformation = transformations::new_rotation_y_matrix(pi / 4.0) *
        transformations::new_translation_matrix(0.0, -2.0, 5.0);
    let r = c.ray_at_pixel(100, 50);

    let expected_r_origin = primatives::point(0.0, 2.0, -5.0);
    let expected_r_vec = primatives::vec3(2.0_f64.sqrt(), 0.0, -2.0_f64.sqrt());

    assert_eq!(r.origin, expected_r_origin);
    assert_eq!(r.direction, expected_r_vec.normalized());


}

#[test]
#[ignore]
fn draw_world_with_camera() {
    let c = camera::new(1000, 1000, std::f64::consts::PI / 2.0);
    let mut w = world::new(c);

    let mut s1 = sphere::new(600.0, primatives::point(0.0, 0.0, -200.0));
    s1.material.color = color::new(0.5, 0.2, 0.7);

    let mut s2 = sphere::new(180.0, primatives::point(100.0, 100.0, 350.0));
    s2.material.color = color::new(0.2, 0.3, 0.8);
    s2.material.specular = 0.2;

    let mut s3 = sphere::new(100.0, primatives::point(-60.0, -60.0, 700.0));
    s3.material.color = color::new(0.2, 0.8, 0.3);
    s3.material.specular = 0.0;

    w.objects.push(&s1);
    w.objects.push(&s2);
    w.objects.push(&s3);

    w.light.location = primatives::point(-400.0, -400.0, 1000.0);

    let el = primatives::point(0.0, 0.0, 1000.0);
    let pv = primatives::point(0.0, 0.0, 0.0);
    let up = primatives::vec3(0.0, -1.0, 0.0);
    w.camera.transformation = transformations::new_view_transformation_matrix(el, pv, up);

    let can = w.render_to_canvas();
    can.antialiased(2).write_to_ppm("out.ppm");
}