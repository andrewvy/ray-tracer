use rand;
use vec::Vec3;
use ray::Ray;
use model::HitRecord;

fn random_in_unit_sphere() -> Vec3 {
    let bounds = Vec3::new(1.0, 1.0, 1.0);
    let mut point: Vec3;

    while {
        point = Vec3::new(
            rand::random::<f64>(),
            rand::random::<f64>(),
            rand::random::<f64>()
        ).mul_t(2.0).sub_vec3(&bounds);

        point.squared_length() >= 1.0
    } {}

    return point;
}


pub struct ScatterRecord {
    pub color: Vec3,
    pub ray: Option<Ray>,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> ScatterRecord;
}

pub struct Lambertian {
    pub albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord) -> ScatterRecord {
        let target = hit.point.add_vec3(&hit.normal).add_vec3(&random_in_unit_sphere());
        let shadow_ray = Ray::new(hit.point, target.sub_vec3(&hit.point));

        ScatterRecord {
            color: self.albedo,
            ray: Some(shadow_ray),
        }
    }
}
