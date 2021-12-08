use anyhow::{anyhow, Result};

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

        let mut last_instruction = Instruction::None;
        loop {
            let pixel = self.get_next_instruction();
            self.pc = pixel.point;

            let instruction = pixel.as_instruction();
            let mut arg = None;

            // does the instruction take an arg?
            if instruction.takes_arg() {
                arg = Some(self.get_next_instruction());
                self.pc = arg.unwrap().point;
            }

            // execute the instruction
            if let Err(e) = self.execute_instruction(instruction, arg) {
                eprintln!("{}", e);
                break;
            }
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction, arg: Option<Pixel>) -> Result<()> {
        match instruction {
            Instruction::Road => Ok(()),
            Instruction::Push => match arg {
                Some(arg) => Ok(self.push(arg)),
                None => Err(anyhow!("no arg supplied")),
            },
            Instruction::Add => self.add(),
            Instruction::OutputUntil => self.output_until(),
            _ => todo!(),
        }
    }

    fn push(&mut self, arg: Pixel) {
        self.stack.push(arg.value)
    }

    fn add(&mut self) -> Result<()> {
        // TODO this is ugly
        let a = self.stack.pop().ok_or(anyhow!("stack is empty"))?;
        let b = self.stack.pop().ok_or(anyhow!("stack is empty"))?;

        self.stack.push(a + b);
        Ok(())
    }

    fn output_until(&mut self) -> Result<()> {
        let mut c = self.stack.pop().ok_or(anyhow!("stack is empty"))?;

        while c != 0 {
            println!("{}", c as u8 as char);
            c = self.stack.pop().ok_or(anyhow!("stack is empty"))?;
        }
        Ok(())
    }

    // prioritize roads over all other instructions besides the one in front of us
    pub fn get_next_instruction(&mut self) -> Pixel {
        let next_pixels = self.get_next_pixels();
        let (first_dir, first_pixel) = next_pixels[0];
        let first_road = next_pixels
            .iter()
            .filter(|(_dir, pixel)| matches!(pixel.as_instruction(), Instruction::Road))
            .next();

        // take the first road available, unless it's in the opposite direction
        // only take the opposite road if there are no other options
        if let Some((dir, road)) = first_road {
            if *dir != self.direction.opposite() {
                self.direction = *dir;
                return *road;
            }
        }

        // if there are no roads that don't lead backwards & there is an
        // instruction in front, take it
        if first_dir == self.direction {
            first_pixel
        } else if let Some((opp_dir, pixel)) = next_pixels.last() {
            // otherwise - if there are no roads to the left or right & nothing in front -
            // we go backwards (no matter if it's a road or not)
            self.direction = *opp_dir;
            *pixel
        } else {
            // it should be impossible for there to be nothing in front of or behind
            unreachable!()
        }
    }

    // try the pixel ahead of us. If that doesn't exist,
    // try the pixel to the 'right' (counter-clockwise & opposite). If that doesn't exist,
    // try the pixel to the 'left' (counter-clockwise). If that doesn't exist,
    // go back the way we came
    fn get_next_pixels(&self) -> Vec<(Direction, Pixel)> {
        let ins = &self.instructions;
        let mut next_pixels = vec![];

        let directions = [
            self.direction,                                // forward
            self.direction.counter_clockwise().opposite(), // right
            self.direction.counter_clockwise(),            // left
            self.direction.opposite(),                     // back
        ];

        for dir in directions {
            if let Some(point) = ins.go(self.pc, dir) {
                next_pixels.push((dir, point));
            }
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
        vm.pc = vm.find_start();
        vm
    }

    fn init_matrix(pixels: Vec<Vec<u16>>) -> Matrix<Pixel> {
        let mut v = vec![];

        for (row_idx, row) in pixels.iter().enumerate() {
            let mut row_vec = vec![];
            for (col_idx, pixel) in row.iter().enumerate() {
                row_vec.push(Pixel::new(*pixel, MatrixPoint(col_idx, row_idx)));
            }
            v.push(row_vec);
        }

        Matrix::new(v)
    }

    #[test]
    fn test_start_one_d() {
        let vm = init_vm(vec![vec![
            START, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306,
        ]]);

        assert_eq!(vm.pc, MatrixPoint(0, 0));
    }

    #[test]
    fn test_start_two_d() {
        let vm = init_vm(vec![
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
            vec![0, 180, 180, 36, 1, START, 2, 108, 36, 48, 108, 306],
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        assert_eq!(vm.pc, MatrixPoint(5, 1));
    }

    #[test]
    fn test_start_bounds() {
        let vm = init_vm(vec![
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
            vec![0, 180, 180, 36, 1, 3, 2, 108, 36, 48, 108, 306],
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, START],
        ]);

        assert_eq!(vm.pc, MatrixPoint(11, 2));
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
        let vm = init_vm(vec![vec![
            START, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306,
        ]]);

        let actual = vm.get_next_pixels();
        let expected = vec![(East, 180)];
        compare_pixels(actual, expected);
    }

    #[test]
    fn test_get_next_pixels_east_middle() {
        let vm = init_vm(vec![
            vec![0, 180, 180, 36, 1, 37, 2, 108, 36, 48, 108, 306],
            vec![0, 180, 180, 36, 1, START, 2, 108, 36, 48, 108, 306],
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        assert_eq!(vm.pc, MatrixPoint(5, 1));
        let pixels = vm.get_next_pixels();

        let expected = vec![(East, 2), (South, 36), (North, 37), (West, 1)];

        compare_pixels(pixels, expected);
    }

    #[test]
    fn test_get_next_pixels_east_bound() {
        let vm = init_vm(vec![
            vec![0, 180, 180, 36, 1, 37, 2, 108, 36, 48, 108, 310],
            vec![0, 180, 180, 36, 1, 108, 2, 108, 36, 48, 108, START],
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

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

        assert_eq!(vm.pc, MatrixPoint(0, 1));
        let actual = vm.get_next_pixels();

        let expected = vec![(North, 0), (South, 0), (East, 180)];

        compare_pixels(actual, expected);
    }

    #[test]
    fn test_get_next_instruction() {
        let mut vm = init_vm(vec![
            vec![0, 180, 180, 36, 1, 37, 2, 108, 70, 48, 108, 310],
            vec![START, 180, 180, 36, 1, 18, 2, 108, 36, 42, 108, 314],
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        vm.direction = East;

        assert_eq!(vm.pc, MatrixPoint(0, 1));
        let pixel = vm.get_next_instruction();

        assert_eq!(pixel.value, 180);
    }

    #[test]
    fn test_get_next_instruction1() {
        let mut vm = init_vm(vec![
            vec![0, 180, 180, 36, 1, 37, 2, 108, 70, 48, 108, 310],
            vec![START, 1, 180, 36, 1, 18, 2, 108, 36, 42, 108, 314],
            vec![0, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        vm.direction = East;

        let pixel = vm.get_next_instruction();

        assert_eq!(pixel.value, 1);
    }

    #[test]
    fn test_get_next_instruction2() {
        let mut vm = init_vm(vec![
            vec![180, 180, 180, 36, 1, 37, 2, 108, 70, 48, 108, 310],
            vec![START, 1, 180, 36, 1, 18, 2, 108, 36, 42, 108, 314],
            vec![180, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        vm.direction = East;

        let pixel = vm.get_next_instruction();

        // take the road to the 'right' (south)
        assert_eq!(pixel.value, 180);
    }

    #[test]
    fn test_get_next_instruction3() {
        let mut vm = init_vm(vec![
            vec![180, 180, 180, 36, 1, 37, 2, 108, 70, 48, 108, 310],
            vec![180, 180, 180, 36, 1, 18, 2, 108, 36, 42, 180, START],
            vec![180, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        vm.direction = East;

        let pixel = vm.get_next_instruction();

        // turn around
        assert_eq!(pixel.value, 180);
    }

    #[test]
    fn test_get_next_instruction4() {
        let mut vm = init_vm(vec![
            vec![180, 180, 180, 36, 1, 37, 2, 108, 70, 48, 108, 310],
            vec![180, 180, 180, 36, 180, START, 2, 108, 36, 42, 180, 200],
            vec![180, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        vm.direction = North;

        let pixel = vm.get_next_instruction();

        // go 'left'
        assert_eq!(pixel.value, 180);
    }

    #[test]
    fn test_get_next_instruction5() {
        let mut vm = init_vm(vec![
            vec![180, 180, 180, 36, 1, 37, 2, 108, 70, 48, 108, 310],
            vec![180, 180, 180, 36, 1, START, 2, 108, 36, 42, 180, 200],
            vec![180, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        vm.direction = North;

        let pixel = vm.get_next_instruction();

        assert_eq!(pixel.value, 37);
    }

    #[test]
    fn test_get_next_instruction6() {
        let mut vm = init_vm(vec![
            vec![180, 180, 180, 36, 1, 37, 2, 108, 70, 48, 108, 180],
            vec![180, 180, 180, 36, 1, 18, 2, 108, 36, 42, 180, START],
            vec![180, 180, 180, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        vm.direction = East;

        let pixel = vm.get_next_instruction();

        // don't turn around if there's another road available
        assert_eq!(pixel.value, 180);
    }

    #[test]
    fn test_get_next_instruction7() {
        let mut vm = init_vm(vec![
            vec![180, 180, 180, 36, 1, 37, 2, 108, 70, 48, 108, 180],
            vec![180, 180, 180, 36, 1, 18, 2, 108, 36, 42, 180, 180],
            vec![180, 1, START, 36, 1, 36, 2, 108, 36, 48, 108, 306],
        ]);

        vm.direction = North;

        let pixel = vm.get_next_instruction();

        // go north
        assert_eq!(pixel.value, 180);
    }
}
