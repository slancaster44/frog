#[cfg(test)]

use frog::matrix;
use frog::primatives;


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