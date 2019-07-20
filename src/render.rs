use std::f64;
use std::mem;

use crate::geometry::Matrix;
use crate::geometry::Vec2f;
use crate::geometry::Vec3f;
use crate::image::Color;
use crate::image::Image;

pub fn line(canvas: &mut Image, x0: f64, y0: f64, x1: f64, y1: f64, color: Color) {
    let (mut x0, mut y0, mut x1, mut y1) = (x0, y0, x1, y1);
    let dx = (x1 as i32 - x0 as i32).abs();
    let dy = (y1 as i32 - y0 as i32).abs();
    let mut steep = false;
    if dx < dy {
        steep = true;
        mem::swap(&mut x0, &mut y0);
        mem::swap(&mut x1, &mut y1);
    }
    if x0 > x1 {
        mem::swap(&mut x0, &mut x1);
        mem::swap(&mut y0, &mut y1);
    }
    for x in (x0 as usize)..(x1 as usize) {
        let t: f64 = (x as f64 - x0 as f64) / ((x1 - x0) as f64);
        let y: f64 = y0 as f64 * (1. - t) + (y1 as f64 * t);
        if steep {
            canvas.put(y as usize, x, color);
        } else {
            canvas.put(x, y as usize, color);
        }
    }
}

pub fn line_v2(canvas: &mut Image, t0: Vec2f, t1: Vec2f, color: Color) {
    line(canvas, t0.x, t0.y, t1.x, t1.y, color);
}

pub fn triangle_wire(canvas: &mut Image, t0: Vec2f, t1: Vec2f, t2: Vec2f, color: Color) {
    if t0.y==t1.y && t0.y==t2.y { return };
    let (mut t0, mut t1, mut t2) = (t0, t1, t2);
    if t0.y>t1.y { mem::swap(&mut t0, &mut t1) };
    if t0.y>t2.y { mem::swap(&mut t0, &mut t2) };
    if t1.y>t2.y { mem::swap(&mut t1, &mut t2) };
    let total_height = (t2.y-t0.y) as usize;
    for i in 0..total_height {
        let second_half = i as f64 > t1.y-t0.y || t1.y == t0.y;
        let segment_height = if second_half { t2.y-t1.y } else { t1.y-t0.y };
        let alpha = i as f64 / total_height as f64;
        let beta = if second_half {
            (i as f64 - (t1.y-t0.y)) / segment_height
        } else {
            i as f64 / segment_height
        };
        let mut a = t0 + (t2-t0)*alpha;
        let mut b = if second_half { t1 + (t2-t1)*beta } else { t0 + (t1-t0)*beta };
        if a.x > b.x { mem::swap(&mut a, &mut b) }
        for j in (a.x as usize)..(b.x as usize + 1) {
            canvas.put(j, t0.y as usize + i, color);
        }
    }
}

pub fn triangle_flat(canvas: &mut Image, zbuffer: &mut Vec<f64>, t0: Vec3f, t1: Vec3f, t2: Vec3f, color: Color) {
    let (mut xmin, mut ymin) = (f64::MAX, f64::MAX);
    let (mut xmax, mut ymax) = (f64::MIN, f64::MIN);
    for t in [t0, t1, t2].iter() {
        xmin = xmin.min(t.x);
        xmax = xmax.max(t.x);
        ymin = ymin.min(t.y);
        ymax = ymax.max(t.y);
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
            let bc_screen = barycentric(t0, t1, t2, p);
            if bc_screen.x<0. || bc_screen.y<0. || bc_screen.z<0. { continue };
            p.z  = t0.z*bc_screen.x;
            p.z += t1.z*bc_screen.y;
            p.z += t2.z*bc_screen.z;
            let fragment_index = (p.x as usize) + (p.y as usize)*canvas.width;
            if zbuffer[fragment_index]<p.z {
                zbuffer[fragment_index] = p.z;
                canvas.put(p.x as usize, p.y as usize, color);
            }
        }
    }
}

