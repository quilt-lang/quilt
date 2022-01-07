use crate::vm::Direction::{self, East, North, South, West};
use crate::Pixel;
use image::RgbaImage;
use std::ops::{Index, IndexMut};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct MatrixPoint(pub usize, pub usize);

impl MatrixPoint {
    pub fn neighbor(&self, direction: Direction) -> Option<Self> {
        match (direction, *self) {
            (North, Self(_, 0)) => None,
            (West, Self(0, _)) => None,
            (North, Self(x, y)) => Some(Self(x, y - 1)),
            (West, Self(x, y)) => Some(Self(x - 1, y)),
            (South, Self(x, y)) => Some(Self(x, y + 1)),
            (East, Self(x, y)) => Some(Self(x + 1, y)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Matrix<T> {
    pub matrix: Vec<Vec<T>>,
}

impl<T: Copy> Matrix<T> {
    pub fn new(matrix: Vec<Vec<T>>) -> Matrix<T> {
        Matrix { matrix }
    }

    pub fn get(&self, point: MatrixPoint) -> Option<T> {
        self.matrix.get(point.1)?.get(point.0).copied()
    }

    /// Tries to move a point in the provided direction
    /// If there is no cell in that direction, None is returned
    /// Otherwise Some(NewMatrixPoint) is returned
    pub fn go(&self, point: MatrixPoint, direction: Direction) -> Option<T> {
        point.neighbor(direction).and_then(|p| self.get(p))
    }

    pub fn corner(&self, point: MatrixPoint, dir1: Direction, dir2: Direction) -> Option<T> {
        point
            .neighbor(dir1)
            .and_then(|p| p.neighbor(dir2))
            .and_then(|p| self.get(p))
    }
}

impl<T: Copy> Index<MatrixPoint> for Matrix<T> {
    type Output = T;
    fn index(&self, index: MatrixPoint) -> &Self::Output {
        &self.matrix[index.1][index.0]
    }
}

impl<T: Copy> IndexMut<MatrixPoint> for Matrix<T> {
    fn index_mut(&mut self, index: MatrixPoint) -> &mut Self::Output {
        &mut self.matrix[index.1][index.0]
    }
}

impl From<&Matrix<Pixel>> for RgbaImage {
    fn from(matrix: &Matrix<Pixel>) -> Self {
        let height = matrix.matrix.len();
        let width = matrix.matrix.get(0).map_or(0, |row| row.len());
        let mut img = RgbaImage::new(width as u32, height as u32);
        for y in 0..height {
            for x in 0..width {
                let pixel = matrix[MatrixPoint(x, y)];
                img.put_pixel(x as u32, y as u32, pixel.hsl.into());
            }
        }
        img
    }
}

#[cfg(test)]
mod test {
    use crate::vm::Direction;

    fn create_test_matrix() -> super::Matrix<usize> {
        let r1 = vec![1, 2, 3];
        let r2 = vec![4, 5, 6];
        let r3 = vec![7, 8, 9];
        let v = vec![r1, r2, r3];
        super::Matrix::new(v)
    }

    #[test]
    fn test_go_boundaries() {
        let m = create_test_matrix();
        assert_eq!(m.go(super::MatrixPoint(0, 0), Direction::North), None);
        assert_eq!(m.go(super::MatrixPoint(0, 0), Direction::West), None);
        assert_eq!(m.go(super::MatrixPoint(2, 2), Direction::East), None);
        assert_eq!(m.go(super::MatrixPoint(2, 2), Direction::South), None);
    }

    #[test]
    fn test_go_happy() {
        let m = create_test_matrix();
        let p = super::MatrixPoint(1, 1);

        assert_eq!(m.go(p, Direction::North).unwrap(), 2);
        assert_eq!(m.go(p, Direction::West).unwrap(), 4);
        assert_eq!(m.go(p, Direction::East).unwrap(), 6);
        assert_eq!(m.go(p, Direction::South).unwrap(), 8);
    }

    #[test]
    fn test_index() {
        let mut m = create_test_matrix();
        let p = super::MatrixPoint(1, 1);
        assert_eq!(m[p], 5);

        m[p] = 1337;
        assert_eq!(m[p], 1337);
    }
}
