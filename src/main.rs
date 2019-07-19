extern crate rand;
extern crate tinyrenderer;

use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::stdout;

use tinyrenderer::image::Image;
use tinyrenderer::model::Model;
use tinyrenderer::geometry::Vec2f;
use tinyrenderer::geometry::Vec3f;
use tinyrenderer::render::to_screen_coords;
use tinyrenderer::render::triangle_flat;

fn main() {

    let obj_file = File::open("african_head.obj").expect("obj file missing");

//    let mut texture_file = File::open("african_head_diffuse.tga").expect("diffuse texture missing");
//    let texture_image = Image::from_tga(&mut texture_file);

    let head = Model::from_obj(BufReader::new(obj_file));
    let light_dir = Vec3f { x: 0., y: 0., z: -1. };

    let width = 800;
    let height = 800;

    let mut canvas = Image::make(width, height);
    canvas.flip();

    let translate = Vec2f { x: 1.0, y: 1.0 }; // obj verts are around both sides of 0
    let scale = Vec2f { x: width as f64 / 2.0, y: height as f64 / 2.0 };

    let mut zbuffer = vec![std::f64::MIN; width*height];

    for face in head.faces() {
        let w0 = head.vert(face.x as usize);
        let w1 = head.vert(face.y as usize);
        let w2 = head.vert(face.z as usize);
        let s0 = to_screen_coords(w0, translate, scale);
        let s1 = to_screen_coords(w1, translate, scale);
        let s2 = to_screen_coords(w2, translate, scale);
        let n = (w2-w0).cross(w1-w0).normalized();
        let intensity = n*light_dir;
        if intensity > 0. {
            let intensity = (intensity*255.) as u8;
            let color = [intensity, intensity, intensity];
            triangle_flat(&mut canvas, &mut zbuffer, s0, s1, s2, color);
        }
    }

    let mut writer = BufWriter::new(stdout());
    canvas.write(&mut writer);
}
