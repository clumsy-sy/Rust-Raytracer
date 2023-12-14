mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vec3;

use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use image::save_buffer;
use ray::Ray;
use sphere::Sphere;
use std::fs::File;
use std::io::Write;
use vec3::Vec3;

fn color(r: &Ray, world: &HittableList) -> Vec3 {
    let mut rec = HitRecord::default();
    if world.hit(&r, 0.0, std::f32::MAX, &mut rec) {
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

fn write_ppm(w: u32, h: u32, max_vlue: i32) {
    let mut file = File::create("output.ppm").expect("Unable to create file");

    let mut list: Vec<Box<dyn Hittable>> = Vec::new();
    list.push(Box::new(Sphere::sphere(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    list.push(Box::new(Sphere::sphere(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
    )));
    let world = HittableList::new(list);

    writeln!(file, "P3\n{} {}\n{}", w, h, max_vlue).expect("Unable to write to file");

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..h).rev() {
        for i in 0..w {
            let u: f32 = i as f32 / w as f32;
            let v: f32 = j as f32 / h as f32;
            let r: Ray = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);
            let p = r.point_at_parameter(2.0);
            let col = color(&r, &world);
            let b: f32 = 0.2;

            let ir: i32 = (255.99 * col.r()) as i32;
            let ig: i32 = (255.99 * col.g()) as i32;
            let ib: i32 = (255.99 * col.b()) as i32;

            writeln!(file, "{} {} {}", ir, ig, ib).expect("Unable to write to file");
        }
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
            let u: f32 = i as f32 / w as f32;
            let v: f32 = j as f32 / h as f32;
            let r: Ray = Ray::ray(origin, lower_left_corner + horizontal * u + vertical * v);
            let p = r.point_at_parameter(2.0);
            let col = color(&r, &world);
            let b: f32 = 0.2;

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

    write_ppm(width, height, max_value);
    write_png(width, height, max_value);
}
