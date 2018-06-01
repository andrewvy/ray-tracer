use std::io::{self, Write};
use std::fs::File;

use vec::Vec3;

pub struct PPM {
    width: i64,
    height: i64,
    file: File,
}

impl PPM {
    pub fn new(width: i64, height: i64) -> PPM {
        let file = File::create("image.ppm").unwrap();

        PPM {
            width: width,
            height: height,
            file: file,
        }
    }

    pub fn write_header(&mut self) -> Result<(), io::Error> {
        self.file.write(
            format!("P3\n{} {}\n255\n", self.width, self.height).as_bytes()
        )?;

        Ok(())
    }

    pub fn write_pixel(&mut self, color: &Vec3) -> Result<(), io::Error> {
        self.file.write(
            format!("{} {} {}\n", color.r() as i64, color.g() as i64, color.b() as i64).as_bytes()
        )?;

        Ok(())
    }
}
