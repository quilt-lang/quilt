use super::pixel_widget::Pixel;
use crate::parser::{parse, pixels};
use crate::vm::Direction;
use crate::{Matrix, MatrixPoint};

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

const BLOCK_LIGHT: char = '\u{2591}';
const BLOCK_FULL: char = '\u{2588}';

pub struct ImageEditor {
    pixels: Matrix<crate::Pixel>,
    pixel_size: u32,
    position: MatrixPoint,
}

impl ImageEditor {
    pub fn new(file: &str, pixel_size: u32) -> Self {
        let pixels = parse(pixels(file, pixel_size).unwrap());
        Self {
            pixels,
            pixel_size,
            position: MatrixPoint::default(),
        }
    }

    pub fn go(&mut self, direction: Direction, steps: usize) {
        let new_pos = (0..steps).fold(self.position, |curr, _| {
            let next = curr.neighbor(direction).unwrap_or(curr);
            self.pixels.get(next).map(|_| next).unwrap_or(curr)
        });
        self.position = new_pos;
    }
}

impl Widget for &ImageEditor {
    fn render(self, area: Rect, buf: &mut Buffer) {
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
