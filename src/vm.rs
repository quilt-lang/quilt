use crate::{Instruction, Pixel};
use crate::{Matrix, MatrixPoint};

const TAPE_SIZE: usize = 360;

pub struct VM {
    stack: Vec<u16>,
    register_a: u16,
    tape: [u16; TAPE_SIZE],
    direction: Direction,
    instructions: Matrix<Pixel>,
    pc: MatrixPoint,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
        }
    }

    pub fn counter_clockwise(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        }
    }
}

impl Default for VM {
    fn default() -> VM {
        VM {
            stack: vec![],
            register_a: 0,
            tape: [0; TAPE_SIZE],
            direction: Direction::East,
            instructions: Matrix::new(vec![]),
            pc: MatrixPoint(0, 0),
        }
    }
}

impl VM {
    pub fn new() -> VM {
        VM::default()
    }

    pub fn execute(&mut self, instructions: Matrix<Pixel>) {
        self.instructions = instructions;
        self.pc = self.find_start();

        //loop { // TODO change condition
        //}
    }

    // prioritize roads over all other instructions besides the one in front of us
    pub fn get_next_instruction(&self) -> Pixel {
        let next_pixels = self.get_next_pixels();
        let (first_dir, first_pixel) = next_pixels[0];
        let first_road = next_pixels
            .iter()
            .filter(|(_dir, pixel)| matches!(pixel.as_instruction(), Instruction::Road))
            .next();
        if let Some((dir, road)) = first_road {
            if *dir != self.direction.opposite() {
                return *road;
            } else if first_dir == self.direction {
                return first_pixel;
            }
        }
        // We bounce
        next_pixels.last().unwrap().1
    }

    // try the pixel ahead of us. If that doesn't exist,
    // try the pixel to the 'right' (counter-clockwise & opposite). If that doesn't exist,
    // try the pixel to the 'left' (counter-clockwise). If that doesn't exist,
    // go back the way we came
    fn get_next_pixels(&self) -> Vec<(Direction, Pixel)> {
        let ins = &self.instructions;
        let dir = self.direction;
        let mut next_pixels = vec![];
        if let Some(point) = ins.go(self.pc, dir) {
            // forward
            next_pixels.push((dir, point));
        }
        if let Some(point) = ins.go(self.pc, dir.counter_clockwise().opposite()) {
            // right
            next_pixels.push((dir.counter_clockwise().opposite(), point));
        }
        if let Some(point) = ins.go(self.pc, dir.counter_clockwise()) {
            // left
            next_pixels.push((dir.counter_clockwise(), point));
        }
        if let Some(point) = ins.go(self.pc, dir.opposite()) {
            // back
            next_pixels.push((dir.opposite(), point));
        }
        next_pixels
    }

    fn get_next_pixel(&self) -> (Direction, Pixel) {
        self.get_next_pixels()[0]
    }

    fn find_start(&self) -> MatrixPoint {
        for (row_idx, row) in self.instructions.matrix.iter().enumerate() {
            for (col_idx, pixel) in row.iter().enumerate() {
                if pixel.as_instruction() == Instruction::Start {
                    return MatrixPoint(col_idx, row_idx);
                }
            }
        }

        // default start coordinates
        MatrixPoint(0, 0)
    }
}

#[cfg(test)]
mod test {
    use super::{Direction, VM};
    use crate::pixel::START;
    use crate::vm::Direction::{East, North, South, West};
    use crate::{Matrix, MatrixPoint, Pixel};

    fn init_vm(matrix: Vec<Vec<u16>>) -> VM {
        let mut vm = VM::new();
        vm.instructions = init_matrix(matrix);
        vm
    }

    fn init_matrix(pixels: Vec<Vec<u16>>) -> Matrix<Pixel> {
        Matrix::new(
            pixels
                .iter()
                .map(|row| row.iter().map(|p| Pixel::new(*p)).collect())
                .collect(),
        )
    }

    #[test]
    fn test_start_one_d() {
        let vm = init_vm(vec![vec![
            START, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306,
        ]]);

        let start = vm.find_start();

        assert_eq!(start, MatrixPoint(0, 0));
    }