pub fn triangle_diffuse(canvas: &mut Image, zbuffer: &mut Vec<f64>, t0: Vec3f, t1: Vec3f, t2: Vec3f, uv0: Vec3f, uv1: Vec3f, uv2: Vec3f, texture: &Image, intensity: f64) {
    let (mut xmin, mut ymin) = (f64::MAX, f64::MAX);
    let (mut xmax, mut ymax) = (f64::MIN, f64::MIN);
    for t in [t0, t1, t2].iter() {
        xmin = xmin.min(t.x);
        xmax = xmax.max(t.x);
        ymin = ymin.min(t.y);
        ymax = ymax.max(t.y);
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
            let bc_screen = barycentric(t0, t1, t2, p);
            if bc_screen.x<0. || bc_screen.y<0. || bc_screen.z<0. { continue };
            p.z  = t0.z*bc_screen.x;
            p.z += t1.z*bc_screen.y;
            p.z += t2.z*bc_screen.z;
            let fragment_index = (p.x as usize) + (p.y as usize)*canvas.width;
            if zbuffer[fragment_index]<p.z {
                zbuffer[fragment_index] = p.z;
                let uv = uv0*bc_screen.x + uv1*bc_screen.y + uv2*bc_screen.z;
                let mut color = texture.get_unit(uv.x, uv.y);
                for i in 0..3 { color[i] = (color[i] as f64 * intensity) as u8 }
                canvas.put(p.x as usize, p.y as usize, color);
            }
        }
    }
}

pub fn triangle_diffuse_trace(canvas: &mut Image, zbuffer: &mut Vec<f64>, t0: Vec3f, t1: Vec3f, t2: Vec3f, uv0: Vec3f, uv1: Vec3f, uv2: Vec3f, texture: &Image, intensity: f64) {
    if t0.y==t1.y && t0.y==t2.y { return };
    let (mut t0, mut t1, mut t2) = (t0, t1, t2);
    let (mut uv0, mut uv1, mut uv2) = (uv0, uv1, uv2);
    if t0.y>t1.y { mem::swap(&mut t0, &mut t1); mem::swap(&mut uv0, &mut uv1) };
    if t0.y>t2.y { mem::swap(&mut t0, &mut t2); mem::swap(&mut uv0, &mut uv2) };
    if t1.y>t2.y { mem::swap(&mut t1, &mut t2); mem::swap(&mut uv1, &mut uv2) };
    let total_height = (t2.y-t0.y) as usize;
    for i in 0..total_height {
        let second_half = i as f64 > t1.y-t0.y || t1.y == t0.y;
        let segment_height = if second_half { t2.y-t1.y } else { t1.y-t0.y };
        let alpha = i as f64 / total_height as f64;
        let beta = if second_half {
            (i as f64 - (t1.y-t0.y)) / segment_height
        } else {
            i as f64 / segment_height
        };
        let mut a   = t0 + (t2-t0)*alpha;
        let mut b   = if second_half { t1 + (t2-t1)*beta } else { t0 + (t1-t0)*beta };
        let mut uva = uv0 + (uv2-uv0)*alpha;
        let mut uvb = if second_half { uv1 + (uv2-uv1)*beta } else { uv0 + (uv1-uv0)*beta };
        if a.x > b.x { mem::swap(&mut a, &mut b); mem::swap(&mut uva, &mut uvb) }
        let (x0, x1) = (a.x as usize, b.x as usize);
        for j in x0..(x1+1) {
            let phi = if x0==x1 { 1.0 } else { (j as f64 - x0 as f64)/(x1 as f64 - x0 as f64) };
            let p = a + (b-a)*phi;
            let fragment_index = (p.x as usize) + (p.y as usize)*canvas.width;
            if zbuffer[fragment_index]<p.z {
                zbuffer[fragment_index] = p.z;
                let uvp = uva + (uvb-uva)*phi;
                let color = texture.get_unit(uvp.x, uvp.y);
                let color = [ (color[0] as f64 * intensity) as u8, (color[1] as f64 * intensity) as u8, (color[2] as f64 * intensity) as u8 ];
                canvas.put(p.x as usize, p.y as usize, color);
            }
        }
    }
}

pub fn to_screen_coords(world_coords: Vec3f, translate: Vec2f, scale: Vec2f) -> Vec3f {
    Vec3f {
        x: (world_coords.x+translate.x) * scale.x,
        y: (world_coords.y+translate.y) * scale.y,
        z: world_coords.z,
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
