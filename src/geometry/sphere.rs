use crate::geometry::hittable::*;
use crate::geometry::ray::Ray;
use crate::tools::vec3::Vec3;

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn sphere(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = Vec3::dot(&r.direction(), &r.direction());
        let b = Vec3::dot(&oc, &r.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;

        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.set_t(temp);
                rec.set_p(r.point_at_parameter(rec.t()));
                rec.set_normal((rec.p() - self.center) / self.radius);
                return true;
            }
            temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                rec.set_t(temp);
                rec.set_p(r.point_at_parameter(rec.t()));
                rec.set_normal((rec.p() - self.center) / self.radius);
                return true;
            }
        }
        return false;
    }
}