//! Bitmaps and simple operations on them

pub type Color = [u8; 3];

/// An Image is a Vector of pixel colors with some
/// metadata describing how to interpret the pixels
pub struct Image {
    /// how many pixels are in each bitmap row
    pub width: usize,
    /// how many rows are in the bitmap
    pub height: usize,
    /// if true, the first pixel row in the vector is the last row of the bitmap
    pub flipped: bool,
    pub pixels: Vec<Color>,
}

impl Image {
    pub fn make(w: usize, h: usize) -> Image {
        Image {
            width: w,
            height: h,
            flipped: false,
            pixels: vec![[0, 0, 0]; w * h],
        }
    }

    /// how many pixels are there?
    pub fn size(&self) -> usize {
        self.width * self.height
    }

    /// flip the flip field (does not mutate any pixel data)
    pub fn flip(&mut self) {
        self.flipped = !self.flipped;
    }

    pub fn put(&mut self, x: usize, y: usize, color: Color) {
        let real_y = if self.flipped { self.height - y } else { y };
        let pixel_index = x % self.width + real_y * self.width;
        if (0..self.pixels.len()).contains(&pixel_index) {
            self.pixels[pixel_index] = color;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Color {
        let y = if self.flipped { self.height - y } else { y };
        let pixel_index = x % self.width + y * self.width;
        self.pixels[pixel_index]
    }

    /// get a pixel using float coordinates between 0.0 and 1.0
    pub fn get_unit(&self, x: f64, y: f64) -> Color {
        let x = (x * self.width as f64) as usize;
        let y = (y * self.height as f64) as usize;
        self.get(x, y)
    }
}
