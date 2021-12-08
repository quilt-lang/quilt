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

#[cfg(test)]
mod test {
    use crate::vm::Direction;

    fn create_test_matrix() -> super::Matrix<usize> {
        let r1 = vec![1, 2, 3];
        let r2 = vec![4, 5, 6];
        let r3 = vec![7, 8, 9];
        let v = vec![r1, r2, r3];
        let m = super::Matrix::new(v);
        return m;
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
}
