use crate::{ray::Ray, shapes::ShapeIntersection, util::vecmath::Bounds3};

pub struct Primitive {

}

impl Primitive {
    pub fn bounds(&self) -> Bounds3 {
        todo!()
    }

    pub fn intersect(&self, ray: &Ray, tmax: f64) -> Option<ShapeIntersection> {
        todo!()
    }

    pub fn intersect_p(&self, ray: &Ray, tmax: f64) -> bool {
        todo!()
    }
}