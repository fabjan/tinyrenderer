use std::fmt;
use std::ops;

#[derive(Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

pub type Vec2f = Vec2<f64>;
pub type Vec2i = Vec2<i32>;

impl Vec2f {
    pub fn zero() -> Vec2f {
        Vec2f { x: 0., y: 0. }
    }
}

impl<T: ops::Add<T, Output = T>> ops::Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 { x: self.x+other.x, y: self.y+other.y }
    }
}

impl<T: ops::Sub<T, Output = T>> ops::Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn sub(self, other: Vec2<T>) -> Vec2<T> {
        Vec2 { x: self.x-other.x, y: self.y-other.y }
    }
}

impl<T: Copy + ops::Mul<T, Output = T>> ops::Mul<T> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, s: T) -> Vec2<T> {
        Vec2 { x: self.x*s, y: self.y*s }
    }
}

#[derive(Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vec3f = Vec3<f64>;
pub type Vec3i = Vec3<i32>;

impl Vec3f {
    pub fn zero() -> Vec3f {
        Vec3f { x: 0., y: 0., z: 0. }
    }
}

impl<T: fmt::Display> fmt::Display for Vec3<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{} {} {}]", self.x, self.y, self.z)
    }
}

impl<T: ops::Add<T, Output = T>> ops::Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn add(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 { x: self.x+other.x, y: self.y+other.y, z: self.z+other.z }
    }
}

impl<T: ops::Sub<T, Output = T>> ops::Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;
    fn sub(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 { x: self.x-other.x, y: self.y-other.y, z: self.z-other.z }
    }
}

impl<T: Copy + ops::Mul<T, Output = T>> ops::Mul<T> for Vec3<T> {
    type Output = Vec3<T>;
    fn mul(self, s: T) -> Vec3<T> {
        Vec3 { x: self.x*s, y: self.y*s, z: self.z*s }
    }
}

impl<T: ops::Mul<T, Output = T> + ops::Add<T, Output = T>> ops::Mul<Vec3<T>> for Vec3<T> {
    type Output = T;
    fn mul(self, other: Vec3<T>) -> T {
        self.x*other.x + self.y*other.y + self.z*other.z
    }
}

impl Vec3f {
    pub fn norm(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    pub fn normalize(&mut self) {
        *self = *self*(1./self.norm());
    }
    pub fn normalized(&self) -> Vec3f {
        *self*(1./self.norm())
    }
}

impl<T: Copy + ops::Mul<T, Output = T> + ops::Sub<T, Output = T>> Vec3<T> {
    pub fn cross(&self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.y*other.z - self.z*other.y,
            y: self.z*other.x - self.x*other.z,
            z: self.x*other.y - self.y*other.x,
        }
    }
}

pub struct Matrix {
    m: Vec<Vec<f64>>,
    pub rows: usize,
    pub cols: usize,
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.rows {
            write!(f, "\n|").unwrap();
            for j in 0..self.cols {
                write!(f, " {}", self.get(i, j)).unwrap();
            }
            write!(f, "|").unwrap();
        }
        Ok(())
    }
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Matrix {
        let col = vec![0.0;cols];
        let m = vec![col;rows];
        Matrix {
            m: m,
            rows: rows,
            cols: cols,
        }
    }
    pub fn identity(dimensions: usize) -> Matrix {
        let mut result = Matrix::new(dimensions, dimensions);
        for i in 0..dimensions {
            for j in 0..dimensions {
                result.put(i, j, if i==j { 1.0 } else { 0.0 });
            }
        }
        result
    }
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.m[row][col]
    }
    pub fn put(&mut self, row: usize, col: usize, v: f64) {
        self.m[row][col] = v;
    }
}

impl ops::Mul<&Matrix> for &Matrix {
    type Output = Matrix;
    fn mul(self, other: &Matrix) -> Matrix {
        assert!(self.cols == other.rows);
        let mut result = Matrix::new(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                result.put(i, j, 0.0);
                for k in 0..self.cols {
                    result.put(i, j, result.get(i, j)+self.get(i, k)*other.get(k, j));
                }
            }
        }
        result
    }
}

impl ops::Mul<&Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, other: &Matrix) -> Matrix {
        &self*other
    }
}
