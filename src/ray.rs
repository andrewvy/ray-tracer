use vec::Vec3;

pub struct Ray<'a> {
    pub origin: &'a Vec3,
    pub direction: &'a Vec3,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Vec3, direction: &'a Vec3) -> Ray<'a> {
        Ray {
            origin: origin,
            direction: direction
        }
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin.add_vec3(
            &self.direction.mul_t(t)
        )
    }
}
