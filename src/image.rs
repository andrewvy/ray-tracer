use std::io::{self, Write};

use vec::Vec3;

pub struct PPM {
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

    pub fn write_header(&self) -> Result<(), io::Error> {
        io::stdout().write(
            format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes()
        ).map(|_| ())
    }

    pub fn write_pixel(&self, color: &Vec3) -> Result<(), io::Error> {
        io::stdout().write(
            format!("{} {} {}\n", color.r() as i64, color.g() as i64, color.b() as i64).as_bytes()
        ).map(|_| ())
    }
}
