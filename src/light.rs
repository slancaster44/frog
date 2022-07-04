use crate::color;
use crate::primatives;

#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub intensity: color::Color,
    pub location: primatives::PointT
}

pub fn new(i: color::Color, l: primatives::PointT) -> Light {
    l.check_type(primatives::TYPE_PNT);

    return Light {
        intensity: i,
        location: l,
    }
}

impl PartialEq for Light {
    fn eq(&self, l:&Light) -> bool {
        return self.intensity == l.intensity && self.location == l.location
    }
}