use crate::geometry::Matrix;
use crate::geometry::Vec3f;
use crate::image::Color;
use crate::image::Image;
use crate::model::Model;
use crate::render::Shader;

pub struct GouraudShader<'a> {
    model: &'a Model, 
    screen_transform: &'a Matrix,
    light: Vec3f,
    varying_intensity: Vec3f,
}

impl<'a> GouraudShader<'a> {
    pub fn new(m: &'a Model, screen_transform: &'a Matrix, light_dir: Vec3f) -> GouraudShader<'a> { 
        GouraudShader {
            model: m,
            screen_transform: screen_transform,
            light: light_dir,
            varying_intensity: Vec3f::new(0.0, 0.0, 0.0),
        }
    }
}

impl Shader for GouraudShader<'_> {
    fn vertex(&mut self, face_i: usize, vert_i: usize) -> Vec3f {
        let intensity = self.model.fnorm(face_i, vert_i)*self.light;
        self.varying_intensity[vert_i] = intensity.max(0.0); 
        let vert_m = Matrix::from_v(self.model.fvert(face_i, vert_i));
        let transformed = self.screen_transform*&vert_m;
        Vec3f::from_m(&transformed)
    }
    fn fragment(&mut self, bar: Vec3f, color: &mut Color) -> bool {
        let intensity = self.varying_intensity*bar;
        for i in 0..3 { color[i] = (255.0*intensity) as u8 }
        true // render fragment
    }
}
