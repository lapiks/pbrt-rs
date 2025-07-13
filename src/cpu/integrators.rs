use crate::{base::{camera::Camera, light::Light, sampler::Sampler}, cpu::primitive::Primitive, ray::Ray, shapes::ShapeIntersection};

pub trait Integrator {
    fn render(&self);
}

pub struct BaseIntegrator {
    aggregate: Primitive,
    lights: Vec<Light>,
    infinite_lights: Vec<Light>,
}

impl BaseIntegrator {
    pub fn new(aggregate: Primitive, lights: Vec<Light>) -> Self {
        let scene_bounds = aggregate.bounds();
        for light in &lights {
            light.preprocess(&scene_bounds);
            // if light.type = infinite
            //   push
        }
        let infinite_lights = Vec::default();
        Self { aggregate, lights, infinite_lights }
    }

    fn intersect(&self, ray: &Ray, tmax: f64) -> Option<ShapeIntersection> {
        self.aggregate.intersect(ray, tmax)
    }

    fn intersect_p(&self, ray: &Ray, tmax: f64) -> bool {
        self.aggregate.intersect_p(ray, tmax)
    }
}

pub struct ImageTileIntegrator {
    base: BaseIntegrator,
    camera: Camera,
    campler_prototype: Sampler,
}

impl Integrator for ImageTileIntegrator {
    fn render(&self) {

    }
}

pub struct RayIntegrator {
    base: ImageTileIntegrator,
}