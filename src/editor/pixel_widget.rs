use image::Rgba;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Color;
use tui::widgets::{Block, Widget};

const BLOCK_LIGHT: char = '\u{2591}';
const BLOCK_FULL: char = '\u{2588}';

/// A tui-rs Widget which displays a pixel
pub struct Pixel<'a> {
    /// A block to wrap the widget in
    block: Option<Block<'a>>,
    /// Pixel position and color to display
    pixel: (u16, u16, Rgba<u8>),
}

impl<'a> Pixel<'a> {
    /// Construct an Pixel widget with a single image.
    pub fn new(x: u16, y: u16, color: Rgba<u8>) -> Pixel<'a> {
        Pixel {
            block: None,
            pixel: (x, y, color),
        }
    }

    /// Set the widget to use the provided block.
    #[allow(unused)]
    pub fn block(mut self, block: Block<'a>) -> Pixel<'a> {
        self.block = Some(block);
        self
    }

    fn draw_pixel(&self, area: Rect, buf: &mut Buffer) {
        let (x, y, p) = self.pixel;
        let cell = buf.get_mut(area.left() + x * 2, area.top() + y);
        cell.set_char(BLOCK_FULL)
            .set_fg(Color::Rgb(p.0[0], p.0[1], p.0[2]));
        let cell = buf.get_mut(area.left() + x * 2 + 1, area.top() + y);
        cell.set_char(BLOCK_FULL)
            .set_fg(Color::Rgb(p.0[0], p.0[1], p.0[2]));
    }
}

impl<'a> Widget for Pixel<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = match self.block {
            Some(ref b) => {
                b.to_owned().render(area, buf);
                b.inner(area)
            }
            None => area,
        };

        if area.width < 1 || area.height < 1 {
            return;
        }

        self.draw_pixel(area, buf);
    }
}

impl From<&crate::Pixel> for Pixel<'_> {
    fn from(p: &crate::Pixel) -> Self {
        Self::new(p.point.0 as u16, p.point.1 as u16, p.hsl.into())
    }
}
