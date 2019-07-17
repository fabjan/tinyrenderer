use std::io::BufWriter;
use std::io::Write;

use crate::image::Image;

impl Image {
    pub fn write<W: Write>(&self, writer: &mut BufWriter<W>) {
        let header = format!("P6 {} {} {}\n", self.width, self.height, 255);
        writer.write_all(header.as_bytes())
            .expect("unable to write image header");
        for p in &self.pixels {
            writer.write_all(p)
                .expect("unable to write pixel data");
        }
    }
}
