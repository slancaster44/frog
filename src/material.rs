use crate::color;

#[derive(Debug, Copy, Clone)]
pub struct Material {
    pub color: color::Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64
}

pub fn new(c: color::Color, a: f64, d: f64, sp: f64, sh: f64) -> Material {
    return Material {
        color: c,
        ambient: a,
        diffuse: d,
        specular: sp,
        shininess: sh
    }
}

pub fn new_default() -> Material {
    return Material {
        color: color::new(1.0, 1.0, 1.0),
        ambient: 0.1,
        diffuse: 0.9,
        specular: 0.9,
        shininess:  200.0
    }
}

pub const EQUIVALENCY_EPSILON: f64 = 0.0001;
impl PartialEq for Material {
    fn eq(&self, m: &Material) -> bool {
        return (self.ambient - m.ambient).abs() < EQUIVALENCY_EPSILON &&
            (self.diffuse - m.diffuse).abs() < EQUIVALENCY_EPSILON &&
            (self.specular - m.specular).abs() < EQUIVALENCY_EPSILON &&
            (self.shininess - m.shininess).abs() < EQUIVALENCY_EPSILON
    }
}