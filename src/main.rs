extern crate rand;

use std::f64;

mod camera;
mod image;
mod ray;
mod vec;

use vec::Vec3;
use ray::Ray;
use image::PPM;
use camera::Camera;

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
}

pub trait Model {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Model for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin.sub_vec3(&self.center);
        let a: f64 = ray.direction.dot(&ray.direction);
        let b: f64 = 2.0 * oc.dot(&ray.direction);
        let c: f64 = oc.dot(&oc) - self.radius * self.radius;
        let discriminant: f64 = b * b - 4.0 * a * c;

        if discriminant > 0.0 {
            let temp = (-b - (b*b - a*c).sqrt()) / a;

            if temp < t_max && temp > t_min {
                let point = ray.point_at_parameter(temp);

                return Some(
                    HitRecord {
                        t: temp,
                        point: point.clone(),
                        normal: point.sub_vec3(&self.center).div_t(self.radius)
                    }
                );
            }

            let temp = (-b + (b*b - a *c).sqrt()) / a;

            if temp < t_max && temp > t_min {
                let point = ray.point_at_parameter(temp);

                return Some(
                    HitRecord {
                        t: temp,
                        point: point,
                        normal: point.sub_vec3(&self.center).div_t(self.radius)
                    }
                );
            }
        }

        return None;
    }
}

impl Model for Vec<Box<Model>> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut best_result = None;
        let mut closest: f64 = t_max;

        for child in self {
            if let Some(hit) = child.hit(ray, t_min, closest) {
                match best_result {
                    None => best_result = Some(hit),
                    Some(prev) => if hit.t < prev.t {
                        closest = hit.t;
                        best_result = Some(hit)
                    }
                }
            }
        }

        best_result
    }
}

fn color_from_ray(ray: &Ray, scene: &Box<Model>) -> Vec3 {
    match scene.hit(ray, 0.0, f64::INFINITY) {
        Some(hit) => {
            let normalized = hit.normal.to_unit_vec3();

            return Vec3::new(
                normalized.p1 + 1.0,
                normalized.p2 + 1.0,
                normalized.p3 + 1.0
            ).mul_t(0.5);
        },
        None => {
            let unit_direction = ray.direction.to_unit_vec3();
            let t: f64 = 0.5 * (unit_direction.y() + 1.0);

            return Vec3::new(1.0, 1.0, 1.0)
                .mul_t(1.0 - t)
                .add_vec3(
                    &Vec3::new(0.5, 0.7, 1.0)
                    .mul_t(t)
                );
        }
    }
}

fn main() {
    let width = 400;
    let height = 200;
    let number_of_samples = 25;

    let mut image = PPM::new(width, height);
    image.write_header().unwrap();

    let scene: Box<Model> =
        Box::new(
            vec![
                Box::new(Sphere {
                    center: Vec3::new(0.0, 0.0, -1.0),
                    radius: 0.5,
                }) as Box<Model>,
                Box::new(Sphere {
                    center: Vec3::new(0.0, -100.5, -1.0),
                    radius: 100.0,
                }) as Box<Model>,
            ]
        );

    let camera = Camera::new();

    for j in (0..height).rev() {
        for i in 0..width {
            let mut sampled_color = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..number_of_samples {
                let u: f64 = (i as f64 + rand::random::<f64>()) / width as f64;
                let v: f64 = (j as f64 + rand::random::<f64>()) / height as f64;
                let ray = camera.get_ray(u, v);

                sampled_color = sampled_color.add_vec3(
                    &color_from_ray(&ray, &scene).mul_t(255.99)
                );
            }

            sampled_color = sampled_color.div_t(number_of_samples as f64);

            image.write_pixel(&sampled_color).unwrap();
        }
    }
}
