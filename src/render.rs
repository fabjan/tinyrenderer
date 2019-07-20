use std::f64;

use crate::geometry::Matrix;
use crate::geometry::Vec3f;
use crate::image::Color;
use crate::image::Image;

pub trait Shader {
    fn vertex(&mut self, face_i: usize, vert_i: usize) -> Vec3f; // let's hope float is fine
    fn fragment(&mut self, bar: Vec3f, color: &mut Color) -> bool;
}

pub fn triangle(canvas: &mut Image, zbuffer: &mut Vec<f64>, shader: &mut Shader, v0: Vec3f, v1: Vec3f, v2: Vec3f) {
    let (mut xmin, mut ymin) = (f64::MAX, f64::MAX);
    let (mut xmax, mut ymax) = (f64::MIN, f64::MIN);
    for v in [v0, v1, v2].iter() {
        xmin = xmin.min(v.x);
        xmax = xmax.max(v.x);
        ymin = ymin.min(v.y);
        ymax = ymax.max(v.y);
    }
    xmin = xmin.max(0.);
    xmax = xmax.min(canvas.width as f64);
    ymin = ymin.max(0.);
    ymax = ymax.min(canvas.height as f64);
    let mut p = Vec3f::zero();
    for x in (xmin as usize)..(xmax as usize + 1) {
        p.x = x as f64;
        for y in (ymin as usize)..(ymax as usize + 1) {
            p.y = y as f64;
            let bc_screen = barycentric(v0, v1, v2, p);
            if bc_screen.x<0. || bc_screen.y<0. || bc_screen.z<0. { continue };
            p.z  = v0.z*bc_screen.x;
            p.z += v1.z*bc_screen.y;
            p.z += v2.z*bc_screen.z;
            let fragment_index = (p.x as usize) + (p.y as usize)*canvas.width;
            if zbuffer[fragment_index]<p.z {
                zbuffer[fragment_index] = p.z;
                let mut color = [ 0, 0, 0 ];
                let keep_fragment = shader.fragment(bc_screen, &mut color);
                if keep_fragment {
                    canvas.put(p.x as usize, p.y as usize, color);
                }
            }
        }
    }
}

fn barycentric(a: Vec3f, b: Vec3f, c: Vec3f, p: Vec3f) -> Vec3f {
    let xs = Vec3f { x: c.x-a.x, y: b.x-a.x, z: a.x-p.x };
    let ys = Vec3f { x: c.y-a.y, y: b.y-a.y, z: a.y-p.y };
    let u = xs.cross(ys);
    if u.z.abs()>0.01 {
        Vec3f { x: 1. - (u.x+u.y)/u.z, y: u.y/u.z, z: u.x/u.z }
    } else {
        Vec3f { x: -1., y: 1., z: 1. }
    }
}

pub fn viewport(x: f64, y: f64, w: f64, h: f64, depth: f64) -> Matrix {
    let mut vp = Matrix::identity(4);

    vp.put(0, 3, x+w/2.0);
    vp.put(1, 3, y+h/2.0);
    vp.put(2, 3, depth/2.0);

    vp.put(0, 0, w/2.0);
    vp.put(1, 1, h/2.0);
    vp.put(2, 2, depth/2.0);

    vp
}

pub fn projection(coeff: f64) -> Matrix {
    let mut projection = Matrix::identity(4);
    projection.put(3, 2, coeff);
    projection
}

pub fn lookat(eye: Vec3f, center: Vec3f, up: Vec3f) -> Matrix {
    let z = (eye-center).normalized();
    let x = up.cross(z).normalized();
    let y = z.cross(x).normalized();
    let mut res = Matrix::identity(4);
    for i in 0..3 {
        res.put(0, i, x[i]);
        res.put(1, i, y[i]);
        res.put(2, i, z[i]);
        res.put(i, 3, -center[i]);
    }
    res
}
