#[cfg(test)]
use frog::ray;
use frog::primatives;
use frog::matrix::transformations;

#[test]
fn ray_creation() {
    let p1 = primatives::point(0.0, 0.0, 0.0);
    let v1 = primatives::vec3(1.0, 1.0, 1.0);
    let r = ray::new(p1, v1);
    assert_eq!(r.direction, v1);
    assert_eq!(r.origin, p1);
}

#[test]
#[should_panic]
fn origin_panic() {
    let v1 = primatives::vec3(1.0, 1.0, 1.0);
    ray::new(v1, v1);
}

#[test]
#[should_panic]
fn direction_panic() {
    let p1 = primatives::point(1.0, 1.0, 1.0);
    ray::new(p1, p1);
}

#[test]
fn ray_cast() {
    let start = primatives::point(2.0, 3.0, 4.0);
    let dir = primatives::vec3(1.0, 0.0, 0.0);
    let r = ray::new(start, dir);

    let expected = primatives::point(2.0, 3.0, 4.0);
    assert_eq!(expected, r.position(0.0));

    let expected = primatives::point(3.0, 3.0, 4.0);
    assert_eq!(expected, r.position(1.0));

    let expected = primatives::point(1.0, 3.0, 4.0);
    assert_eq!(expected, r.position(-1.0));

    let expected = primatives::point(4.5, 3.0, 4.0);
    assert_eq!(expected, r.position(2.5));

    let start = primatives::point(0.0, 0.0, -5.0);
    let dir = primatives::vec3(0.0, 0.0, 1.0);

    let expected = primatives::point(0.0, 0.0, -1.0);
    let r = ray::new(start, dir);
    assert_eq!(expected, r.position(4.0));
}

#[test]
fn ray_transformations() {
    let r1 = ray::new(primatives::point(1.0, 2.0, 3.0), primatives::vec3(0.0, 1.0, 0.0));
    let m = transformations::new_translation_matrix(3.0, 4.0, 5.0);

    let r2 = m * r1;
    let expected_origin = primatives::point(4.0, 6.0, 8.0);
    let expected_direction = primatives::vec3(0.0, 1.0, 0.0);

    assert_eq!(r2.origin, expected_origin);
    assert_eq!(r2.direction, expected_direction);

    let r = ray::new(primatives::point(1.0, 2.0, 3.0), primatives::vec3(0.0, 1.0, 0.0));
    let m = transformations::new_scaling_matrix(2.0, 3.0, 4.0);

    let r3 = m * r;
    let expected_origin = primatives::point(2.0, 6.0, 12.0);
    let expected_direction = primatives::vec3(0.0, 3.0, 0.0);

    assert_eq!(r3.origin, expected_origin);
    assert_eq!(r3.direction, expected_direction);
}