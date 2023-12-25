use crate::tools::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    orig: Vec3,
    dir: Vec3,
}

impl Ray {
    pub fn ray(a: Vec3, b: Vec3) -> Ray {
        Ray { orig: a, dir: b }
    }

    pub fn origin(self) -> Vec3 {
        self.orig
    }

    pub fn direction(self) -> Vec3 {
        self.dir
    }

    pub fn point_at_parameter(self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}
