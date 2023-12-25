use vec3::Vec3;

use crate::tools::vec3;

use super::Ray;

#[derive(Default)]
pub struct HitRecord {
    t: f64,
    p: Vec3,
    normal: Vec3,
}

impl HitRecord {
    pub fn p(&self) -> Vec3 {
        self.p
    }
    pub fn t(&self) -> f64 {
        self.t
    }
    pub fn normal(&self) -> Vec3 {
        self.normal
    }
    pub fn set_p(&mut self, val: Vec3) {
        self.p = val
    }
    pub fn set_t(&mut self, val: f64) {
        self.t = val
    }
    pub fn set_normal(&mut self, val: Vec3) {
        self.normal = val
    }
}

pub trait Hittable {
    fn hit(&self, _r: &Ray, _t_min: f64, _t_max: f64, rec: &mut HitRecord) -> bool {
        false
    }
}
