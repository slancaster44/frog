use crate::primatives;

pub struct Intersection<ShapeT>  {
    pub time: f64,
    pub location: primatives::PointT,
    pub shape: ShapeT
}

pub fn new<ShapeT>(t: f64, l: primatives::PointT, s: ShapeT) -> Intersection<ShapeT> {
    return Intersection {
        time: t,
        location: l,
        shape: s
    }
}