use crate::vm::Direction;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MatrixPoint(pub usize, pub usize);

#[derive(Debug)]
pub struct Matrix<T> {
    pub matrix: Vec<Vec<T>>,
}

impl<T: Copy> Matrix<T> {
    pub fn new(matrix: Vec<Vec<T>>) -> Matrix<T> {
        Matrix { matrix }
    }

    pub fn cell_exists(&self, point: MatrixPoint) -> bool {
        let MatrixPoint(x, y) = point;

        if y >= self.matrix.len() {
            return false;
        }

        if let Some(row) = self.matrix.get(0) {
            if x >= row.len() {
                false
            } else {
                true
            }
        } else {
            false
        }
    }

    pub fn get(&self, point: MatrixPoint) -> Option<T> {
        if self.cell_exists(point) {
            Some(self.matrix[point.1][point.0])
        } else {
            None
        }
    }

    /// Tries to move a point in the provided direction
    /// If there is no cell in that direction, None is returned
    /// Otherwise Some(NewMatrixPoint) is returned
    pub fn go(&self, point: MatrixPoint, direction: Direction) -> Option<T> {
        let MatrixPoint(x, y) = point;

        match direction {
            Direction::North => {
                if y == 0 {
                    None
                } else {
                    self.get(MatrixPoint(x, y - 1))
                }
            }
            Direction::West => {
                if x == 0 {
                    None
                } else {
                    self.get(MatrixPoint(x - 1, y))
                }
            }
            Direction::South => self.get(MatrixPoint(x, y + 1)),
            Direction::East => self.get(MatrixPoint(x + 1, y)),
        }
    }
}
