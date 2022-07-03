pub mod transformations;
use std::ops;
use crate::primatives;

pub const EQUIVALENCY_EPSILON:f64 = 0.0001;

#[derive(Debug, Clone, Copy)]
pub struct Matrix4x4 {
    pub contents: [[f64; 4]; 4]
}

pub const IDENTITY_MATRIX_4X4: Matrix4x4 = Matrix4x4 {
    contents: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]
};

pub fn new4x4() -> Matrix4x4 {
    return Matrix4x4 {
        contents: [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0]
        ]
    }
}

impl Matrix4x4 {
    pub fn get_col(&self, iter: usize) -> [f64; 4] {
        return [self.contents[0][iter],
             self.contents[1][iter], 
             self.contents[2][iter],
             self.contents[3][iter]]
    }

    pub fn transposed(&self) -> Matrix4x4 {
        return Matrix4x4 {
            contents: [self.get_col(0), self.get_col(1), self.get_col(2), self.get_col(3)]
        }
    }

    pub fn submatrix(&self, r: usize, c: usize) -> Matrix3x3 {
        if r >= 4 || c >= 4 {
            panic!("Invalid Index");
        }

        let mut ret_val = new3x3();
        let mut row_bias = 0;
        let mut column_bias = 0;

        for row_iter in 0..4 {
            if row_iter != r {
                for col_iter in 0..4 {
                    if col_iter != c {
                        ret_val[row_iter-row_bias][col_iter-column_bias] = self[row_iter][col_iter]
                    } else {
                        column_bias = 1;
                    }
                }
                column_bias = 0;
            } else {
                row_bias = 1;
            }
        }

        return ret_val;
    }

    pub fn minor(&self, r:usize, c:usize) -> f64 {
        return self.submatrix(r, c).determinant()
    }

    pub fn cofactor(&self, r:usize, c:usize) -> f64 {
        let s = self.minor(r, c);
        return
            if ((r % 2 == 0) && (c % 2 == 0)) || ((r % 2 != 0) && (c % 2 != 0)) {
                s
            } else {
                -s
            }
    }

    pub fn determinant(&self) -> f64 {
        return (self[0][0] * self.cofactor(0, 0)) + 
            (self[0][1] * self.cofactor(0, 1)) + 
            (self[0][2] * self.cofactor(0, 2)) + 
            (self[0][3] * self.cofactor(0, 3))
    }

    pub fn is_invertable(&self) -> bool {
        return self.determinant() != 0.0
    }

    pub fn matrix_of_cofactors(&self) -> Matrix4x4 {
        let mut ret_val = new4x4();

        for row_iter in 0..4 {
            for col_iter in 0..4 {
                ret_val[row_iter][col_iter] = self.cofactor(row_iter, col_iter)
            }
        }

        return ret_val;
    }

    pub fn inverse(&self) -> Matrix4x4 {
        let d = self.determinant();
        if d == 0.0 {
            panic!("Matrix is not invertable");
        }

        let mut ret_val = self.matrix_of_cofactors();
        ret_val = ret_val.transposed();

        for row_iter in 0..4 {
            for col_iter in 0..4 {
                ret_val[row_iter][col_iter] = ret_val[row_iter][col_iter] / d;
            }
        }
 
        return ret_val
    }
}

impl ops::Index<usize> for Matrix4x4 {
    type Output = [f64; 4];
    fn index(&self, index: usize) -> &[f64; 4] {
        return &self.contents[index];
    }
}

impl ops::IndexMut<usize> for Matrix4x4 {
    fn index_mut(&mut self, index: usize) -> &mut [f64; 4] {
        &mut self.contents[index]
    }
}

impl ops::Mul<Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;
    fn mul(self, m1:Matrix4x4) -> Matrix4x4 {
        let mut ret_val : Matrix4x4 = new4x4();

        for row_iter in 0..4 {
            let cur_row = self.contents[row_iter];

            for column_iter in 0..4 {
                let cur_column = m1.get_col(column_iter);

                ret_val[row_iter][column_iter] = (cur_row[0] * cur_column[0]) + 
                    (cur_row[1] * cur_column[1]) +
                    (cur_row[2] * cur_column[2]) +
                    (cur_row[3] * cur_column[3]) 
            }
        }

        return ret_val;
    }
}


impl ops::Mul<primatives::Tuple> for Matrix4x4 {
    type Output = primatives::Tuple;
    fn mul(self, t1:primatives::Tuple) -> primatives::Tuple {
        return primatives::Tuple {
            x: (self[0][0] * t1.x) + (self[0][1] * t1.y) + (self[0][2] * t1.z) + (self[0][3] * t1.w),
            y: (self[1][0] * t1.x) + (self[1][1] * t1.y) + (self[1][2] * t1.z) + (self[1][3] * t1.w),
            z: (self[2][0] * t1.x) + (self[2][1] * t1.y) + (self[2][2] * t1.z) + (self[2][3] * t1.w),
            w: (self[3][0] * t1.x) + (self[3][1] * t1.y) + (self[3][2] * t1.z) + (self[3][3] * t1.w)
        };
    }
}

