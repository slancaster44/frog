use std::ops;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64
}

pub const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0
};

pub fn new(r:f64, g:f64, b:f64) -> Color {
    return Color {
        red:r,
        green:g,
        blue:b
    };
}

impl Color {
    pub fn scaled_from_1_to_255(&self)  -> Color {
        return Color {
            red: scale_num_1_to_255(self.red),
            green: scale_num_1_to_255(self.green),
            blue: scale_num_1_to_255(self.blue)
        };
    }
}

fn scale_num_1_to_255(val:f64) -> f64 {
    return if val > 1.0 {
        255.0
    } else if val < 0.0 {
        0.0
    } else {
        (val * 255.0).floor()
    };
}

pub const EQUIVALENCY_EPSILON:f64 = 0.0001;
impl PartialEq for Color {
    fn eq(&self, c1:&Color) -> bool {
        return (self.red - c1.red).abs() < EQUIVALENCY_EPSILON && 
        (self.green - c1.green).abs() < EQUIVALENCY_EPSILON &&
        (self.blue - c1.blue).abs() < EQUIVALENCY_EPSILON;
    }
}

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, c1:Color) -> Color {
        return Color {
            red: self.red + c1.red,
            green: self.green + c1.green,
            blue: self.blue + c1.blue
        }
    }
}

impl ops::Sub<Color> for Color {
    type Output = Color;

    fn sub(self, c1:Color) -> Color {
        return Color {
            red: self.red - c1.red,
            green: self.green - c1.green,
            blue: self.blue - c1.blue
        }
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, scalar:f64) -> Color {
        return Color {
            red: scalar * self.red,
            green: scalar * self.green,
            blue: scalar * self.blue
        }
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, c1:Color) -> Color {
        return Color {
            red: c1.red * self,
            green: c1.green * self,
            blue: c1.blue * self
        }
    }
}

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, c1:Color) -> Color {
        return Color {
            red: c1.red * self.red,
            green: c1.green * self.green,
            blue: c1.blue * self.blue
        }
    }
}