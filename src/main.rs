extern crate tinyrenderer;

use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::stdout;
use std::time::Instant;

use tinyrenderer::image::Image;
use tinyrenderer::model::Model;
use tinyrenderer::geometry::Vec3f;

use tinyrenderer::render::lookat;
use tinyrenderer::render::projection;
use tinyrenderer::render::viewport;
use tinyrenderer::render::triangle;

use tinyrenderer::render::Shader;
use tinyrenderer::shaders::GouraudShader;

macro_rules! f {
    ($e:expr) => ($e as f64);
}

const WIDTH: usize  = 800;
const HEIGHT: usize = 800;

fn main() {

    // setup scene
    let light  = Vec3f::new(1.0, 1.0, 1.0).normalized();
    let eye    = Vec3f::new(1.0, 1.0, 3.0);
    let center = Vec3f::new(0.0, 0.0, 0.0);

    let model_view = lookat(eye, center, Vec3f::new(0.0, 1.0, 0.0));
    let projection = projection(-1.0/(eye-center).norm());
    let view_port  = viewport(f!(WIDTH)/8.0, f!(HEIGHT)/8.0,
                              f!(WIDTH)*0.75, f!(HEIGHT)*0.75, 255.);
    let vpmv = &view_port*&projection*&model_view;

    // load resources
    let head = load_obj("african_head.obj");
    let texture_image = load_tga("african_head_diffuse.tga");

    let mut shader = GouraudShader::new(&head, &vpmv, &texture_image, light);

    // draw stuff!
    eprint!("rendering...");
    let timer = Instant::now();
    let mut canvas = Image::make(WIDTH, HEIGHT);
    canvas.flip();
    let mut zbuffer = vec![std::f64::MIN; WIDTH*HEIGHT];
    for i in 0..(head.nfaces()) {
        let v0 = shader.vertex(i, 0);
        let v1 = shader.vertex(i, 1);
        let v2 = shader.vertex(i, 2);
        triangle(&mut canvas, &mut zbuffer, &mut shader, v0, v1, v2);
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
