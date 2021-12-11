use super::pixel_widget::Pixel;
use crate::parser::{parse, pixels};
use crate::vm::Direction;
use crate::{Matrix, MatrixPoint};

use anyhow::Result;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{Block, Widget};

const BLOCK_LIGHT: char = '\u{2591}';
const BLOCK_FULL: char = '\u{2588}';

pub struct ImageEditor<'a> {
    pixels: Matrix<crate::Pixel>,
    pixel_size: u32,
    position: MatrixPoint,
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// State of editor (normal or replace mode)
    state: State,
    /// Input buffer for replace mode
    input: String,
}

#[derive(Copy, Clone, Debug)]
pub enum State {
    Normal,
    Replace,
}

impl<'a> ImageEditor<'a> {
    pub fn new(file: &str, pixel_size: u32) -> Self {
        let pixels = parse(pixels(file, pixel_size).unwrap());
        Self {
            pixels,
            pixel_size,
            position: MatrixPoint::default(),
            block: None,
            state: State::Normal,
            input: String::new(),
        }
    }

    /// Set the widget to use the provided block.
    #[allow(unused)]
    pub fn block(&mut self, block: Block<'a>) {
        self.block = Some(block);
    }

    pub fn go(&mut self, direction: Direction, steps: usize) {
        let new_pos = (0..steps).fold(self.position, |curr, _| {
            let next = curr.neighbor(direction).unwrap_or(curr);
            self.pixels.get(next).map(|_| next).unwrap_or(curr)
        });
        self.position = new_pos;
    }

    pub fn pixel_info(&self) -> String {
        self.pixels
            .get(self.position)
            .map(|p| {
                format!(
                    "{}\n\ni: {:?}\nd: {}\nc: {:?}",
                    p.hsl,
                    p.as_instruction(),
                    p.hsl.h,
                    p.as_condition()
                )
            })
            .unwrap_or("error".to_string())
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn input(&self) -> &str {
        self.input.as_str()
    }

    pub fn set_state(&mut self, state: State) {
        self.input.clear();
        self.state = state;
    }

    pub fn pop(&mut self) {
        let _ = self.input.pop();
    }

    pub fn push(&mut self, c: char) {
        self.input.push(c);
    }

    pub fn submit(&mut self) -> Result<()> {
        let hue = self.input.parse::<u16>()?.clamp(0, 359);
        self.pixels[self.position].hsl.h = hue;
        Ok(self.set_state(State::Normal))
    }
}

impl<'a> Widget for &ImageEditor<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = match self.block {
            Some(ref b) => {
                b.to_owned().render(area, buf);
                b.inner(area)
            }
            None => area,
        };

        for (y, row) in self.pixels.matrix.iter().enumerate() {
            if y as usize >= area.height as usize {
                return;
            }
            for (x, pixel) in row.iter().enumerate() {
                // x takes up 2 cells
                if x as usize * 2 + 1 >= area.width as usize {
                    break;
                }
                let pixel: Pixel = pixel.into();
                let glyph = if pixel.position == self.position {
                    BLOCK_LIGHT
                } else {
                    BLOCK_FULL
                };
                pixel.draw_pixel(glyph, area, buf);
            }
        }
    }
}
