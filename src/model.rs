use vec::Vec3;
use ray::Ray;
use material::{Material, Lambertian};

#[derive(Clone, Copy)]
pub struct HitRecord<'a> {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub material: &'a Material,
}

pub trait Model {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Box<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: Box::new(Lambertian {
                albedo: Vec3::new(0.8, 0.3, 0.3)
            })
        }
    }
}

impl Model for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin.sub_vec3(&self.center);
        let a: f64 = ray.direction.dot(&ray.direction);
        let b: f64 = oc.dot(&ray.direction);
        let c: f64 = oc.dot(&oc) - self.radius * self.radius;
        let discriminant: f64 = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - (b*b - a*c).sqrt()) / a;

            if temp < t_max && temp > t_min {
                let point = ray.point_at_parameter(temp);

                return Some(
                    HitRecord {
                        t: temp,
                        point: point.clone(),
                        normal: point.sub_vec3(&self.center).div_t(self.radius),
                        material: &*self.material,
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
                        normal: point.sub_vec3(&self.center).div_t(self.radius),
                        material: &*self.material,
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

