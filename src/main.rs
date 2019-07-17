use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::stdout;

extern crate tinyrenderer;
use tinyrenderer::image::Image;
use tinyrenderer::model::Model;

fn main() {

    let obj_file = File::open("head.obj").expect("file `head.obj` missing");
    let head = Model::from_obj(BufReader::new(obj_file));

    let width = 800;
    let height = 800;
    let fwidth = width as f64;
    let fheight = height as f64;
    let mut image = Image::make(width, height);

    let white = [255, 255, 255];

    image.flip();
    for face in head.faces() {
        let vertices = [head.vert(face.x as usize), head.vert(face.y as usize), head.vert(face.z as usize)];
        for v in 0..3 {
            let v0 = vertices[v];
            let v1 = vertices[(v+1) % 3];
            let (x0, y0, x1, y1) = (
                (v0.x+1.) * fwidth/2.,
                (v0.y+1.) * fheight/2.,
                (v1.x+1.) * fwidth/2.,
                (v1.y+1.) * fheight/2.,
            );
            image.line(x0, y0, x1, y1, white);
        }
    }

    let mut writer = BufWriter::new(stdout());
    image.write(&mut writer);
}
