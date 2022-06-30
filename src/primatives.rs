use std::ops;


#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,

    /* W indicates whether the tuple is a vector or if it is a point.
     * Because of the w values chosen for vectors (0.0), and points (1.0), this w
     * value will become an invalid value (any value other than 0.0 or 1.0) when
     * an invalid operation occurs (eg, the addition of two points).
     */
    pub w: f64
}

//Alias types for Tuple
pub type PointT = Tuple;
pub type Vec3T = Tuple;

pub const TYPE_VEC:f64 = 0.0;
pub const TYPE_PNT:f64 = 1.0;

//Margin of error used when testing if tuples are equal
pub const EQUIVALENCY_EPSILON:f64 = 0.0001;

pub fn vec3(x: f64, y:f64, z:f64) -> Tuple {
    return Tuple {x:x, y:y, z:z, w:TYPE_VEC};
}

pub fn point(x:f64, y:f64, z:f64) -> Tuple  {
    return Tuple {x:x, y:y, z:z, w:TYPE_PNT};
}

impl Tuple {

    pub fn check_type_validity(&self) {
        if self.w != TYPE_VEC && self.w != TYPE_PNT {
            panic!("This tuple has an invalid type");
        }
    }

    pub fn check_type(&self, t:f64) {
        if self.w != t {
            panic!("This tuple did not have the expected type");
        }
    }

    pub fn magnitude(&self) -> f64 {
        //When the tuple is a vector, w = 0. So it has no effect on the calculation of magnitude
        //It is included here for consistancy, as w as included in other calculations (addition, subtraction, etc)
        let ret_val_squared = (self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w);
        return ret_val_squared.sqrt();
    }

    pub fn normalized(&self) -> Tuple {
        let mag = self.magnitude();
        return Tuple { 
            x:self.x / mag,
            y:self.y / mag,
            z:self.z / mag,
            w:self.w / mag
        };
    }
}

impl ops::Add<Tuple> for Tuple {
    type Output = Tuple;

    fn add(self, t2:Tuple) -> Tuple {
        return Tuple {
            x: self.x + t2.x,
            y: self.y + t2.y,
            z: self.z + t2.z,
            w: self.w + t2.w
        }
    }
}

impl ops::Sub<Tuple> for Tuple {
    type Output = Tuple;

    fn sub(self, t2:Tuple) -> Tuple {
        return Tuple {
            x: self.x - t2.x,
            y: self.y - t2.y,
            z: self.z - t2.z,
            w: self.w - t2.w
        }
    }
}

pub fn dot_product(t1:Tuple, t2:Tuple) -> f64 {
    return (t1.x * t2.x) + (t1.y * t2.y) + (t1.z * t2.z) + (t1.w * t2.w);
}

pub fn cross_product(t1:Tuple, t2:Tuple) -> Tuple {
    return vec3(t1.y * t2.z - t1.z * t2.y,
        t1.z * t2.x - t1.x * t2.z,
        t1.x * t2.y - t1.y * t2.x);
}

impl ops::Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, scalar:f64) -> Tuple {
        return Tuple {
            x:scalar * self.x,
            y:scalar * self.y,
            z:scalar * self.z,
            w:scalar * self.w
        }
    }
}

impl ops::Mul<Tuple> for f64 {
    type Output = Tuple;

    fn mul(self, t1:Tuple) -> Tuple {
        return Tuple {
            x:self * t1.x,
            y:self * t1.y,
            z:self * t1.z,
            w:self * t1.w
        }
    }
}

impl ops::Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        return Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, t1:&Tuple) -> bool {
        return (self.x - t1.x).abs() < EQUIVALENCY_EPSILON && 
            (self.y - t1.y).abs() < EQUIVALENCY_EPSILON &&
            (self.z - t1.z).abs() < EQUIVALENCY_EPSILON &&
            (self.w - t1.w).abs() < EQUIVALENCY_EPSILON;
    }
}