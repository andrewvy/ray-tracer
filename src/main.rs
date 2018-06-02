extern crate rand;

use std::f64;

mod camera;
mod image;
mod ray;
mod vec;
mod model;
mod material;

use vec::Vec3;
use ray::Ray;
use image::PPM;
use camera::Camera;
use model::{Model, Sphere};

fn color_from_ray(ray: &Ray, scene: &Box<Model>, depth: i64) -> Vec3 {
    match scene.hit(ray, 0.001, f64::INFINITY) {
        Some(hit) => {
            if depth < 50 {
                let scatter_record = hit.material.scatter(&ray, &hit);

                if let Some(scatter_ray) = scatter_record.ray {
                    return color_from_ray(
                        &scatter_ray,
                        scene,
                        depth + 1
                    ).mul_vec3(&scatter_record.color);
                } else {
                    return Vec3::new(0.0, 0.0, 0.0);
                }
            } else {
                return Vec3::new(0.0, 0.0, 0.0);
            }
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
    let number_of_samples = 50;

    let mut image = PPM::new(width, height);
    image.write_header().unwrap();

    let scene: Box<Model> =
        Box::new(
            vec![
                Box::new(
                    Sphere::new(
                        Vec3::new(0.0, 0.005, -1.0),
                        0.5
                    )
                ) as Box<Model>,
                Box::new(
                    Sphere::new(
                        Vec3::new(0.0, -100.5, -1.0),
                        100.0
                    )
                ) as Box<Model>,
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
                    &color_from_ray(&ray, &scene, 0)
                );
            }

            sampled_color = sampled_color
                .div_t(number_of_samples as f64);

            sampled_color = Vec3 {
                p1: sampled_color.p1.sqrt(),
                p2: sampled_color.p2.sqrt(),
                p3: sampled_color.p3.sqrt(),
            }.mul_t(255.99);

            image.write_pixel(&sampled_color).unwrap();
        }
    }
}
