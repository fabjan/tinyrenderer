//! Rasterize triangles, with vertex and fragment shaders.
use std::f64;

use crate::geometry::Matrix;
use crate::geometry::Vec3f;
use crate::image::Color;
use crate::image::Image;

/// A shader can change vertices and fragments (pixels).
pub trait Shader {
    /// Where should this vertex be?
    fn vertex(&mut self, face_i: usize, vert_i: usize) -> Vec3f; // let's hope float is fine
    /// What is the color the fragment at `bar`?  
    /// Returns true iff the fragment should be rendered.
    fn fragment(&mut self, bar: Vec3f, color: &mut Color) -> bool;
}

/// Render a triangle v0,v1,v2 to the given canvas.
pub fn triangle(
    canvas: &mut Image,
    zbuffer: &mut Vec<f64>,
    shader: &mut dyn Shader,
    v0: Vec3f,
    v1: Vec3f,
    v2: Vec3f,
) {
    // figure out the bounding box
    let (mut xmin, mut ymin) = (f64::MAX, f64::MAX);
    let (mut xmax, mut ymax) = (f64::MIN, f64::MIN);
    for v in [v0, v1, v2].iter() {
        xmin = xmin.min(v.x);
        xmax = xmax.max(v.x);
        ymin = ymin.min(v.y);
        ymax = ymax.max(v.y);
    }

    let xmin = (xmin as usize).max(0);
    let xmax = (xmax as usize).min(canvas.width);
    let ymin = (ymin as usize).max(0);
    let ymax = (ymax as usize).min(canvas.height);

    // for every pixel p inside the box ...
    let mut p = Vec3f::zero();
    for x in xmin..=xmax {
        p.x = x as f64;
        for y in ymin..=ymax {
            p.y = y as f64;

            // ... calculate the barycentric coordinates for p
            let bc_screen = barycentric(v0, v1, v2, p);
            // if any part of the coordinate is negative, p is outside the triangle
            if bc_screen.x < 0. || bc_screen.y < 0. || bc_screen.z < 0. {
                continue;
            };
            p.z = v0.z * bc_screen.x;
            p.z += v1.z * bc_screen.y;
            p.z += v2.z * bc_screen.z;

            // don't draw fragments behind something we have already drawn
            let fragment_index = (p.x as usize) + (p.y as usize) * canvas.width;
            if zbuffer[fragment_index] < p.z {
                zbuffer[fragment_index] = p.z;

                // apply fragment shader
                let mut color = [0, 0, 0];
                let keep_fragment = shader.fragment(bc_screen, &mut color);
                // does this really play right with the z-index check before?
                if keep_fragment {
                    canvas.put(p.x as usize, p.y as usize, color);
                }
            }
        }
    }
}

/// The [barycentric coordinates] for p in the triangle a,b,c.
/// [barycentric coordinates]: https://en.wikipedia.org/wiki/Barycentric_coordinate_system
#[allow(clippy::many_single_char_names)]
fn barycentric(a: Vec3f, b: Vec3f, c: Vec3f, p: Vec3f) -> Vec3f {
    let xs = Vec3f {
        x: c.x - a.x,
        y: b.x - a.x,
        z: a.x - p.x,
    };
    let ys = Vec3f {
        x: c.y - a.y,
        y: b.y - a.y,
        z: a.y - p.y,
    };
    let u = xs.cross(ys);
    // TODO hoist this magic number up as a constant
    if u.z.abs() > 0.01 {
        Vec3f {
            x: 1. - (u.x + u.y) / u.z,
            y: u.y / u.z,
            z: u.x / u.z,
        }
    } else {
        Vec3f {
            x: -1.,
            y: 1.,
            z: 1.,
        }
    }
}

/// Create a view port centered at x, y, depth, with the given width and height.
pub fn viewport(x: f64, y: f64, w: f64, h: f64, depth: f64) -> Matrix {
    let mut vp = Matrix::identity(4);

    vp.put(0, 3, x + w / 2.0);
    vp.put(1, 3, y + h / 2.0);
    vp.put(2, 3, depth / 2.0);

    vp.put(0, 0, w / 2.0);
    vp.put(1, 1, h / 2.0);
    vp.put(2, 2, depth / 2.0);

    vp
}

/// Create a perspective projection matrix.
pub fn projection(coeff: f64) -> Matrix {
    let mut projection = Matrix::identity(4);
    projection.put(3, 2, coeff);
    projection
}

/// Create a matrix representing a camera with the given orientation.
pub fn lookat(eye: Vec3f, center: Vec3f, up: Vec3f) -> Matrix {
    let z = (eye - center).normalized();
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
