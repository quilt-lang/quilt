pub struct MatrixPoint(pub usize, pub usize);

#[derive(Debug)]
pub struct Matrix<T> {
    pub matrix: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn new(matrix: Vec<Vec<T>>) -> Matrix<T> {
        Matrix { matrix }
    }

    pub fn cell_exists(&self, point: &MatrixPoint) -> bool {
        let x = point.0;
        let y = point.1;

        if y < 0 || y >= self.matrix.len() {
            return false;
        }

        if let Some(row) = self.matrix.get(0) {
            if x < 0 || x >= row.len() {
                false
            } else {
                true
            }
        } else {
            false
        }
    }
}
