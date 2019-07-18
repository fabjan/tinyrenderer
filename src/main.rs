extern crate rand;
extern crate tinyrenderer;

use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::stdout;

use tinyrenderer::image::Image;
use tinyrenderer::model::Model;
use tinyrenderer::geometry::Vec3f;

fn main() {

    let obj_file = File::open("african_head.obj").expect("obj file missing");
    let head = Model::from_obj(BufReader::new(obj_file));
    let light_dir = Vec3f { x: 0., y: 0., z: -1. };

    let width = 800;
    let height = 800;
    let fwidth = width as f64;
    let fheight = height as f64;
    let mut image = Image::make(width, height);
    let mut zbuffer = vec![std::f64::MIN; width*height];

    image.flip();
    for face in head.faces() {
        let world_coords = [head.vert(face.x as usize), head.vert(face.y as usize), head.vert(face.z as usize)];
        let screen_coords: Vec<Vec3f> = world_coords.iter()
            //.map(|v| Vec2f { x: (v.x+1.) * fwidth/2., y: (v.y+1.) * fheight/2. } )
            .map(|v| Vec3f { x: (v.x+1.) * fwidth/2., y: (v.y+1.) * fheight/2., z: v.z } )
            .collect();
        let mut n = (world_coords[2]-world_coords[0]).cross(world_coords[1]-world_coords[0]);
        n.normalize();
        let intensity = n*light_dir;
        if intensity > 0. {
            let intensity = (intensity*255.) as u8;
            let color = [intensity, intensity, intensity];
            image.triangle_points(&mut zbuffer, screen_coords[0], screen_coords[1], screen_coords[2], color);
        }
    }

    let mut writer = BufWriter::new(stdout());
    image.write(&mut writer);
}
