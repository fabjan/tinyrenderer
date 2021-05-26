//! Some (well, one) example shader implementations.
use crate::geometry::Matrix;
use crate::geometry::Vec3f;
use crate::image::Color;
use crate::image::Image;
use crate::model::Model;
use crate::render::Shader;

/// A classic smooth shader!
/// https://en.wikipedia.org/wiki/Gouraud_shading
pub struct GouraudShader<'a> {
    model: &'a Model,
    screen_transform: &'a Matrix,
    diffuse_texture: &'a Image,
    light: Vec3f,
    varying_intensity: Vec3f,
    varying_uv: Matrix,
}

impl<'a> GouraudShader<'a> {
    pub fn new(
        model: &'a Model,
        screen_transform: &'a Matrix,
        diffuse_texture: &'a Image,
        light: Vec3f,
    ) -> GouraudShader<'a> {
        GouraudShader {
            model,
            screen_transform,
            diffuse_texture,
            light,
            varying_intensity: Vec3f::zero(),
            varying_uv: Matrix::new(2, 3),
        }
    }
}

impl Shader for GouraudShader<'_> {
    fn vertex(&mut self, face_i: usize, vert_i: usize) -> Vec3f {
        let intensity = self.model.fnorm(face_i, vert_i) * self.light;
        m_put_col(&mut self.varying_uv, vert_i, self.model.fuv(face_i, vert_i));
        self.varying_intensity[vert_i] = intensity.max(0.0);
        let vert_m = Matrix::from_v(self.model.fvert(face_i, vert_i));
        let transformed = self.screen_transform * &vert_m;
        Vec3f::from_m(&transformed)
    }
    fn fragment(&mut self, coords: Vec3f, color: &mut Color) -> bool {
        let intensity = self.varying_intensity * coords;
        let uv = m_mul_v(&self.varying_uv, coords);
        let mut diffuse = self.diffuse_texture.get_unit(uv.x, uv.y);
        for c in &mut diffuse {
            *c = (*c as f64 * intensity) as u8
        }
        *color = diffuse;
        true // render fragment
    }
}

fn m_put_col(m: &mut Matrix, c: usize, v: Vec3f) {
    m.put(0, c, v.x);
    m.put(1, c, v.y);
}

fn m_mul_v(m: &Matrix, v: Vec3f) -> Vec3f {
    Vec3f::new(
        Vec3f::new(m.get(0, 0), m.get(0, 1), m.get(0, 2)) * v,
        Vec3f::new(m.get(1, 0), m.get(1, 1), m.get(1, 2)) * v,
        0.0,
    )
}
