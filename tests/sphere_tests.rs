#[cfg(test)]

use frog::primatives;
use frog::shapes;
use frog::shapes::{Shape};
use frog::ray;
use frog::matrix::transformations;
use frog::canvas;
use frog::color;
use frog::material;

#[test]
#[should_panic]
fn sphere_creation() {
    let p1 = primatives::point(0.0, 0.0, 0.0);
    let radius = 2.0;

    let c1 = shapes::sphere::new(radius, p1);
    assert_eq!(p1, c1.origin);
    assert_eq!(radius, c1.radius);
    assert_eq!(material::new_default(), c1.material);

    let v1 = primatives::vec3(0.0, 0.0, 1.0);
    shapes::sphere::new(radius, v1);
}

#[test]
fn ray_sphere_intersection() {
    let s1 = shapes::sphere::new(1.0, primatives::point(0.0, 0.0, 0.0));

    let r1 = ray::new(primatives::point(0.0, 0.0, -5.0), primatives::vec3(0.0, 0.0, 1.0));
    let intersections = s1.intersect(r1);
    
    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].location, r1.position(6.0));
    assert_eq!(intersections[1].location, r1.position(4.0));

    let s2 = shapes::sphere::new(1.0, primatives::point(0.0, 0.0, 0.0));
    let r2 = ray::new(primatives::point(0.0, 1.0, -5.0), primatives::vec3(0.0, 0.0, 1.0));
    let intersections = s2.intersect(r2);

    assert_eq!(intersections.len(), 1);
    assert_eq!(intersections[0].location, r2.position(5.0));

    let s3 = shapes::sphere::new(1.0, primatives::point(0.0, 0.0, 1.0));
    let r3 = ray::new(primatives::point(0.0, 2.0, -5.0), primatives::vec3(0.0, 0.0, 1.0));
    let intersections = s3.intersect(r3);

    assert_eq!(intersections.len(), 0);

    //Ensures that intersections behind ray origin are detected
    let s4 = shapes::sphere::new(1.0, primatives::point(0.0, 0.0, 0.0));
    let r4 = ray::new(primatives::point(0.0, 0.0, 0.0,), primatives::vec3(0.0, 0.0, 1.0));
    let intersections = s4.intersect(r4);

    assert_eq!(intersections.len(), 2);

    let s5 = shapes::sphere::new(2.0, primatives::point(0.0, 0.0, 0.0));
    let r5 = ray::new(primatives::point(0.0, 0.0, 0.0), primatives::vec3(0.0, 0.0, 1.0));
    let intersections = s5.intersect(r5);
    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].location, primatives::point(0.0, 0.0, 2.0));
    assert_eq!(intersections[1].location, primatives::point(0.0, 0.0, -2.0));

    let r = ray::new(primatives::point(0.0, 0.0, -5.0), primatives::vec3(0.0, 0.0, 1.0));
    let mut s = shapes::sphere::new(1.0, primatives::point(0.0, 0.0, 0.0));
    s = transformations::new_scaling_matrix(2.0, 2.0, 2.0) * s;

    let intersections = s.intersect(r);
    assert_eq!(intersections.len(), 2);
    assert_eq!(intersections[0].location, r.position(7.0));
    assert_eq!(intersections[1].location, r.position(3.0));

    let r = ray::new(primatives::point(0.0, 0.0, -5.0), primatives::vec3(0.0, 0.0, 1.0));
    let mut s = shapes::sphere::new(1.0, primatives::point(0.0, 0.0, 0.0));
    s = transformations::new_translation_matrix(5.0, 0.0, 0.0) * s;

    let intersections = s.intersect(r);
    assert_eq!(intersections.len(), 0);

    let r = ray::new(primatives::point(300.0, 300.0, 0.0), primatives::vec3(300.0, 300.0, 300.0));
    let mut s = shapes::sphere::new(1.0, primatives::point(300.0, 300.0, 300.0));
    s = transformations::new_scaling_matrix(300.0, 300.0, 300.0) * s;
    let intersections = s.intersect(r);
    assert_eq!(intersections.len(), 2);
}

#[test]
#[ignore]
fn draw_sphere_intersection() {
    let s = shapes::sphere::new(75.0, primatives::point(75.0, 75.0, 0.0));
    let c1 = color::new(0.5, 1.0, 1.0);
    let mut this_canvas = canvas::new(150, 150);
    this_canvas.origin = (0, 0);

    for x_coord in 0..150 {
        for y_coord in 0..150 {
            let r = ray::new(primatives::point(x_coord as f64, y_coord as f64, 0.0), primatives::vec3(0.0, 0.0, 1.0));
            let intersections = s.intersect(r);
            if intersections.len() != 0 {
                this_canvas.plot(x_coord, y_coord, c1);
            }
        }
    }

    this_canvas.write_to_ppm("sphere_2d.ppm");
}

#[test]
fn sphere_normal() {
    let s = shapes::sphere::new(1.0, primatives::point(0.0, 0.0, 0.0));
    let n = s.normal_at(primatives::point(1.0, 0.0, 0.0));

    n.check_type(primatives::TYPE_VEC);
    assert_eq!(n, primatives::vec3(1.0, 0.0, 0.0));

    assert_eq!(n.normalized(), n);

    let s = shapes::sphere::new(2.0, primatives::point(0.0, 0.0, 0.0));
    let n = s.normal_at(primatives::point(0.0, 2.0, 0.0));

    n.check_type(primatives::TYPE_VEC);
    assert_eq!(n, primatives::vec3(0.0, 1.0, 0.0));

    assert_eq!(n.normalized(), n);

    let s = shapes::sphere::new(2.0, primatives::point(1.0, 1.0, 1.0));
    let n = s.normal_at(primatives::point(1.0, 1.0, 3.0));

    n.check_type(primatives::TYPE_VEC);
    assert_eq!(n, primatives::vec3(0.0, 0.0, 1.0));

    assert_eq!(n.normalized(), n);

    let mut s = shapes::sphere::new(1.0, primatives::point(0.0, 0.0, 0.0));
    s = transformations::new_translation_matrix(0.0, 1.0, 0.0) * s;
    let n = s.normal_at(primatives::point(0.0, 1.70711, -0.70711));

    n.check_type(primatives::TYPE_VEC);
    assert_eq!(n, primatives::vec3(0.0, 0.70711, -0.70711));

    assert_eq!(n.normalized(), n);
    
    let mut s = shapes::sphere::new(1.0, primatives::point(0.0, 0.0, 0.0));
    s = transformations::new_scaling_matrix(1.0, 0.5, 1.0) * s;
    s = transformations::new_rotation_z_matrix(transformations::PI / 5.0) * s;
    let n = s.normal_at(primatives::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));

    n.check_type(primatives::TYPE_VEC);
    assert_eq!(n, primatives::vec3(0.0, 0.97014, -0.24254));

    assert_eq!(n.normalized(), n);
}