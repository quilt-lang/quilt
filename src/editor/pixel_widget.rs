use crate::MatrixPoint;

use image::Rgba;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Color;
use tui::widgets::Block;

/// A tui-rs Widget which displays a pixel
pub struct Pixel<'a> {
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Pixel position
    pub position: MatrixPoint,
    /// Pixel color
    color: Rgba<u8>,
}

impl<'a> Pixel<'a> {
    /// Construct an Pixel widget with a single image.
    pub fn new(x: u16, y: u16, color: Rgba<u8>) -> Pixel<'a> {
        Pixel {
            block: None,
            position: MatrixPoint(x as usize, y as usize),
            color,
        }
    }

    /// Set the widget to use the provided block.
    #[allow(unused)]
    pub fn block(mut self, block: Block<'a>) -> Pixel<'a> {
        self.block = Some(block);
        self
    }

    pub fn draw_pixel(&self, glyph: char, area: Rect, buf: &mut Buffer) {
        let x = self.position.0 as u16;
        let y = self.position.1 as u16;
        let color = Color::Rgb(self.color.0[0], self.color.0[1], self.color.0[2]);
        let cell = buf.get_mut(area.left() + x * 2, area.top() + y);
        cell.set_char(glyph).set_fg(color);
        let cell = buf.get_mut(area.left() + x * 2 + 1, area.top() + y);
        cell.set_char(glyph).set_fg(color);
    }
}

impl From<&crate::Pixel> for Pixel<'_> {
    fn from(p: &crate::Pixel) -> Self {
        Self::new(p.point.0 as u16, p.point.1 as u16, p.hsl.into())
    }
}
