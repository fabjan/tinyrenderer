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
use tinyrenderer::render::projection;
use tinyrenderer::render::triangle_diffuse;
use tinyrenderer::render::viewport;

macro_rules! f {
    ($e:expr) => ($e as f64);
}

const WIDTH: usize  = 800;
const HEIGHT: usize = 800;

fn main() {

    // setup scene
    let light  = Vec3f::new(0.2, 0.1, -1.0);
    let eye    = Vec3f::new(1.0, 1.0, 3.0);
    let center = Vec3f::new(0.0, 0.0, 0.0);

    let model_view = lookat(eye, center, Vec3f::new(0.0, 1.0, 0.0));
    let projection = projection(-1.0/(eye-center).norm());
    let view_port  = viewport(f!(WIDTH)/8.0, f!(HEIGHT)/8.0,
                              f!(WIDTH)*0.75, f!(HEIGHT)*0.75, 255.);

    // load resources
    let head = load_obj("african_head.obj");
    let texture_image = load_tga("african_head_diffuse.tga");

    // draw stuff!
    eprint!("rendering...");
    let timer = Instant::now();
    let vpmv = &view_port*&projection*&model_view;
    let mut canvas = Image::make(WIDTH, HEIGHT);
    canvas.flip();
    let mut zbuffer = vec![std::f64::MIN; WIDTH*HEIGHT];
    for (_i, face) in head.faces().enumerate() {
        let w0 = head.vert(face.verts.x as usize);
        let w1 = head.vert(face.verts.y as usize);
        let w2 = head.vert(face.verts.z as usize);
        let s0 = Vec3f::from_m(&(&vpmv*&Matrix::from_v(w0)));
        let s1 = Vec3f::from_m(&(&vpmv*&Matrix::from_v(w1)));
        let s2 = Vec3f::from_m(&(&vpmv*&Matrix::from_v(w2)));
        let uv0 = head.uv(face.uvs.x as usize);
        let uv1 = head.uv(face.uvs.y as usize);
        let uv2 = head.uv(face.uvs.z as usize);
        let n = (w2-w0).cross(w1-w0).normalized();
        let intensity = n*light;
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

fn load_obj(filename: &str) -> Model {
    eprint!("loading model {}...", filename);
    let timer = Instant::now();
    let obj_file = File::open(filename).expect("unable to open OBJ file");
    let model = Model::from_obj(BufReader::new(obj_file));
    eprintln!("...done ({}ms)", timer.elapsed().as_millis());
    model
}

fn load_tga(filename: &str) -> Image {
    eprint!("loading texture {}...", filename);
    let timer = Instant::now();
    let mut texture_file = File::open(filename).expect("unable to open TGA file");
    let texture = Image::from_tga(&mut texture_file);
    eprintln!("...done ({}ms)", timer.elapsed().as_millis());
    texture
}
