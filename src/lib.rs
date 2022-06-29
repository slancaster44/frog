
pub mod primatives;
pub mod matrix;
pub mod color;
pub mod canvas;

#[cfg(test)]
mod primatives_tests {
    use crate::primatives;
    use crate::canvas;
    use crate::color;

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

        let result = primatives::dot_product(&vec1, &vec2);
        assert_eq!(result, 3.0);
    }

    #[test]
    fn cross_product() {
        let vec1 = primatives::vec3(1.0, 2.0, 3.0);
        let vec2 = primatives::vec3(2.0, 3.0, 4.0);

        let result_1_2 = primatives::cross_product(&vec1, &vec2);
        let expected_1_2 = primatives::vec3(-1.0, 2.0, -1.0);
        assert_eq!(result_1_2, expected_1_2);

        let result_2_1 = primatives::cross_product(&vec2, &vec1);
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
}

#[cfg(test)]
mod color_tests {
    use crate::color;

    #[test]
    fn test_color_creation() {
        let this_color = color::Color {
            red: -0.5,
            green: 0.3,
            blue: 1.7
        };

        assert_eq!(this_color.red, -0.5);
    }

    #[test]
    fn test_color_eq() {
        let c1 = color::new(0.0, 0.0, 0.0);
        let c2 = color::new(0.0, 0.0, 0.0);

        assert_eq!(c1, c2);
    }

