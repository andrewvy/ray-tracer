#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub p1: f64,
    pub p2: f64,
    pub p3: f64,
}

impl Vec3 {
    pub fn new(p1: f64, p2: f64, p3: f64) -> Vec3 {
        Vec3 {
            p1: p1,
            p2: p2,
            p3: p3,
        }
    }

    pub fn x(&self) -> f64 { self.p1 }
    pub fn y(&self) -> f64 { self.p2 }
    pub fn z(&self) -> f64 { self.p3 }

    pub fn r(&self) -> f64 { self.p1 }
    pub fn g(&self) -> f64 { self.p2 }
    pub fn b(&self) -> f64 { self.p3 }

    pub fn squared_length(&self) -> f64 {
        self.dot(self)
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn add_vec3(&self, v2: &Vec3) -> Vec3 {
        Vec3 {
            p1: self.p1 + v2.p1,
            p2: self.p2 + v2.p2,
            p3: self.p3 + v2.p3,
        }
    }

    pub fn sub_vec3(&self, v2: &Vec3) -> Vec3 {
        Vec3 {
            p1: self.p1 - v2.p1,
            p2: self.p2 - v2.p2,
            p3: self.p3 - v2.p3,
        }
    }

    pub fn mul_vec3(&self, v2: &Vec3) -> Vec3 {
        Vec3 {
            p1: self.p1 * v2.p1,
            p2: self.p2 * v2.p2,
            p3: self.p3 * v2.p3,
        }
    }

    pub fn mul_t(&self, t: f64) -> Vec3 {
        Vec3 {
            p1: self.p1 * t,
            p2: self.p2 * t,
            p3: self.p3 * t,
        }
    }

    pub fn div_t(&self, t: f64) -> Vec3 {
        Vec3 {
            p1: self.p1 / t,
            p2: self.p2 / t,
            p3: self.p3 / t,
        }
    }

    pub fn to_unit_vec3(&self) -> Vec3 {
        self.div_t(self.length())
    }

    pub fn dot(&self, v2: &Vec3) -> f64 {
        self.p1 * v2.p1 + self.p2 * v2.p2 + self.p3 * v2.p3
    }

    pub fn cross(&self, v2: &Vec3) -> Vec3 {
        Vec3 {
            p1: (self.p2 * v2.p3 - self.p3 * v2.p2),
            p2: (self.p1 * v2.p3 - self.p3 * v2.p1),
            p3: (self.p1 * v2.p2 - self.p2 * v2.p1)
        }
    }
}
