use std::io::stdout;
use std::io::BufWriter;

extern crate tinyrenderer;
use tinyrenderer::image::Image;

fn main() {

    let width = 100;
    let mut image = Image::make(width, width);
    let white = [255, 255, 255];
    let red   = [255,   0,   0];
    let green = [  0, 255,   0];

    image.flip();
    image.line(13, 20, 80, 40, white);
    image.line(20, 13, 40, 80, red); 

    let mut writer = BufWriter::new(stdout());
    image.write(&mut writer);
}