    #[test]
    fn test_start_two_d() {
        let vm = init_vm(vec![
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
            vec![0, 180, 180, 36, 1, START, 2, 108, 36, 48, 108, 306],
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        let start = vm.find_start();

        assert_eq!(start, MatrixPoint(5, 1));
    }

    #[test]
    fn test_start_bounds() {
        let vm = init_vm(vec![
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
            vec![0, 180, 180, 36, 1, 3, 2, 108, 36, 48, 108, 306],
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, START],
        ]);

        let start = vm.find_start();

        assert_eq!(start, MatrixPoint(11, 2));
    }

    fn compare_pixels(actual: Vec<(Direction, Pixel)>, expected: Vec<(Direction, u16)>) {
        assert_eq!(actual.len(), expected.len());

        for (idx, (dir, pixel)) in actual.iter().enumerate() {
            let (expected_dir, expected_pixel) = expected[idx];
            assert_eq!(*dir, expected_dir);
            assert_eq!(pixel.value, expected_pixel);
        }
    }

    #[test]
    fn test_get_next_pixels_east() {
        let mut vm = init_vm(vec![vec![
            START, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306,
        ]]);

        vm.pc = vm.find_start();
        let actual = vm.get_next_pixels();
        let expected = vec![(East, 180)];
        compare_pixels(actual, expected);
    }

    #[test]
    fn test_get_next_pixels_east_middle() {
        let mut vm = init_vm(vec![
            vec![0, 180, 180, 36, 1, 37, 2, 108, 36, 48, 108, 306],
            vec![0, 180, 180, 36, 1, START, 2, 108, 36, 48, 108, 306],
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        vm.pc = vm.find_start();
        assert_eq!(vm.pc, MatrixPoint(5, 1));
        let pixels = vm.get_next_pixels();

        let expected = vec![(East, 2), (South, 36), (North, 37), (West, 1)];

        compare_pixels(pixels, expected);
    }

    #[test]
    fn test_get_next_pixels_east_bound() {
        let mut vm = init_vm(vec![
            vec![0, 180, 180, 36, 1, 37, 2, 108, 36, 48, 108, 310],
            vec![0, 180, 180, 36, 1, 108, 2, 108, 36, 48, 108, START],
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        vm.pc = vm.find_start();
        assert_eq!(vm.pc, MatrixPoint(11, 1));
        let actual = vm.get_next_pixels();

        let expected = vec![(South, 306), (North, 310), (West, 108)];

        compare_pixels(actual, expected);
    }

    #[test]
    fn test_get_next_pixels_north() {
        let mut vm = init_vm(vec![
            vec![0, 180, 180, 36, 1, 37, 2, 108, 36, 48, 108, 310],
            vec![0, 180, 180, 36, 1, 108, 2, 108, 36, START, 108, 314],
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        vm.direction = North;

        vm.pc = vm.find_start();
        assert_eq!(vm.pc, MatrixPoint(9, 1));
        let actual = vm.get_next_pixels();

        let expected = vec![(North, 48), (East, 108), (West, 36), (South, 48)];

        compare_pixels(actual, expected);
    }

    #[test]
    fn test_get_next_pixels_north_bound() {
        let mut vm = init_vm(vec![
            vec![0, 180, 180, 36, 1, 37, 2, 108, START, 48, 108, 310],
            vec![0, 180, 180, 36, 1, 18, 2, 108, 36, 42, 108, 314],
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        vm.direction = North;

        vm.pc = vm.find_start();
        assert_eq!(vm.pc, MatrixPoint(8, 0));
        let actual = vm.get_next_pixels();

        let expected = vec![(East, 48), (West, 108), (South, 36)];

        compare_pixels(actual, expected);
    }

    #[test]
    fn test_get_next_pixels_west() {
        let mut vm = init_vm(vec![
            vec![0, 180, 180, 36, 1, 37, 2, 108, 70, 48, 108, 310],
            vec![0, 180, START, 36, 1, 18, 2, 108, 36, 42, 108, 314],
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        vm.direction = West;

        vm.pc = vm.find_start();
        assert_eq!(vm.pc, MatrixPoint(2, 1));
        let actual = vm.get_next_pixels();

        let expected = vec![(West, 180), (North, 180), (South, 180), (East, 36)];

        compare_pixels(actual, expected);
    }

    #[test]
    fn test_get_next_pixels_south_bound() {
        let mut vm = init_vm(vec![
            vec![0, 180, 180, 36, 1, 37, 2, 108, 70, 48, 108, 310],
            vec![0, 180, 180, 36, 1, 18, 2, 108, 36, 42, 108, 314],
            vec![START, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        vm.direction = South;

        vm.pc = vm.find_start();
        assert_eq!(vm.pc, MatrixPoint(0, 2));
        let actual = vm.get_next_pixels();

        let expected = vec![(East, 180), (North, 0)];

        compare_pixels(actual, expected);
    }

    #[test]
    fn test_get_next_pixels_south() {
        let mut vm = init_vm(vec![
            vec![0, 180, START, 36, 1, 37, 2, 108, 70, 48, 108, 310],
            vec![0, 180, 100, 36, 1, 18, 2, 108, 36, 42, 108, 314],
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        vm.direction = South;

        vm.pc = vm.find_start();
        assert_eq!(vm.pc, MatrixPoint(2, 0));
        let actual = vm.get_next_pixels();

        let expected = vec![(South, 100), (West, 180), (East, 36)];

        compare_pixels(actual, expected);
    }

    #[test]
    fn test_get_next_pixels_west_bound() {
        let mut vm = init_vm(vec![
            vec![0, 180, 180, 36, 1, 37, 2, 108, 70, 48, 108, 310],
            vec![START, 180, 180, 36, 1, 18, 2, 108, 36, 42, 108, 314],
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        vm.direction = West;

        vm.pc = vm.find_start();
        assert_eq!(vm.pc, MatrixPoint(0, 1));
        let actual = vm.get_next_pixels();

        let expected = vec![(North, 0), (South, 0), (East, 180)];

        compare_pixels(actual, expected);
    }
}
