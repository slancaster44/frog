use crate::primatives;

pub struct Ray {
    pub origin: primatives::Tuple,
    pub direction: primatives::Tuple
}

pub fn new(origin: primatives::Tuple, direction: primatives::Tuple) -> Ray {
    origin.check_type(primatives::TYPE_PNT);
    direction.check_type(primatives::TYPE_VEC);

    return Ray {
        origin: origin,
        direction: direction,
    }
}

impl Ray {
    pub fn position(&self, time: f64) -> primatives::Tuple {
        //Enlage direction vectory by the time, and then add
        //that to the origin point to get the ray's current location
        return self.origin + (self.direction * time);
    }
}