//! Netpbm support,
//! because also implementing _creating_ TGA files seemed like a bother.

use std::io::BufWriter;
use std::io::Write;

use crate::image::Image;

impl Image {
    /// Save the bitmap data as a Netpbm image (P6, a binary PixMap).
    pub fn write<W: Write>(&self, writer: &mut BufWriter<W>) {
        let header = format!("P6 {} {} {}\n", self.width, self.height, 255);
        writer
            .write_all(header.as_bytes())
            .expect("unable to write image header");

        // TODO I don't think this handles flipped images properly.
        for p in &self.pixels {
            writer.write_all(p).expect("unable to write pixel data");
        }
    }
}
