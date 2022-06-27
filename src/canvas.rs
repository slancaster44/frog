use crate::color;
use std::fs;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub contents: Vec<color::Color>,
}

pub fn new(w: usize, h: usize) -> Canvas {
    return Canvas {
        width: w,
        height: h,
        contents: vec![color::new(0.0, 0.0, 0.0); w * h],
    };
}

impl Canvas {
    pub fn plot(&mut self, x: usize, y: usize, c1: color::Color) {
        self.check_location(x, y);
        let location = (y * self.width) + x;
        self.contents[location] = c1;
    }

    pub fn read(&self, x: usize, y: usize) -> color::Color {
        self.check_location(x, y);
        let location = (y * self.width) + x;
        return self.contents[location];
    }

    pub fn check_location(&self, x: usize, y: usize) {
        if x > self.width || y > self.height {
            panic!("Invalid plot location: {} {}", x, y);
        }
    }

    pub fn write_to_ppm(&self, filename: &str) {
        let mut file = fs::File::create(filename).expect("PPM file creation failed");

        let header = format!("P3\n{} {}\n255\n", self.width, self.height);
        file.write_all(header.as_bytes()).expect("Failed to write PPM header");

        for i in &self.contents {
            let pixel_in_255 = i.scaled_from_1_to_255();
            let pixel_as_text = format!("{} {} {}\n", pixel_in_255.red, pixel_in_255.green, pixel_in_255.blue);
            file.write_all(pixel_as_text.as_bytes()).expect("Failed to write pixel");
        }
    }
}
