#[cfg(test)]

use frog::matrix::transformations;
use frog::primatives;
use frog::canvas;
use frog::color;

#[test]
fn translation() {
    let translation = transformations::new_translation_matrix(5.0, -3.0, 2.0);
    let p1 = primatives::point(-3.0, 4.0, 5.0);

    let tranformed_point = translation * p1;
    let expected = primatives::point(2.0, 1.0, 7.0);
    assert_eq!(tranformed_point, expected);

    let inv = translation.inverse();
    let trans_pnt_2 = inv * tranformed_point;
    assert_eq!(trans_pnt_2, p1);

    //zero in vectors w-value should cause translation to
    //do nothing
    let v1 = primatives::vec3(-3.0, 4.0, 5.0);
    let v2 = translation * v1;
    assert_eq!(v1, v2);
}

#[test]
fn scaling() {
    let transform = transformations::new_scaling_matrix(2.0, 3.0, 4.0);

    let p1 = primatives::point(-4.0, 6.0, 8.0);
    let expected = primatives::point(-8.0, 18.0, 32.0);
    let result = transform * p1;
    assert_eq!(result, expected);

    let result = transform.inverse() * result;
    assert_eq!(p1, result);

    let v1 = primatives::vec3(-4.0, 6.0, 8.0);
    let expected = primatives::vec3(-8.0, 18.0, 32.0);
    let result = transform * v1;
    assert_eq!(result, expected);

    let result = transform.inverse() * result;
    assert_eq!(v1, result);
}

#[test]
fn rotation_x() {
    let p1 = primatives::point(0.0, 1.0, 0.0);

    let pi = transformations::PI;
    let half_quarter = transformations::new_rotation_x_matrix(pi / 4.0);
    let full_quarter = transformations::new_rotation_x_matrix(pi / 2.0);
    
    let root_2_over_2 = (2.0_f64).sqrt() / 2.0;
    let expected = primatives::point(0.0, root_2_over_2, root_2_over_2);
    let expected2 = primatives::point(0.0, 0.0, 1.0);
    assert_eq!(expected, half_quarter * p1);
    assert_eq!(expected2, full_quarter * p1);

    assert_eq!(p1, half_quarter.inverse() * expected);
    assert_eq!(p1, full_quarter.inverse() * expected2);

}

#[test]
fn rotation_y() {
    let p1 = primatives::point(0.0, 0.0, 1.0);

    let pi = transformations::PI;
    let half_quarter = transformations::new_rotation_y_matrix(pi / 4.0);
    let full_quarter = transformations::new_rotation_y_matrix(pi / 2.0);

    let root_2_over_2 = (2.0_f64).sqrt() / 2.0;
    let expected = primatives::point(root_2_over_2, 0.0, root_2_over_2);
    let expected2 = primatives::point(1.0, 0.0, 0.0);

    assert_eq!(expected, half_quarter * p1);
    assert_eq!(expected2, full_quarter * p1);

    assert_eq!(p1, half_quarter.inverse() * expected);
    assert_eq!(p1, full_quarter.inverse() * expected2);
}

#[test]
fn rotation_z() {
    let p1 = primatives::point(0.0, 1.0, 0.0);

    let pi = transformations::PI;
    let half_quarter = transformations::new_rotation_z_matrix(pi / 4.0);
    let full_quarter = transformations::new_rotation_z_matrix(pi / 2.0);

    let root_2_over_2 = (2.0_f64).sqrt() / 2.0;
    let expected = primatives::point(-root_2_over_2, root_2_over_2, 0.0);
    let expected2 = primatives::point(-1.0, 0.0, 0.0);

    assert_eq!(expected, half_quarter * p1);
    assert_eq!(expected2, full_quarter * p1);

    assert_eq!(p1, half_quarter.inverse() * expected);
    assert_eq!(p1, full_quarter.inverse() * expected2);
}

#[test]
fn shearing() {
    let p1 = primatives::point(2.0, 3.0, 4.0);

    let transform = transformations::new_shearing_matrix(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    let expected = primatives::point(5.0, 3.0, 4.0);
    assert_eq!(transform * p1, expected);

    let transform = transformations::new_shearing_matrix(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
    let expected = primatives::point(6.0, 3.0, 4.0);
    assert_eq!(transform * p1, expected);

    let transform = transformations::new_shearing_matrix(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
    let expected = primatives::point(2.0, 5.0, 4.0);
    assert_eq!(transform * p1, expected);

    let transform = transformations::new_shearing_matrix(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
    let expected = primatives::point(2.0, 7.0, 4.0);
    assert_eq!(transform * p1, expected);

    let transform = transformations::new_shearing_matrix(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
    let expected = primatives::point(2.0, 3.0, 6.0);
    assert_eq!(transform * p1, expected);

    let transform = transformations::new_shearing_matrix(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
    let expected = primatives::point(2.0, 3.0, 7.0);
    assert_eq!(transform * p1, expected);
    assert_eq!(transform.inverse() * expected, p1);
}

#[ignore]
#[test]
fn clock_face() {
    let mut can = canvas::new(550, 550);
    let p1 = primatives::point(100.0, 100.0, 0.0);

    let pi = transformations::PI;
    let clock_rotation = transformations::new_rotation_z_matrix((2.0 * pi) / 12.0);

    let c1 = color::new(1.0, 0.0, 0.0);
    can.plot(p1.x as i32, p1.y as i32, c1);
        
    let mut p2 = clock_rotation * p1;
    while p2 != p1 {
        can.plot(p2.x as i32, p2.y as i32, c1);
        p2 = clock_rotation * p2;
    }

    can.write_to_ppm("clockface.ppm");
}





