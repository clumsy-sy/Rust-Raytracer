use rustRaytracer::{geometry::*, tools::*};

fn color(r: &Ray, world: &HittableList) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(&r, 0.0, std::f64::MAX, &mut rec) {
        return 0.5
            * Vec3::new(
                rec.normal().x() + 1.0,
                rec.normal().y() + 1.0,
                rec.normal().z() + 1.0,
            );
    } else {
        let unit_direction = Vec3::unit_vector(&r.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);

        Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
    }
}

fn write_png(w: u32, h: u32, _max_vlue: i32) {
    let mut buffer: Vec<u8> = Vec::new();

    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::sphere(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
    )));
    let world = HittableList::new(list);

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..h).rev() {
        for i in 0..w {
            let u: f64 = i as f64 / w as f64;
            let v: f64 = j as f64 / h as f64;
            let r: Ray = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);
            let p = r.point_at_parameter(2.0);
            let col = color(&r, &world);
            let b: f64 = 0.2;

            let ir: u8 = (255.99 * col.r()) as u8;
            let ig: u8 = (255.99 * col.g()) as u8;
            let ib: u8 = (255.99 * col.b()) as u8;
            buffer.push(ir);
            buffer.push(ig);
            buffer.push(ib);
        }
    }
    image::save_buffer("image.png", &buffer, w, h, image::ColorType::Rgb8).unwrap();
}

fn main() {
    let width: u32 = 200;
    let height: u32 = 100;
    let max_value: i32 = 255;

    write_png(width, height, max_value);
}
