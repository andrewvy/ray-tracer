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

    fn sub_vec3(&self, v2: &Vec3) -> Vec3 {
        Vec3 {
            p1: self.p1 - v2.p1,
            p2: self.p2 - v2.p2,
            p3: self.p3 - v2.p3
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

    fn dot(&self, v2: &Vec3) -> f64 {
        self.p1 * v2.p1 + self.p2 * v2.p2 + self.p3 + v2.p3
    }

    fn cross(&self, v2: &Vec3) -> Vec3 {
        Vec3 {
            p1: (self.p2 * v2.p3 - self.p3 * v2.p2),
            p2: (self.p1 * v2.p3 - self.p3 * v2.p1),
            p3: (self.p1 * v2.p2 - self.p2 * v2.p1)
        }
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

fn hit_sphere(center: &Vec3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin.sub_vec3(&center);
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = (b * b) - (4.0 * a * c);

    return discriminant > 0.0;
}

fn color_from_ray(ray: &Ray) -> Vec3 {
    let center = Vec3::new(0.0, 0.0, -1.0);
    let radius = 0.5;

    if hit_sphere(&center, radius, ray) {
        return Vec3::new(1.0, 0.0, 0.0);
    }

    let unit_direction = ray.direction.to_unit_vec3();
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);

    Vec3::new(1.0, 1.0, 1.0)
        .mul_t(1.0 - t)
        .add_vec3(
            &Vec3::new(0.5, 0.7, 1.0)
            .mul_t(t)
        )
}

struct PPM {
    width: i64,
    height: i64
}

impl PPM {
    pub fn new(width: i64, height: i64) -> PPM {
        PPM {
            width: width,
            height: height
        }
    }

    fn write_header(&self) -> Result<(), io::Error> {
        io::stdout().write(
            format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes()
        ).map(|_| ())
    }

    fn write_pixel(&self, color: &Vec3) -> Result<(), io::Error> {
        io::stdout().write(
            format!("{} {} {}\n", color.r() as i64, color.g() as i64, color.b() as i64).as_bytes()
        ).map(|_| ())
    }
}

fn main() {
    let width = 200;
    let height = 100;

    let image = PPM::new(width, height);
    image.write_header().unwrap();

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..height).rev() {
        for i in 0..width {
            let u: f64 = i as f64 / width as f64;
            let v: f64 = j as f64 / height as f64;

            let u_vec3 = &horizontal.mul_t(u);
            let v_vec3 = &vertical.mul_t(v);
            let direction = lower_left_corner
                .add_vec3(&u_vec3)
                .add_vec3(&v_vec3);

            let ray = Ray::new(
                &origin,
                &direction
            );

            let color = color_from_ray(&ray).mul_t(255.99);
            image.write_pixel(&color).unwrap();
        }
    }
}