//Rust is dumb. THE ONLY DIFFERENCE IS THAT THIS IS A REFERNCE TO A MATRIX. *anger*
//Thank the lord for copy-paste. And if any future genius can figure out how to get
//rid of this code duplication. Let me know how they did it. lancasterharp@gmail.com
impl ops::Mul<primatives::Tuple> for &Matrix4x4 {
    type Output = primatives::Tuple;
    fn mul(self, t1:primatives::Tuple) -> primatives::Tuple {
        return primatives::Tuple {
            x: (self[0][0] * t1.x) + (self[0][1] * t1.y) + (self[0][2] * t1.z) + (self[0][3] * t1.w),
            y: (self[1][0] * t1.x) + (self[1][1] * t1.y) + (self[1][2] * t1.z) + (self[1][3] * t1.w),
            z: (self[2][0] * t1.x) + (self[2][1] * t1.y) + (self[2][2] * t1.z) + (self[2][3] * t1.w),
            w: (self[3][0] * t1.x) + (self[3][1] * t1.y) + (self[3][2] * t1.z) + (self[3][3] * t1.w)
        };
    }

}

impl PartialEq for Matrix4x4 {
    fn eq(&self, m1:&Matrix4x4) -> bool {
        for row_iter in 0..4 {
            for col_iter in 0..4 {
                if (self[row_iter][col_iter] - m1[row_iter][col_iter]).abs() > EQUIVALENCY_EPSILON {
                    return false;
                }
            }
        }

        return true;
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
pub struct Matrix3x3 {
    pub contents: [[f64; 3]; 3]
}

pub fn new3x3() -> Matrix3x3 {
    return Matrix3x3 {
        contents: [
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0]
        ]
    }
}

pub const IDENTITY_MATRIX_3X3: Matrix3x3 = Matrix3x3 {
    contents: [
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0]
    ]
};

impl Matrix3x3 {
    pub fn submatrix(&self, r:usize, c:usize) -> Matrix2x2 {
        if r >= 3 || c >= 3 {
            panic!("Invalid Index");
        }

        let mut ret_val = new2x2();
        let mut row_bias = 0;
        let mut column_bias = 0;

        for row_iter in 0..3 {
            if row_iter != r {
                for col_iter in 0..3 {
                    if col_iter != c {
                        ret_val[row_iter-row_bias][col_iter-column_bias] = self[row_iter][col_iter]
                    } else {
                        column_bias = 1;
                    }
                }
                column_bias = 0;
            } else {
                row_bias = 1;
            }
        }

        return ret_val;
    }

    pub fn minor(&self, r:usize, c:usize) -> f64 {
        let s = self.submatrix(r, c);
        return s.determinant();
    }

    pub fn cofactor(&self, r:usize, c:usize) -> f64 {
        let m = self.minor(r, c);

        return
            if (r * 3) + c % 2 == 0 {
                m
            } else {
                -m
            }
    }

    pub fn determinant(&self) -> f64 {
        return (self[0][0] * self.cofactor(0, 0)) + (self[0][1] * self.cofactor(0, 1)) + (self[0][2] * self.cofactor(0, 2));
    }

    pub fn is_invertable(&self) -> bool {
        return self.determinant() != 0.0;
    }
}

impl ops::Index<usize> for Matrix3x3 {
    type Output = [f64; 3];
    fn index(&self, index: usize) -> &[f64; 3] {
        return &self.contents[index];
    }
}

impl ops::IndexMut<usize> for Matrix3x3 {
    fn index_mut(&mut self, index: usize) -> &mut [f64; 3] {
        &mut self.contents[index]
    }
}

impl PartialEq for Matrix3x3 {
    fn eq(&self, m1:&Matrix3x3) -> bool {
        for row_iter in 0..3 {
            for col_iter in 0..3 {
                if (self[row_iter][col_iter] - m1[row_iter][col_iter]).abs() > EQUIVALENCY_EPSILON {
                    return false
                }
            }
        }

        return true;
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Copy, Clone)]
pub struct Matrix2x2 {
    pub contents: [[f64; 2]; 2]
}

pub const IDENTITY_MATRIX_2X2: Matrix2x2 = Matrix2x2 {
    contents: [
        [1.0, 0.0],
        [0.0, 1.0]
    ]
};

pub fn new2x2() -> Matrix2x2 {
    return Matrix2x2 {
        contents: [[0.0, 0.0], [0.0, 0.0]]
    }
}

impl Matrix2x2 {
    pub fn determinant(&self) -> f64 {
        return (self[1][1] * self[0][0]) - (self[1][0] * self[0][1])
    }

    pub fn is_invertable(&self) -> bool {
        return self.determinant() != 0.0;
    }
}

impl ops::Index<usize> for Matrix2x2 {
    type Output = [f64; 2];
    fn index(&self, index: usize) -> &[f64; 2] {
        return &self.contents[index];
    }
}

impl ops::IndexMut<usize> for Matrix2x2 {
    fn index_mut(&mut self, index: usize) -> &mut [f64; 2] {
        &mut self.contents[index]
    }
}

impl PartialEq for Matrix2x2 {
    fn eq(&self, m1:&Matrix2x2) -> bool {
        for row_iter in 0..2 {
            for col_iter in 0..2 {
                if (self[row_iter][col_iter] - m1[row_iter][col_iter]).abs() > EQUIVALENCY_EPSILON {
                    return false;
                }
            }
        }

        return true;
    }
}