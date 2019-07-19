extern crate rand;
extern crate tinyrenderer;

use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::stdout;
use std::time::Instant;

use tinyrenderer::image::Image;
use tinyrenderer::model::Model;
use tinyrenderer::geometry::Vec2f;
use tinyrenderer::geometry::Vec3f;
use tinyrenderer::render::to_screen_coords;
use tinyrenderer::render::triangle_diffuse;

fn main() {

    eprint!("loading model...");
    let timer = Instant::now();
    let obj_file = File::open("african_head.obj").expect("obj file missing");
    let head = Model::from_obj(BufReader::new(obj_file));
    eprintln!("...done ({}ms)", timer.elapsed().as_millis());

    eprint!("loading diffuse texture...");
    let timer = Instant::now();
    let mut texture_file = File::open("african_head_diffuse.tga").expect("diffuse texture missing");
    let mut texture_image = Image::from_tga(&mut texture_file);
    texture_image.flip();
    eprintln!("...done ({}ms)", timer.elapsed().as_millis());

    let light_dir = Vec3f { x: 0., y: 0., z: -1. };
    let width = 800;
    let height = 800;

    let mut canvas = Image::make(width, height);
    canvas.flip();

    let translate = Vec2f { x: 1.0, y: 1.0 }; // obj verts are around both sides of 0
    let scale = Vec2f { x: width as f64 / 2.0, y: height as f64 / 2.0 };

    let mut zbuffer = vec![std::f64::MIN; width*height];

    eprint!("rendering...");
    let timer = Instant::now();
    for (_i, face) in head.faces().enumerate() {
        let w0 = head.vert(face.verts.x as usize);
        let w1 = head.vert(face.verts.y as usize);
        let w2 = head.vert(face.verts.z as usize);
        let s0 = to_screen_coords(w0, translate, scale);
        let s1 = to_screen_coords(w1, translate, scale);
        let s2 = to_screen_coords(w2, translate, scale);
        let uv0 = head.uv(face.uvs.x as usize);
        let uv1 = head.uv(face.uvs.y as usize);
        let uv2 = head.uv(face.uvs.z as usize);
        let n = (w2-w0).cross(w1-w0).normalized();
        let intensity = n*light_dir;
        if intensity > 0. {
            triangle_diffuse(&mut canvas, &mut zbuffer, s0, s1, s2, uv0, uv1, uv2, &texture_image, intensity);
        }
    }
    eprintln!("...done ({}ms)", timer.elapsed().as_millis());

    eprint!("saving image...");
    let timer = Instant::now();
    let mut writer = BufWriter::new(stdout());
    canvas.write(&mut writer);
    eprintln!("...done ({}ms)", timer.elapsed().as_millis());
}
