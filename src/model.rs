// Copyright 2021 Fabian Bergström
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//!
//! Modeling 3D objects, and reading [Wavefront .obj] files.
//!
//! [Wavefront .obj]: https://en.wikipedia.org/wiki/Wavefront_.obj_file
//!

use std::io::BufRead;
use std::str::FromStr;

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
            let line = line.expect("line cannot be empty");
            let tokens = &mut line.split_whitespace();
            match tokens.next() {
                Some("v") => {
                    let (x, y, z) = parse_triplet(tokens);
                    obj_verts.push(Vec3f { x, y, z });
                }
                Some("vt") => {
                    // not all obj files have 3 dimensions for the uv coords
                    let (x, y, z) = (
                        parse_next(tokens),
                        parse_next(tokens),
                        parse_next_or(tokens, 0.),
                    );
                    obj_uvs.push(Vec3f { x, y, z });
                }
                Some("vn") => {
                    let (x, y, z) = parse_triplet(tokens);
                    obj_norms.push(Vec3f { x, y, z });
                }
                Some("f") => {
                    let (mut a, mut b, mut c) = (
                        tokens.next().expect("face missing v0").split('/'),
                        tokens.next().expect("face missing v1").split('/'),
                        tokens.next().expect("face missing v2").split('/'),
                    );

                    let (v0, uv0, n0): (usize, usize, usize) = parse_triplet(&mut a);
                    let (v1, uv1, n1): (usize, usize, usize) = parse_triplet(&mut b);
                    let (v2, uv2, n2): (usize, usize, usize) = parse_triplet(&mut c);

                    // in wavefront obj indices start at 1, not 0
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

fn parse_triplet<T>(tokens: &mut dyn Iterator<Item = &str>) -> (T, T, T)
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    (parse_next(tokens), parse_next(tokens), parse_next(tokens))
}

fn parse_next<T: FromStr>(tokens: &mut dyn Iterator<Item = &str>) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    tokens
        .next()
        .expect("not enough tokens")
        .parse()
        .expect("cannot parse token")
}

fn parse_next_or<T: FromStr>(tokens: &mut dyn Iterator<Item = &str>, default: T) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    tokens
        .next()
        .map(|t| t.parse().expect("cannot parse token"))
        .unwrap_or(default)
}
