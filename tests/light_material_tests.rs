#[cfg(test)]

use frog::color;
use frog::primatives;
use frog::light;
use frog::material;
use frog::shading;
use frog::shapes;
use frog::shapes::{Shape};
use frog::ray;
use frog::matrix::transformations;
use frog::canvas;

#[test]
fn light_creation() {
    let i = color::new(1.0, 1.0, 1.0);
    let l = primatives::point(0.0, 0.0, 0.0);

    let light = light::new(i, l);
    assert_eq!(light.intensity, i);
    assert_eq!(light.location, l);
}

#[test]
fn material_creation() {
    let m = material::new_default();

    assert_eq!(m.ambient, 0.1);
    assert_eq!(m.diffuse, 0.9);
    assert_eq!(m.specular, 0.9);
    assert_eq!(m.shininess, 200.0);
}

#[test]
fn lighting_test() {
    let location_to_be_lit = primatives::point(0.0, 0.0, 0.0);
    let material_for_item = material::new_default();

    //Test what happens when light is behind the eye.
    let location_of_eye = primatives::point(0.0, 0.0, -1.0);
    let vec_from_eye_to_location = location_of_eye - location_to_be_lit;
    assert_eq!(vec_from_eye_to_location, primatives::vec3(0.0, 0.0, -1.0));

    let normal_vec = primatives::vec3(0.0, 0.0, -1.0); //placeholder; in a real scene this will be calculted off a shape
    let light = light::new(color::new(1.0, 1.0, 1.0), primatives::point(0.0, 0.0, -10.0)); //Light is behind eye

    let result = shading::shade(material_for_item, light, location_to_be_lit, vec_from_eye_to_location, normal_vec);
    let expected = color::new(1.9, 1.9, 1.9);
    assert_eq!(expected, result);

    //When the eye is above the light source
    let eyev = primatives::vec3(0.0, 2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0);
    let normalv = primatives::vec3(0.0, 0.0, -1.0);
    let light = light::new(color::new(1.0, 1.0, 1.0), primatives::point(0.0, 0.0, -10.0));

    let result = shading::shade(material_for_item, light, location_to_be_lit, eyev, normalv);
    let expected = color::new(1.0, 1.0, 1.0);
    assert_eq!(result, expected);

    //When light is above eye
    let eyev = primatives::vec3(0.0, 0.0, -1.0);
    let normalv = primatives::vec3(0.0, 0.0, -1.0);
    let light = light::new(color::new(1.0, 1.0, 1.0), primatives::point(0.0, 10.0, -10.0));

    let result = shading::shade(material_for_item, light, location_to_be_lit, eyev, normalv);
    let expected = color::new(0.7364, 0.7364, 0.7364);
    assert_eq!(result, expected);

    //When specular component is at full strength
    let eyev = primatives::vec3(0.0, -2.0_f64.sqrt()/2.0, -2.0_f64.sqrt()/2.0);
    let normalv = primatives::vec3(0.0, 0.0, -1.0);
    let light = light::new(color::new(1.0, 1.0, 1.0), primatives::point(0.0, 10.0, -10.0));

    let result = shading::shade(material_for_item, light, location_to_be_lit, eyev, normalv);
    let expected = color::new(1.6364, 1.6364, 1.6364);
    assert_eq!(result, expected);

    //When light is behind the surface
    let eyev = primatives::vec3(0.0, 0.0, -1.0);
    let normalv = primatives::vec3(0.0, 0.0, -1.0);
    let light = light::new(color::new(1.0, 1.0, 1.0), primatives::point(0.0, 0.0, 10.0));

    let result = shading::shade(material_for_item, light, location_to_be_lit, eyev, normalv);
    let expected = color::new(0.1, 0.1, 0.1);
    assert_eq!(result, expected);

    
}

#[test]
#[ignore]
fn draw_sphere_shaded() {
    let mut s = shapes::sphere::new(300.0, primatives::point(350.0, 350.0, 0.0));
    s.material.color = color::new(1.0, 0.5, 1.0);

    let light = light::new(color::new(1.0, 1.0, 1.0), primatives::point(-200.0, -400.0, 800.0));
    
    //Because camera work has not been properly implemented, the specular/diffuse
    //Highlights are acting funny
    let mut vantage_point = primatives::point(400.0, 400.0, 400.0);
    let trans = transformations::new_rotation_z_matrix((2.0 * transformations::PI)/3.0);
    let trans2 = transformations::new_rotation_x_matrix((1.4 * transformations::PI) / 3.0);
    vantage_point = trans2 * trans * vantage_point;

    let mut this_canvas = canvas::new(700, 700);
    this_canvas.origin = (0, 0);

    for x_coord in 0..this_canvas.width {
        for y_coord in 0..this_canvas.height {
            let r = ray::new(primatives::point(x_coord as f64, y_coord as f64, 0.0), primatives::vec3(0.0, 0.0, 1.0));
            let intersections = s.intersect(r);

            if intersections.len() != 0 {
                let pnt = intersections[0].location;
                pnt.check_type(primatives::TYPE_PNT);

                let normalv = s.normal_at(pnt);
                let eyev = vantage_point - pnt;

                let c1 = shading::shade(s.material, light, pnt, eyev, normalv);
                this_canvas.plot(x_coord as i32, y_coord as i32, c1);
            }
        }
    }

    this_canvas = this_canvas.antialiased(4);
    this_canvas.write_to_ppm("sphere_shaded.ppm");
}