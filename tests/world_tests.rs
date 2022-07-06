#[cfg(test)]

use frog::world;
use frog::shapes::sphere;
use frog::shapes::{Shape};
use frog::primatives;
use frog::color;
use frog::matrix::transformations;
use frog::ray;
use frog::shading;
use frog::light;
use frog::canvas;

#[test]
fn world_creation() {
    let mut w = world::new();
    
    let mut s1 = sphere::new(1.0, primatives::point(0.0, 0.0, 0.0));
    s1.material.color = color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    w.objects.push(&s1);

    let mut s2 = sphere::new(1.0, primatives::point(0.0, 0.0, 0.0));
    let trans = transformations::new_scaling_matrix(0.5, 0.5, 0.5);
    s2 = trans * s2;
    w.objects.push(&s2);

    let r1 = ray::new(primatives::point(0.0, 0.0, -5.0), primatives::vec3(0.0, 0.0, 1.0));
    let intersections = w.intersect(r1);

    assert_eq!(intersections.len(), 4);
    assert_eq!(intersections[0].time, 4.0);
    assert_eq!(intersections[1].time, 4.5);
    assert_eq!(intersections[2].time, 5.5);
    assert_eq!(intersections[3].time, 6.0);

    let intersections = s1.intersect(r1);
    assert_eq!(intersections[1].eyev, primatives::vec3(0.0, 0.0, -1.0));
    assert_eq!(intersections[1].normalv, primatives::vec3(0.0, 0.0, -1.0));
    assert_eq!(intersections[1].inside, false);

    let r2 = ray::new(primatives::point(0.0, 0.0, 0.0), primatives::vec3(0.0, 0.0, 1.0));
    let intersections = s1.intersect(r2);
    assert_eq!(intersections[0].inside, true);
}

#[test]
fn shade_intersection() {
    let mut w = world::new();
    
    let mut s1 = sphere::new(1.0, primatives::point(0.0, 0.0, 0.0));
    s1.material.color = color::new(0.8, 1.0, 0.6);
    s1.material.diffuse = 0.7;
    s1.material.specular = 0.2;
    w.objects.push(&s1);

    let mut s2 = sphere::new(1.0, primatives::point(0.0, 0.0, 0.0));
    let trans = transformations::new_scaling_matrix(0.5, 0.5, 0.5);
    s2 = trans * s2;
    w.objects.push(&s2);

    let r = ray::new(primatives::point(0.0, 0.0, -5.0), primatives::vec3(0.0, 0.0, 1.0));
    let c = w.color_at_ray(r);

    assert_eq!(c, color::new(0.38066, 0.47583, 0.2855));

    let intersects = s1.intersect(r);

    let c = shading::shade_intersection(intersects[1], w.light);
    assert_eq!(c, color::new(0.38066, 0.47583, 0.2855));

    w.light = light::new(color::new(1.0, 1.0, 1.0), primatives::point(0.0, 0.25, 0.0));
    let r = ray::new(primatives::point(0.0, 0.0, 0.0), primatives::vec3(0.0, 0.0, 1.0));

    let intersects = s2.intersect(r);
    let c = shading::shade_intersection(intersects[0], w.light);

    assert_eq!(c, color::new(0.90498, 0.90498, 0.90498));

    let r = ray::new(primatives::point(0.0, 0.0, -5.0), primatives::vec3(0.0, 1.0, 0.0));
    let c = w.color_at_ray(r);

    assert_eq!(c, color::new(0.0, 0.0, 0.0));
}

#[test]
fn draw_world() {
    let mut w = world::new();

    let mut s1 = sphere::new(400.0, primatives::point(0.0, 0.0, 0.0));
    s1.material.color = color::new(0.5, 0.2, 0.7);

    let mut s2 = sphere::new(180.0, primatives::point(100.0, 100.0, 350.0));
    s2.material.color = color::new(0.2, 0.3, 0.8);
    s2.material.specular = 0.2;

    let mut s3 = sphere::new(100.0, primatives::point(-60.0, -60.0, 500.0));
    s3.material.color = color::new(0.2, 0.8, 0.3);
    s3.material.specular = 0.0;

    w.objects.push(&s1);
    w.objects.push(&s2);
    w.objects.push(&s3);
    w.light.location = primatives::point(-400.0, -400.0, 1000.0);

    let mut can = canvas::new(1000, 1000);
    
    for x_loc in 0..can.width {
        for y_loc in 0..can.height {
            let x = (x_loc as i32) - (can.width as i32 / 2);
            let y = (y_loc as i32) - (can.height as i32 / 2);

            let r = ray::new(
                primatives::point(x as f64, y as f64, -1000.0), 
                primatives::vec3(0.0, 0.0, -1.0));

            let c = w.color_at_ray(r);
            can.plot(x, y, c);
        }
    }

    can.antialiased(2).write_to_ppm("out.ppm");
}