use super::pixel_widget::Pixel;
use crate::parser::{parse, pixels};
use crate::vm::Direction;
use crate::{Matrix, MatrixPoint};

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
}

impl<'a> ImageEditor<'a> {
    pub fn new(file: &str, pixel_size: u32) -> Self {
        let pixels = parse(pixels(file, pixel_size).unwrap());
        Self {
            pixels,
            pixel_size,
            position: MatrixPoint::default(),
            block: None,
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
            .map(|p| format!("{}\n\n{:?}", p.hsl, p.as_instruction()))
            .unwrap_or("error".to_string())
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
            if y + area.y as usize >= area.height as usize {
                return;
            }
            for (x, pixel) in row.iter().enumerate() {
                if x + area.x as usize >= area.width as usize {
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