    #[test]
    fn test_color_add() {
        let c1 = color::new(0.9, 0.5, 0.75);
        let c2 = color::new(0.7, 0.1, 0.25);

        let result = c1 + c2;
        let expected = color::new(1.6, 0.6, 1.0);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_color_sub() {
        let c1 = color::new(0.9, 0.5, 0.75);
        let c2 = color::new(0.7, 0.1, 0.25);

        let result = c1 - c2;
        let expected = color::new(0.2, 0.4, 0.5);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_color_scalar() {
        let scalar = 2.0;
        let c1 = color::new(0.2, 0.3, 0.4);

        let r1 = scalar * c1;
        let r2 = c1 * scalar;
        let expected = color::new(0.4, 0.6, 0.8);

        assert_eq!(r1, expected);
        assert_eq!(r2, expected);
    }

    #[test]
    fn test_color_mul() {
        let c1 = color::new(1.0, 0.2, 0.4);
        let c2 = color::new(0.9, 1.0, 0.1);
        
        let result = c2 * c1;
        let expected = color::new(0.9, 0.2, 0.04);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_color_to_255() {
        let c1 = color::new(1.5, -2.0, 0.5);
        
        let result = c1.scaled_from_1_to_255();
        let expected = color::new(255.0, 0.0, 127.0);

        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod canvas_tests {
    use crate::canvas;
    use crate::color;

    #[test]
    fn test_canvas_creation() {
        let canvas = canvas::new(1080, 960);
        assert_eq!(canvas.width, 1080);
        assert_eq!(canvas.height, 960);
        assert_eq!(canvas.contents[0], color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_canvas_plot_and_read() {
        let mut canvas = canvas::new(100, 100);

        let red = color::new(1.0, 0.0, 0.0);
        let green = color::new(0.0, 1.0, 0.0);
        let blue = color::new(0.0, 0.0, 1.0);
        let white = color::new(1.0, 1.0, 1.0);

        canvas.plot(-1, -1, green);
        let read_out = canvas.read(-1, -1);
        assert_eq!(green, read_out);

        canvas.plot(1, 1, red);
        canvas.plot(1, -1, blue);
        canvas.plot(-1, 1, white);

        //canvas.write_to_ppm("coord_plane_test.ppm");
    }

    #[test]
    #[should_panic]
    fn test_plot_out_of_bounds() {
        let mut canvas = canvas::new(100, 100);
        let c1 = color::new(1.0, 0.0, 0.0);
        canvas.plot(50, 50, c1);
    }
}

#[cfg(test)]
mod matrix_tests {
    use crate::matrix;
    use crate::primatives;


    #[test]
    fn matrix_creation() {
        let mut m1 = matrix::new4x4();
        m1.contents[1][1] = 10.0;

        assert_eq!(m1.contents[0][0], 0.0);
        assert_eq!(m1.contents[1][1], 10.0);
    }

    #[test]
    fn matrix_eq() {
        let m1 = matrix::new4x4();
        let m2 = matrix::new4x4();

        let mut m3 = matrix::new4x4();
        m3.contents[0][0] = 10.0;

        assert_ne!(m2, m3);
        assert_eq!(m1, m2);
    }

    #[test]
    fn index_4x4() {
        let mut m1 = matrix::new4x4();
        let mut m2 = matrix::new4x4();

        m1[0] = [1.0, 2.0, 3.0, 4.0];
        m2[0] = [1.0, 2.0, 3.0, 4.0];

        m1[1][0] = 10.0;
        m2[1][0] = 10.0;
        assert_eq!(m1, m2);
    }

    #[test]
    fn get_col_4x4() {
        let mut m1 = matrix::new4x4();
        m1[0] = [1.0, 2.0, 3.0, 4.0];
        m1[1] = [2.0, 3.0, 4.0, 5.0];
        m1[2] = [3.0, 4.0, 5.0, 6.0];
        m1[3] = [5.0, 5.0, 6.0, 7.0];

        let expected = [1.0, 2.0, 3.0, 5.0];
        let result = m1.get_col(0);

        assert_eq!(expected, result);
    }

    #[test]
    fn mul_4x4() {
        let mut m1 = matrix::new4x4();
        let mut m2 = matrix::new4x4();

        m1[0] = [1.0, 2.0, 3.0, 4.0];
        m1[1] = [5.0, 6.0, 7.0, 8.0];
        m1[2] = [9.0, 8.0, 7.0, 6.0];
        m1[3] = [5.0, 4.0, 3.0, 2.0];

        m2[0] = [-2.0, 1.0, 2.0, 3.0];
        m2[1] = [3.0, 2.0, 1.0, -1.0];
        m2[2] = [4.0, 3.0, 6.0, 5.0];
        m2[3] = [1.0, 2.0, 7.0, 8.0];

        let m3 = m1 * m2;
        let mut expected = matrix::new4x4();

        expected[0] = [20.0, 22.0, 50.0, 48.0];
        expected[1] = [44.0, 54.0, 114.0, 108.0];
        expected[2] = [40.0, 58.0, 110.0, 102.0];
        expected[3] = [16.0, 26.0, 46.0, 42.0];

        assert_eq!(m3, expected);
    }

    #[test]
    fn mul_4x4_and_tuple() {
        let v1 = primatives::point(1.0, 2.0, 3.0);
        let mut m1 = matrix::new4x4();

        m1[0] = [1.0, 2.0, 3.0, 4.0];
        m1[1] = [2.0, 4.0, 4.0, 2.0];
        m1[2] = [8.0, 6.0, 4.0, 1.0];
        m1[3] = [0.0, 0.0, 0.0, 1.0];

        let expected = primatives::point(18.0, 24.0, 33.0);
        let result = m1 * v1;

        assert_eq!(expected, result);
    }

    #[test]
    fn id_4x4() {
        let mut m1 = matrix::new4x4();
        m1[0] = [1.0, 2.0, 3.0, 4.0];
        m1[1] = [5.0, 6.0, 7.0, 8.0];
        m1[2] = [9.0, 8.0, 7.0, 6.0];
        m1[3] = [5.0, 4.0, 3.0, 2.0];

        let result = m1 * matrix::IDENTITY_MATRIX_4X4;

        assert_eq!(result, m1);
    }

    #[test]
    fn transpose_4x4() {
        let mut m1 = matrix::new4x4();
        m1[0] = [0.0, 9.0, 3.0, 0.0];
        m1[1] = [9.0, 8.0, 0.0, 8.0];
        m1[2] = [1.0, 8.0, 5.0, 3.0];
        m1[3] = [0.0, 0.0, 5.0, 8.0];

        let mut expected = matrix::new4x4();
        expected[0] = [0.0, 9.0, 1.0, 0.0];
        expected[1] = [9.0, 8.0, 8.0, 0.0];
        expected[2] = [3.0, 0.0, 5.0, 5.0];
        expected[3] = [0.0, 8.0, 3.0, 8.0];

        let result = m1.transposed();

        assert_eq!(result, expected)
    }

    #[test]
    fn determinant_2x2() {
        let mut m1 = matrix::new2x2();
        m1[0] = [1.0, 5.0];
        m1[1] = [-3.0, 2.0];

        let result = m1.determinant();
        assert_eq!(result, 17.0);
    }

    #[test]
    fn submatricies() {
        let mut m1 = matrix::new4x4();
        m1[0] = [0.0, 9.0, 3.0, 0.0];
        m1[1] = [9.0, 8.0, 0.0, 8.0];
        m1[2] = [1.0, 8.0, 5.0, 3.0];
        m1[3] = [0.0, 0.0, 5.0, 8.0];

        let mut expected = matrix::new3x3();
        expected[0] = [9.0, 8.0, 0.0];
        expected[1] = [1.0, 8.0, 5.0];
        expected[2] = [0.0, 0.0, 5.0];

        let r1 = m1.submatrix(0, 3);
        assert_eq!(r1, expected);

        let mut e2 = matrix::new2x2();
        e2[0] = [9.0, 0.0];
        e2[1] = [1.0, 5.0];

        let r2 = r1.submatrix(2, 1);
        assert_eq!(r2, e2);
    }

    #[test]
    fn minor_3x3() {
        let mut m1 = matrix::new3x3();
        m1[0] = [3.0, 5.0, 0.0];
        m1[1] = [2.0, -1.0, -7.0];
        m1[2] = [6.0, -1.0, 5.0];

        let expected = 25.0;
        let result = m1.minor(1, 0);

        assert_eq!(expected, result);
    }

    #[test]
    fn cofactor_3x3() {
        let mut m1 = matrix::new3x3();
        m1[0] = [3.0, 5.0, 0.0];
        m1[1] = [2.0, -1.0, -7.0];
        m1[2] = [6.0, -1.0, 5.0];

        let r1 = m1.cofactor(0, 0);
        let e1 = -12.0;
        assert_eq!(r1, e1);

        let r2 = m1.cofactor(1, 0);
        let e2 = -25.0;
        assert_eq!(r2, e2);
    }

    #[test]
    fn determinant_3x3() {
        let mut m1 = matrix::new3x3();
        m1[0] = [1.0, 2.0, 6.0];
        m1[1] = [-5.0, 8.0, -4.0];
        m1[2] = [2.0, 6.0, 4.0];

        let r1 = m1.cofactor(0, 0);
        let e1 = 56.0;
        assert_eq!(r1, e1);

        let r2 = m1.cofactor(0, 1);
        let e2 = 12.0;
        assert_eq!(r2, e2);

        let r3 = m1.cofactor(0, 2);
        let e3 = -46.0;
        assert_eq!(r3, e3);

        let r4 = m1.determinant();
        let e4 = -196.0;
        assert_eq!(r4, e4);
    }

    #[test]
    fn determinant_4x4() {
        let mut m1 = matrix::new4x4();
        m1[0] = [-2.0, -8.0, 3.0, 5.0];
        m1[1] = [-3.0, 1.0, 7.0, 3.0];
        m1[2] = [1.0, 2.0, -9.0, 6.0];
        m1[3] = [-6.0, 7.0, 7.0, -9.0];

        let r1 = m1.cofactor(0, 0);
        let e1 = 690.0;
        assert_eq!(r1, e1);

        let r2 = m1.cofactor(0, 1);
        let e2 = 447.0;
        assert_eq!(r2, e2);

        let r3 = m1.cofactor(0, 2);
        let e3 = 210.0;
        assert_eq!(r3, e3);

        let r4 = m1.cofactor(0, 3);
        let e4 = 51.0;
        assert_eq!(r4, e4);

        let r5 = m1.determinant();
        let e5 = -4071.0;
        assert_eq!(r5, e5);
    }

    #[test]
    fn matrix_inversion() {
        let mut m1 = matrix::new4x4();
        m1[0] = [-5.0, 2.0, 6.0, -8.0];
        m1[1] = [1.0, -5.0, 1.0, 8.0];
        m1[2] = [7.0, 7.0, -6.0, -7.0];
        m1[3] = [1.0, -3.0, 7.0, 4.0];

        let r1 = m1.matrix_of_cofactors();
        let mut e1 = matrix::new4x4();
        e1[0] = [116.0, -430.0, -42.0, -278.0];
        e1[1] = [240.0, -775.0, -119.0, -433.0];
        e1[2] = [128.0, -236.0, -28.0, -160.0];
        e1[3] = [-24.0, 277.0, 105.0, 163.0];
        assert_eq!(r1, e1);

        let expected = 105.0/532.0;
        let result = m1.inverse()[2][3];
        assert_eq!(result, expected);

        let mut m2 = matrix::new4x4();
        m2[0] = [8.0, -5.0, 9.0, 2.0];
        m2[1] = [7.0, 5.0, 6.0, 1.0];
        m2[2] = [-6.0, 0.0, 9.0, 6.0];
        m2[3] = [-3.0, 0.0, -9.0, -4.0];

        let mut e2 = matrix::new4x4();
        e2[0] = [-0.15385, -0.15385, -0.28205, -0.53846];
        e2[1] = [-0.07692, 0.12308, 0.02564, 0.03077];
        e2[2] = [0.35897, 0.35897, 0.43590, 0.92308];
        e2[3] = [-0.69231,-0.69231, -0.76923, -1.92308];

        let r2 = m2.inverse();
        assert_eq!(r2, e2);

        let m3 = m2 * m1;
        let should_be_same_as_m2 = m3 * m1.inverse();
        assert_eq!(m2, should_be_same_as_m2)
    }
}

#[cfg(test)]
mod matrix_tranformations {
    use crate::matrix::transformations;
    use crate::primatives;
    use crate::canvas;
    use crate::color;

    #[test]
    fn translation() {
        let translation = transformations::new_translation(5.0, -3.0, 2.0);
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
        let transform = transformations::new_scaling(2.0, 3.0, 4.0);

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
        let half_quarter = transformations::new_rotation_x(pi / 4.0);
        let full_quarter = transformations::new_rotation_x(pi / 2.0);
     
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
        let half_quarter = transformations::new_rotation_y(pi / 4.0);
        let full_quarter = transformations::new_rotation_y(pi / 2.0);

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
        let half_quarter = transformations::new_rotation_z(pi / 4.0);
        let full_quarter = transformations::new_rotation_z(pi / 2.0);

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

        let transform = transformations::new_shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let expected = primatives::point(5.0, 3.0, 4.0);
        assert_eq!(transform * p1, expected);

        let transform = transformations::new_shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let expected = primatives::point(6.0, 3.0, 4.0);
        assert_eq!(transform * p1, expected);

        let transform = transformations::new_shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let expected = primatives::point(2.0, 5.0, 4.0);
        assert_eq!(transform * p1, expected);

        let transform = transformations::new_shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let expected = primatives::point(2.0, 7.0, 4.0);
        assert_eq!(transform * p1, expected);

        let transform = transformations::new_shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let expected = primatives::point(2.0, 3.0, 6.0);
        assert_eq!(transform * p1, expected);

        let transform = transformations::new_shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
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
        let clock_rotation = transformations::new_rotation_z((2.0 * pi) / 12.0);

        let c1 = color::new(1.0, 0.0, 0.0);
        can.plot(p1.x as i32, p1.y as i32, c1);
         
        let mut p2 = clock_rotation * p1;
        while p2 != p1 {
            can.plot(p2.x as i32, p2.y as i32, c1);
            p2 = clock_rotation * p2;
        }

        can.write_to_ppm("clockface.ppm");
    }
}
