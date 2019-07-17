use std::mem;

type Color = [u8; 3];

pub struct Image {
    pub width: usize,
    pub height: usize,
    pub flipped: bool,
    pub pixels: Vec<Color>,
}

// generic operations
impl Image {
    pub fn make(w: usize, h: usize) -> Image {
        Image {
            width: w,
            height: h,
            flipped: false,
            pixels: vec![[0,0,0]; w*h],
        }
    }
    pub fn size(&self) -> usize {
        self.width * self.height
    }
}

// drawing operations
impl Image {
    pub fn flip(&mut self) {
        self.flipped = !self.flipped;
    }
    pub fn put(&mut self, x: usize, y: usize, color: Color) {
        let real_y = if self.flipped { self.height - y } else { y } ;
        self.pixels[x % self.width + real_y * self.width] = color;
    }
    pub fn line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, color: Color) {
        let (mut x0, mut y0, mut x1, mut y1) = (x0, y0, x1, y1);
        let dx = (x1 as i32 - x0 as i32).abs();
        let dy = (y1 as i32 - y0 as i32).abs();
        let mut steep = false;
        if dx < dy {
            steep = true;
            mem::swap(&mut x0, &mut y0);
            mem::swap(&mut x1, &mut y1);
        }
        if x0 > x1 {
            mem::swap(&mut x0, &mut x1);
            mem::swap(&mut y0, &mut y1);
        }
        for x in x0..x1 {
            let t: f64 = (x as f64 - x0 as f64) / ((x1 - x0) as f64);
            let y: f64 = y0 as f64 * (1. - t) + (y1 as f64 * t);
            if steep {
                self.put(y as usize, x, color);
            } else {
                self.put(x, y as usize, color);
            }
        }
    }
}
