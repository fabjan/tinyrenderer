use std::io::BufRead;

use crate::geometry::Vec3f;
use crate::geometry::Vec3i;

pub struct Model {
    verts: Vec<Vec3f>,
    faces: Vec<Vec3i>,
}

impl Model {
    pub fn from_obj<R: BufRead>(obj_data: R) -> Model {
        let mut obj_verts = Vec::new();
        let mut obj_faces = Vec::new();
        for line in obj_data.lines() {
            let line = line.unwrap();
            let mut tokens = line.split(' ');
            match tokens.next() {
                Some("v") => {
                    let (x, y, z) = (
                        tokens.next().unwrap().parse().unwrap(),
                        tokens.next().unwrap().parse().unwrap(),
                        tokens.next().unwrap().parse().unwrap(),
                    );
                    obj_verts.push(Vec3f {x, y, z});
                }
                Some("f") => {
                    let (a, b, c): (i32, i32, i32) = (
                        tokens.next().unwrap().split('/').next().unwrap().parse().unwrap(),
                        tokens.next().unwrap().split('/').next().unwrap().parse().unwrap(),
                        tokens.next().unwrap().split('/').next().unwrap().parse().unwrap(),
                    );
                    // in wavefront obj all indices start at 1, not zero
                    obj_faces.push(Vec3i {x: a-1, y: b-1, z: c-1});
                }
                _ => continue
            }
        }
        Model {
            verts: obj_verts,
            faces: obj_faces,
        }
    }
    pub fn nverts(&self) -> usize {
        self.verts.len()
    }
    pub fn nfaces(&self) -> usize {
        self.faces.len()
    }
    pub fn vert(&self, i: usize) -> Vec3f {
        self.verts[i].clone()
    }
    pub fn face(&self, i: usize) -> Vec3i {
        self.faces[i].clone()
    }
    pub fn faces(&self) -> std::slice::Iter<Vec3i> {
        self.faces.iter()
    }
}
