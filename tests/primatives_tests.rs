#[cfg(test)]

use frog::primatives;
use frog::canvas;
use frog::color;

#[test]
fn test_vector_construction() {
    let ret_val = primatives::vec3(1.0, 2.0, 3.0);
    let expect = primatives::Tuple {x:1.0, y:2.0, z:3.0, w:0.0};
    
    assert_eq!(expect.x, ret_val.x);
    assert_eq!(expect.y, ret_val.y);
    assert_eq!(expect.z, ret_val.z);
    assert_eq!(expect.w, ret_val.w);
}

#[test]
fn test_point_construction() {
    let ret_val = primatives::point(1.0, 2.0, 3.0);
    let expect = primatives::Tuple {x:1.0, y:2.0, z:3.0, w:primatives::TYPE_PNT};
    
    assert_eq!(expect.x, ret_val.x);
    assert_eq!(expect.y, ret_val.y);
    assert_eq!(expect.z, ret_val.z);
    assert_eq!(expect.w, ret_val.w);
}

#[test]
#[should_panic]
fn test_invalid_type() {
    let val = primatives::Tuple {x:1.0, y:2.0, z:3.0, w:2.0};
    val.check_type_validity();
}

#[test]
#[should_panic]
fn test_invalid_pnt() {
    let vec = primatives::vec3(0.0, 0.0, 0.0);
    vec.check_type(primatives::TYPE_PNT);
}

#[test]
#[should_panic]
fn test_invalid_vec3() {
    let pnt = primatives::point(0.0, 0.0, 0.0);
    pnt.check_type(primatives::TYPE_VEC);
}

#[test]
fn add_two_points() {
    let pnt1 = primatives::point(1.0, 2.0, 3.0);
    let pnt2 = primatives::point(1.0, 2.0, 3.0);

    let result = pnt1 + pnt2;

    //Adding two points results in an invalid type indicator
    assert_ne!(result.w, primatives::TYPE_PNT); 
    assert_ne!(result.w, primatives::TYPE_VEC);
}

#[test]
fn add_two_vecs() {
    let vec1 = primatives::vec3(1.0, 2.0, 3.0);
    let vec2 = primatives::vec3(1.0, 2.0, 3.0);

    let result = vec1 + vec2;

    //Adding two vectors results in vector
    assert_eq!(result.w, primatives::TYPE_VEC);
}

#[test]
fn add_vec_and_points() {
    let vec1 = primatives::vec3(1.0, 2.0, 3.0);
    let pnt1 = primatives::point(1.0, 2.0, 3.0);

    let result = pnt1 + vec1;

    //Adding a vector to a point results in a point
    assert_eq!(result.w, primatives::TYPE_PNT);
}

#[test]
fn sub_two_points() {
    let pnt1 = primatives::point(1.0, 2.0, 3.0);
    let pnt2 = primatives::point(1.0, 2.0, 3.0);

    let result = pnt1 - pnt2;

    //Adding two points results in a vector that, when applied to pnt1, would result in pnt2
    assert_eq!(result.w, primatives::TYPE_VEC);
}

#[test]
fn sub_two_vecs() {
    let vec1 = primatives::vec3(1.0, 2.0, 3.0);
    let vec2 = primatives::vec3(1.0, 2.0, 3.0);

    let result = vec1 - vec2;

    assert_eq!(result.w, primatives::TYPE_VEC);
}

#[test]
fn sub_vec_from_pnt() {
    let vec1 = primatives::vec3(1.0, 2.0, 3.0);
    let pnt1 = primatives::point(1.0, 2.0, 3.0);

    let result = pnt1 - vec1;

    //Applies a vector to a point
    assert_eq!(result.w, primatives::TYPE_PNT);
}

#[test]
fn sub_pnt_from_vec() {
    let vec1 = primatives::vec3(1.0, 2.0, 3.0);
    let pnt1 = primatives::point(1.0, 2.0, 3.0);

    let result = vec1 - pnt1;

    //Subtracting a point from a vector makes no sense
    assert_ne!(result.w, primatives::TYPE_PNT);
    assert_ne!(result.w, primatives::TYPE_VEC);
}

#[test]
fn neg_vec() {
    let vec1 = primatives::vec3(1.0, 2.0, 3.0);
    let result = -vec1;

    assert_eq!(result.w, primatives::TYPE_VEC);
}

