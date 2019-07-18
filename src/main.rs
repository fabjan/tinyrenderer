extern crate rand;
extern crate tinyrenderer;

use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::stdout;

use rand::prelude::random;

use tinyrenderer::image::Image;
use tinyrenderer::model::Model;
use tinyrenderer::geometry::Vec2f;

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
        let t1 = Vec2f {
            x: (vertices[0].x+1.) * fwidth/2.,
            y: (vertices[0].y+1.) * fheight/2.,
        };
        let t2 = Vec2f {
            x: (vertices[1].x+1.) * fwidth/2.,
            y: (vertices[1].y+1.) * fheight/2.,
        };
        let t3 = Vec2f {
            x: (vertices[2].x+1.) * fwidth/2.,
            y: (vertices[2].y+1.) * fheight/2.,
        };
        let color: [u8; 3] = [ random(), random(), random() ];
        image.triangle(t1, t2, t3, color);
    }

    let mut writer = BufWriter::new(stdout());
    image.write(&mut writer);
}
