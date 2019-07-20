extern crate rand;
extern crate tinyrenderer;

use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::stdout;
use std::time::Instant;

use tinyrenderer::image::Image;
use tinyrenderer::model::Model;
use tinyrenderer::geometry::Matrix;
use tinyrenderer::geometry::Vec3f;
use tinyrenderer::render::lookat;
use tinyrenderer::render::m2v;
use tinyrenderer::render::triangle_diffuse;
use tinyrenderer::render::viewport;
use tinyrenderer::render::v2m;

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

    let width = 800;
    let height = 800;
    let depth = 255;

    let light_dir = Vec3f::new(0.2, 0.1, -1.0);
    let eye = Vec3f::new(1.0, 1.0, 3.0);
    let center = Vec3f::new(0.0, 0.0, 0.0);

    let model_view = lookat(eye, center, Vec3f::new(0.0, 1.0, 0.0));
    let mut projection = Matrix::identity(4);
    projection.put(3, 2, -1.0/(eye-center).norm());
    let view_port = viewport(width as f64/8.0, height as f64/8.0, width as f64*0.75, height as f64*0.75, depth as f64);

    let vpmv = &view_port*&projection*&model_view;

    eprint!("rendering...");
    let timer = Instant::now();
    let mut canvas = Image::make(width, height);
    canvas.flip();
    let mut zbuffer = vec![std::f64::MIN; width*height];
    for (_i, face) in head.faces().enumerate() {
        let w0 = head.vert(face.verts.x as usize);
        let w1 = head.vert(face.verts.y as usize);
        let w2 = head.vert(face.verts.z as usize);
        let s0 = m2v(&vpmv*&v2m(w0));
        let s1 = m2v(&vpmv*&v2m(w1));
        let s2 = m2v(&vpmv*&v2m(w2));
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
