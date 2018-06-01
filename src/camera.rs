use vec::Vec3;
use ray::Ray;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left_corner: Vec3::new(-2.0, -1.0, -1.0),
            horizontal: Vec3::new(4.0, 0.0, 0.0),
            vertical: Vec3::new(0.0, 2.0, 0.0),
            origin: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let u_vec3 = self.horizontal.mul_t(u);
        let v_vec3 = self.vertical.mul_t(v);
        let direction = self.lower_left_corner
            .add_vec3(&u_vec3)
            .add_vec3(&v_vec3);

        Ray::new(
            self.origin,
            direction
        )
    }
}
