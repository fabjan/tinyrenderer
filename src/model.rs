use std::io::BufRead;

use crate::geometry::Vec3f;
use crate::geometry::Vec3i;

pub struct Face {
    pub verts: Vec3i,
    pub uvs: Vec3i,
    pub norms: Vec3i,
}

pub struct Model {
    verts: Vec<Vec3f>,
    uvs: Vec<Vec3f>,
    norms: Vec<Vec3f>,
    faces: Vec<Face>,
}

impl Model {
    pub fn from_obj<R: BufRead>(obj_data: R) -> Model {
        let mut obj_verts = Vec::new();
        let mut obj_uvs = Vec::new();
        let mut obj_norms = Vec::new();
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
                Some("vt") => {
                    tokens.next(); // skip an extra space
                    let (x, y, z) = (
                        tokens.next().unwrap().parse().unwrap(),
                        tokens.next().unwrap().parse().unwrap(),
                        tokens.next().unwrap().parse().unwrap(),
                    );
                    obj_uvs.push(Vec3f {x, y, z})
                }
                Some("vn") => {
                    tokens.next(); // skip an extra space
                    let (x, y, z) = (
                        tokens.next().unwrap().parse().unwrap(),
                        tokens.next().unwrap().parse().unwrap(),
                        tokens.next().unwrap().parse().unwrap(),
                    );
                    obj_norms.push(Vec3f {x, y, z})
                }
                Some("f") => {
                    let (mut a, mut b, mut c) = (
                        tokens.next().unwrap().split('/'),
                        tokens.next().unwrap().split('/'),
                        tokens.next().unwrap().split('/'),
                    );
                    let (v0, uv0, n0): (i32, i32, i32) = (
                        a.next().unwrap().parse().unwrap(),
                        a.next().unwrap().parse().unwrap(),
                        a.next().unwrap().parse().unwrap(),
                    );
                    let (v1, uv1, n1): (i32, i32, i32) = (
                        b.next().unwrap().parse().unwrap(),
                        b.next().unwrap().parse().unwrap(),
                        b.next().unwrap().parse().unwrap(),
                    );
                    let (v2, uv2, n2): (i32, i32, i32) = (
                        c.next().unwrap().parse().unwrap(),
                        c.next().unwrap().parse().unwrap(),
                        c.next().unwrap().parse().unwrap(),
                    );
                    // in wavefront obj all indices start at 1, not zero
                    let face = Face {
                        verts: Vec3i { x: v0-1, y: v1-1, z: v2-1 },
                        uvs: Vec3i { x: uv0-1, y: uv1-1, z: uv2-1 },
                        norms: Vec3i { x: n0-1, y: n1-1, z: n2-1},
                    };
                    obj_faces.push(face);
                }
                _ => continue
            }
        }
        Model {
            verts: obj_verts,
            uvs: obj_uvs,
            norms: obj_norms,
            faces: obj_faces,
        }
    }
    pub fn vert(&self, i: usize) -> Vec3f {
        self.verts[i]
    }
    pub fn face(&self, i: usize) -> &Face {
        &self.faces[i]
    }
    pub fn faces(&self) -> std::slice::Iter<Face> {
        self.faces.iter()
    }
    pub fn uv(&self, i: usize) -> Vec3f {
        self.uvs[i]
    }
    pub fn norm(&self, i: usize) -> Vec3f {
        self.norms[i]
    }
}
