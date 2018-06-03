extern crate image;
extern crate rand;

use std::f64;
use std::f64::consts::PI;

use image::ImageBuffer;

mod camera;
mod ray;
mod vec;
mod model;
mod material;

use vec::Vec3;
use ray::Ray;
use camera::Camera;
use model::{Model, Sphere};
use material::Lambertian;

fn color_from_ray(mut ray: Ray, scene: &Box<Model>) -> Vec3 {
    let white = Vec3::new(1.0, 1.0, 1.0);
    let sky_blue = Vec3::new(0.5, 0.7, 1.0).mul_t(0.3).add_vec3(&white.mul_t(0.7));

    let mut attenuation = Vec3::new(1.0, 1.0, 1.0);
    let mut depth = 0;

    while let Some(hit) = scene.hit(&ray, 0.001, f64::INFINITY) {
        let scatter_record = hit.material.scatter(&ray, &hit);

        attenuation = attenuation.mul_vec3(&scatter_record.color);
        if let Some(scatter_ray) = scatter_record.ray {
            ray = scatter_ray;
        }

        depth += 1;
        if depth > 50 {
            break;
        }
    }

    let sun_direction = Vec3::new(1.0, 1.0, 1.0).to_unit_vec3();
    let unit_direction = ray.direction.to_unit_vec3();
    let t = 0.5 * (unit_direction.y() + 1.0);

    if sun_direction.dot(&unit_direction) >= (5.0 * PI / 180.0).cos() {
        return Vec3::new(5.0, 5.0, 3.0).mul_vec3(&attenuation);
    } else {
        let sky = white.mul_t(1.0 - t).add_vec3(&sky_blue.mul_t(t));
        return attenuation.mul_vec3(&sky);
    }
}

fn main() {
    let width = 400;
    let height = 200;
    let number_of_samples = 50;

    let mut image = ImageBuffer::new(width, height);

    let scene: Box<Model> =
        Box::new(
            vec![
                Box::new(
                    Sphere {
                        center: Vec3::new(0.0, 0.005, -1.0),
                        radius: 0.5,
                        material: Box::new(Lambertian {
                            albedo: Vec3::new(0.8, 0.3, 0.3)
                        }),
                    }
                ) as Box<Model>,
                Box::new(
                    Sphere {
                        center: Vec3::new(0.0, -100.5, -1.0),
                        radius: 100.0,
                        material: Box::new(Lambertian {
                            albedo: Vec3::new(0.52, 0.58, 0.68)
                        }),
                    }
                ) as Box<Model>,
            ]
        );

    let camera = Camera::new();

    for y in (0..height).rev() {
        let j = height - y;

        for x in 0..width {
            let mut sampled_color = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..number_of_samples {
                let u: f64 = (x as f64 + rand::random::<f64>()) / width as f64;
                let v: f64 = (j as f64 + rand::random::<f64>()) / height as f64;
                let ray = camera.get_ray(u, v);

                sampled_color = sampled_color.add_vec3(
                    &color_from_ray(ray, &scene)
                );
            }

            sampled_color = sampled_color
                .div_t(number_of_samples as f64);

            sampled_color = Vec3 {
                p1: sampled_color.p1.sqrt(),
                p2: sampled_color.p2.sqrt(),
                p3: sampled_color.p3.sqrt(),
            }.mul_t(255.99);

            let pixel = image::Rgb([sampled_color.p1 as u8, sampled_color.p2 as u8, sampled_color.p3 as u8]);

            image.put_pixel(x, y, pixel);
        }
    }

    image::ImageRgb8(image).save("image.png").unwrap();
}
