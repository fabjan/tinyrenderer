extern crate tinyrenderer;

use std::env;
use std::fs::File;
use std::io::stdout;
use std::io::BufReader;
use std::io::BufWriter;
use std::time::Instant;

use tinyrenderer::geometry::Vec3f;
use tinyrenderer::image::Image;
use tinyrenderer::model::Model;

use tinyrenderer::render::lookat;
use tinyrenderer::render::projection;
use tinyrenderer::render::triangle;
use tinyrenderer::render::viewport;

use tinyrenderer::render::Shader;
use tinyrenderer::shaders::GouraudShader;

// I wanted to macro, don't judge!
macro_rules! f {
    ($e:expr) => {
        $e as f64
    };
}

fn main() {
    let mut args = env::args();
    args.next(); // we don't care about the command name, shift it
    let model_obj = args
        .next()
        .expect("argv[1] should be an obj file with the model");
    let texture_tga = args
        .next()
        .expect("argv[2] should be a TGA file with the texture");
    let width = args
        .next()
        .unwrap_or_else(|| "800".to_owned())
        .parse()
        .expect("argv[3] should be the image size");
    let height = width;

    // load resources
    let head = load_obj(model_obj.as_str());
    let texture_image = load_tga(texture_tga.as_str());
    eprintln!("model has {} faces", head.nfaces());

    // setup scene
    // TODO read from TOML?
    let light = Vec3f::new(1.0, 1.0, 1.0).normalized();
    let eye = Vec3f::new(1.0, 1.0, 3.0);
    let center = Vec3f::new(0.0, 0.0, 0.0);
    let model_view = lookat(eye, center, Vec3f::new(0.0, 1.0, 0.0));
    let projection = projection(-1.0 / (eye - center).norm());

    let view_port = viewport(
        f!(width) / 8.0,
        f!(height) / 8.0,
        f!(width) * 0.75,
        f!(height) * 0.75,
        255.,
    );
    let vpmv = &view_port * &projection * &model_view;

    let mut shader = GouraudShader::new(&head, &vpmv, &texture_image, light);

    let mut timer = Timer::default();

    // draw stuff!
    timer.start("rendering");
    let mut canvas = Image::make(width, height);
    canvas.flip();
    let mut zbuffer = vec![std::f64::MIN; width * height];
    for i in 0..(head.nfaces()) {
        let v0 = shader.vertex(i, 0);
        let v1 = shader.vertex(i, 1);
        let v2 = shader.vertex(i, 2);
        triangle(&mut canvas, &mut zbuffer, &mut shader, v0, v1, v2);
    }
    timer.stop();

    timer.start("saving image");
    let mut writer = BufWriter::new(stdout());
    canvas.write(&mut writer);
    timer.stop();
}

fn load_obj(filename: &str) -> Model {
    let mut t = Timer::default();
    t.start(&format!("loading model {}", filename));
    let obj_file = File::open(filename).expect("unable to open OBJ file");
    let model = Model::from_obj(BufReader::new(obj_file));
    t.stop();
    model
}

fn load_tga(filename: &str) -> Image {
    let mut t = Timer::default();
    t.start(&format!("loading texture {}", filename));
    let mut texture_file = File::open(filename).expect("unable to open TGA file");
    let texture = Image::from_tga(&mut texture_file);
    t.stop();
    texture
}

struct Timer {
    since: Instant,
}

impl Timer {
    fn start(self: &mut Self, message: &str) -> &mut Self {
        eprint!("{}...", message);
        self.since = Instant::now();
        self
    }
    fn stop(self: &Self) {
        eprintln!("...done ({}ms)", self.since.elapsed().as_millis());
    }
}

impl Default for Timer {
    fn default() -> Self {
        Timer {
            since: Instant::now(),
        }
    }
}
