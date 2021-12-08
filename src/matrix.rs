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
        let MatrixPoint(x, y) = point;

        if *y >= self.matrix.len() {
            return false;
        }

        if let Some(row) = self.matrix.get(0) {
            if *x >= row.len() {
                false
            } else {
                true
            }
        } else {
            false
        }
    }
}
