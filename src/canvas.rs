use crate::color;
use std::fs;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub origin: (i32, i32),
    pub width: usize,
    pub height: usize,
    pub contents: Vec<color::Color>,
}

pub fn new(w: usize, h: usize) -> Canvas {
    return Canvas {
        origin: (w as i32 / 2 as i32, h as i32 / 2 as i32),
        width: w,
        height: h,
        contents: vec![color::new(0.0, 0.0, 0.0); w * h],
    };
}

impl Canvas {
    pub fn plot(&mut self, x: i32, y: i32, c1: color::Color) {
        let location = self.convert_location(x, y);
        self.contents[location] = c1;
    }

    fn convert_location(&self, x_arg: i32, y_arg: i32) -> usize {
        let x = x_arg + self.origin.0;
        let y = y_arg + self.origin.1;

        if x < 0 || y < 0 {
            panic!("Coordinate out of bounds ({}, {})", x_arg, y_arg);
        } else if x as usize > self.width || y as usize > self.height {
            panic!("Coordinate out of bounds ({}, {})", x_arg, y_arg);
        }

        return ((y as usize) * self.width) + (x as usize);
    }

    pub fn read(&self, x: i32, y: i32) -> color::Color {
        let location = self.convert_location(x, y);
        return self.contents[location];
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
