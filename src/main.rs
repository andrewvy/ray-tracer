use std::io::{self, Write};

struct Vec3 {
    p1: f64,
    p2: f64,
    p3: f64
}

impl Vec3 {
    pub fn new(p1: f64, p2: f64, p3: f64) -> Vec3 {
        Vec3 {
            p1: p1,
            p2: p2,
            p3: p3
        }
    }

    fn x(&self) -> f64 { self.p1 }
    fn y(&self) -> f64 { self.p2 }
    fn z(&self) -> f64 { self.p3 }

    fn r(&self) -> f64 { self.p1 }
    fn g(&self) -> f64 { self.p2 }
    fn b(&self) -> f64 { self.p3 }

    fn length(&self) -> f64 {
        (self.p1 * self.p1 + self.p2 * self.p2 + self.p3 * self.p3).sqrt()
    }

    fn add_vec3(&self, v2: &Vec3) -> Vec3 {
        Vec3 {
            p1: self.p1 + v2.p1,
            p2: self.p2 + v2.p2,
            p3: self.p3 + v2.p3
        }
    }

    fn mul_t(&self, t: f64) -> Vec3 {
        Vec3 {
            p1: self.p1 * t,
            p2: self.p2 * t,
            p3: self.p3 * t
        }
    }

    fn div_t(&self, t: f64) -> Vec3 {
        Vec3 {
            p1: self.p1 / t,
            p2: self.p2 / t,
            p3: self.p3 / t
        }
    }

    fn to_unit_vec3(&self) -> Vec3 {
        self.div_t(self.length())
    }
}

struct Ray<'a> {
    origin: &'a Vec3,
    direction: &'a Vec3
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Vec3, direction: &'a Vec3) -> Ray<'a> {
        Ray {
            origin: origin,
            direction: direction
        }
    }

    fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin.add_vec3(
            &self.direction.mul_t(t)
        )
    }
}

fn color_from_ray(ray: &Ray) -> Vec3 {
    let unit_direction = ray.direction.to_unit_vec3();
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);

    Vec3::new(1.0, 1.0, 1.0)
        .mul_t(1.0 - t)
        .add_vec3(
            &Vec3::new(0.5, 0.7, 1.0)
            .mul_t(t)
        )
}

fn main() {
    let nx = 200;
    let ny = 100;

    io::stdout().write(
        format!("P3\n{} {}\n255\n", nx, ny).as_bytes()
    ).unwrap();

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u: f64 = i as f64 / nx as f64;
            let v: f64 = j as f64 / ny as f64;

            let u_vec3 = &horizontal.mul_t(u);
            let v_vec3 = &vertical.mul_t(v);
            let direction = lower_left_corner
                .add_vec3(&u_vec3)
                .add_vec3(&v_vec3);

            let ray = Ray::new(
                &origin,
                &direction
            );

            let color = color_from_ray(&ray);

            let ir = 255.99 * color.r();
            let ig = 255.99 * color.g();
            let ib = 255.99 * color.b();

            io::stdout().write(
                format!("{} {} {}\n", ir as i64, ig as i64, ib as i64).as_bytes()
            ).unwrap();
        }
    }
}