#[test]
fn neg_pnt() {
    let pnt1 = primatives::point(1.0, 2.0, 3.0);

    let result = -pnt1;

    //Subtracting a point from a vector makes no sense
    assert_ne!(result.w, primatives::TYPE_PNT);
    assert_ne!(result.w, primatives::TYPE_VEC);
}

#[test]
//reverse order of above
fn scale_vec() {
    let scalar:f64 = 3.5;
    let vec = primatives::vec3(1.0, -2.0, 3.0);

    let result = scalar * vec;
    assert_eq!(result.w, primatives::TYPE_VEC);

    let result_1 = vec * scalar;
    assert_eq!(result_1.w, primatives::TYPE_VEC);
}

#[test]
fn dot_product() {
    let vec1 = primatives::vec3(1.0, 3.0, -5.0);
    let vec2 = primatives::vec3(4.0, -2.0, -1.0);

    let result = primatives::dot_product(vec1, vec2);
    assert_eq!(result, 3.0);
}

#[test]
fn cross_product() {
    let vec1 = primatives::vec3(1.0, 2.0, 3.0);
    let vec2 = primatives::vec3(2.0, 3.0, 4.0);

    let result_1_2 = primatives::cross_product(vec1, vec2);
    let expected_1_2 = primatives::vec3(-1.0, 2.0, -1.0);
    assert_eq!(result_1_2, expected_1_2);

    let result_2_1 = primatives::cross_product(vec2, vec1);
    let expected_2_1 = primatives::vec3(1.0, -2.0, 1.0);
    assert_eq!(result_2_1, expected_2_1);
}

#[test]
fn test_magnitude() {
    let vec1 = primatives::vec3(1.0, 2.0, 3.0);

    let result = vec1.magnitude();
    assert_eq!(14_f64.sqrt(), result);
}

#[test]
fn test_normalization() {
    let vec1 = primatives::vec3(3.0, 1.0, 2.0);

    let vec1_normalized = vec1.normalized();
    let expected = primatives::vec3(0.80178, 0.26726, 0.53452);
    assert_eq!(expected, vec1_normalized);
}

#[test]
fn tuple_eq() {
    let vec1 = primatives::vec3(1.0, 2.0, 3.0);
    let vec2 = primatives::vec3(1.0, 2.0, 3.0);
    let vec3 = primatives::vec3(3.0, 2.0, 3.0);

    assert_eq!(vec1, vec2);
    assert_ne!(vec1, vec3);
}

#[test]
fn reflection() {
    let v1 = primatives::vec3(1.0, -1.0, 0.0);
    let normal = primatives::vec3(0.0, 1.0, 0.0);
    let r = normal.reflect(v1);

    assert_eq!(r, primatives::vec3(1.0, 1.0, 0.0));

    let v2 = primatives::vec3(0.0, -1.0, 0.0);
    let normal = primatives::vec3(2.0_f64.sqrt()/2.0, 2.0_f64.sqrt()/2.0, 0.0);
    let r = normal.reflect(v2);

    assert_eq!(r, primatives::vec3(1.0, 0.0, 0.0));
}

#[ignore]
#[test]
fn projectile_test() {
    #[derive(Copy, Clone)]
    struct Env {
        wind_speed: primatives::Vec3T,
        gravity: primatives::Vec3T
    }

    struct Projectile {
        location: primatives::PointT,
        velocity: primatives::Vec3T
    }
    
    fn tick(env: Env, proj: Projectile) -> Projectile {
        return Projectile {
            location: proj.location + proj.velocity,
            velocity: proj.velocity + env.gravity + env.wind_speed
        }
    }

    let this_env = Env {
        wind_speed: primatives::vec3(-0.01, 0.0, 0.0),
        gravity: primatives::vec3(0.0, -0.1, 0.0)
    };

    let mut this_proj = Projectile {
        location: primatives::point(0.0, 1.0, 0.0),
        velocity: primatives::vec3(1.0, 1.8, 0.0).normalized() * 11.25
    };

    let mut this_canvas = canvas::new(900, 550);
    this_canvas.origin = (0, 0);
    let this_color = color::new(0.0, 1.0, 0.0);

    this_proj = tick(this_env, this_proj);
    while this_proj.location.y > 0.0 {
        this_canvas.plot(this_proj.location.x as i32, this_proj.location.y as i32, this_color);
        this_proj = tick(this_env, this_proj);
    }

    this_canvas.write_to_ppm("out.ppm");
}


