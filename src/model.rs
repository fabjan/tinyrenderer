//! Modeling 3D objects, and reading [Wavefront .obj] files.
//!
//! [Wavefront .obj]: https://en.wikipedia.org/wiki/Wavefront_.obj_file
use std::io::BufRead;

use crate::geometry::Vec3;
use crate::geometry::Vec3f;

/// A face references vertices, texture uv coords, and normals by index
pub struct Face {
    pub verts: Vec3<usize>,
    pub uvs: Vec3<usize>,
    pub norms: Vec3<usize>,
}

/// A model has a bunch of faces (triangles), see Face for what data they contain.
/// The face data lives in the model, since multiple faces share data.
pub struct Model {
    verts: Vec<Vec3f>,
    uvs: Vec<Vec3f>,
    norms: Vec<Vec3f>,
    faces: Vec<Face>,
}

impl Model {
    /// Parse the given data as a Waveform .obj into a model
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
                    obj_verts.push(Vec3f { x, y, z });
                }
                Some("vt") => {
                    tokens.next(); // skip an extra space
                    let (x, y, z) = (
                        tokens.next().unwrap().parse().unwrap(),
                        tokens.next().unwrap().parse().unwrap(),
                        tokens.next().unwrap().parse().unwrap(),
                    );
                    obj_uvs.push(Vec3f { x, y, z })
                }
                Some("vn") => {
                    tokens.next(); // skip an extra space
                    let (x, y, z) = (
                        tokens.next().unwrap().parse().unwrap(),
                        tokens.next().unwrap().parse().unwrap(),
                        tokens.next().unwrap().parse().unwrap(),
                    );
                    obj_norms.push(Vec3f { x, y, z })
                }
                Some("f") => {
                    let (mut a, mut b, mut c) = (
                        tokens.next().unwrap().split('/'),
                        tokens.next().unwrap().split('/'),
                        tokens.next().unwrap().split('/'),
                    );
                    let (v0, uv0, n0): (usize, usize, usize) = (
                        a.next().unwrap().parse().unwrap(),
                        a.next().unwrap().parse().unwrap(),
                        a.next().unwrap().parse().unwrap(),
                    );
                    let (v1, uv1, n1): (usize, usize, usize) = (
                        b.next().unwrap().parse().unwrap(),
                        b.next().unwrap().parse().unwrap(),
                        b.next().unwrap().parse().unwrap(),
                    );
                    let (v2, uv2, n2): (usize, usize, usize) = (
                        c.next().unwrap().parse().unwrap(),
                        c.next().unwrap().parse().unwrap(),
                        c.next().unwrap().parse().unwrap(),
                    );
                    // in wavefront obj all indices start at 1, not zero
                    let face = Face {
                        verts: Vec3::new(v0 - 1, v1 - 1, v2 - 1),
                        uvs: Vec3::new(uv0 - 1, uv1 - 1, uv2 - 1),
                        norms: Vec3::new(n0 - 1, n1 - 1, n2 - 1),
                    };
                    obj_faces.push(face);
                }
                _ => continue,
            }
        }
        Model {
            verts: obj_verts,
            uvs: obj_uvs,
            norms: obj_norms,
            faces: obj_faces,
        }
    }

    /// get vertex by index
    pub fn vert(&self, i: usize) -> Vec3f {
        self.verts[i]
    }

    /// get the `v`th vertex of the face `f`
    pub fn fvert(&self, f: usize, v: usize) -> Vec3f {
        self.verts[self.faces[f].verts[v]]
    }

    /// how many faces are there?
    pub fn nfaces(&self) -> usize {
        self.faces.len()
    }

    /// get face by index
    pub fn face(&self, i: usize) -> &Face {
        &self.faces[i]
    }

    pub fn faces(&self) -> std::slice::Iter<Face> {
        self.faces.iter()
    }

    /// get uv coords by index
    pub fn uv(&self, i: usize) -> Vec3f {
        self.uvs[i]
    }

    /// get the `v`th uv coord of the face `f`
    pub fn fuv(&self, f: usize, v: usize) -> Vec3f {
        self.uvs[self.faces[f].uvs[v]]
    }

    /// get a normal by index
    pub fn norm(&self, i: usize) -> Vec3f {
        self.norms[i]
    }

    /// get the `v`th normal of the face `f`
    pub fn fnorm(&self, f: usize, v: usize) -> Vec3f {
        self.norms[self.faces[f].norms[v]]
    }
}
