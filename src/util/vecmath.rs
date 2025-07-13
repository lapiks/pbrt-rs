use glam::DVec3;

pub struct Bounds3 {
    pub p_min: DVec3,
    pub p_max: DVec3,
}

impl Default for Bounds3 {
    fn default() -> Self {
        Self { p_min: DVec3::MIN, p_max: DVec3::MAX }
    }
}

impl Bounds3 {
    pub fn new(p_min: DVec3, p_max: DVec3) -> Self {
        Self { p_min, p_max }
    }

    pub fn from_points(p1: DVec3, p2: DVec3) -> Self {
        Self {
            p_min: p1.min(p2),
            p_max: p1.max(p2),
        }
    }

    pub fn diagonal(&self) -> DVec3 {
        self.p_max - self.p_min
    }

    pub fn surface_area(&self) -> f64 {
        let d = self.diagonal();
        2.0 * (d.x * d.y + d.x * d.z + d.y * d.z)
    }

    pub fn volume(&self) -> f64 {
        let d = self.diagonal();
        d.x * d.y * d.z
    }

    pub fn empty(&self) -> bool {
        self.p_min.x >= self.p_max.x || 
        self.p_min.y >= self.p_max.y || 
        self.p_min.z >= self.p_max.z
    }
}