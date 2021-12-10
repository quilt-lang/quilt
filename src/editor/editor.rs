use super::pixel_widget::Pixel;
use crate::parser::{parse, pixels};
use crate::Matrix;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

pub struct ImageEditor {
    pixels: Matrix<crate::Pixel>,
    pixel_size: u32,
}

impl ImageEditor {
    pub fn new(file: &str, pixel_size: u32) -> Self {
        let pixels = parse(pixels(file, pixel_size).unwrap());
        Self { pixels, pixel_size }
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
                pixel.render(area, buf);
            }
        }
    }
}
