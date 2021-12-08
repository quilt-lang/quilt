use crate::hsl::Hsl;
use crate::{Matrix, MatrixPoint, Pixel};
use anyhow::Result;
use image::io::Reader as ImageReader;
use image::GenericImageView;
use std::path::PathBuf;

pub fn pixels<P: Into<PathBuf>>(path: P) -> Result<Vec<(u32, u32, Hsl)>> {
    Ok(ImageReader::open(path.into())?
        .decode()?
        .pixels()
        .map(|(x, y, px)| (x, y, px.into()))
        .collect())
}

pub fn parse(pixels: Vec<(u32, u32, Hsl)>) -> Matrix<Pixel> {
    let mut rows = vec![];
    let mut row: Vec<Pixel> = vec![];
    let mut prev_y = 0;
    for (x, y, p) in pixels {
        if y != prev_y {
            rows.push(row);
            prev_y = y;
            row = vec![];
        }
        let hsl: Hsl = p.into();
        row.push(Pixel::new(hsl.h, MatrixPoint(x as usize, y as usize)));
    }
    rows.push(row);
    Matrix::new(rows)
}

#[cfg(test)]
mod test {
    use crate::hsl::Hsl;

    fn create_pixels(v: Vec<u16>, width: usize, height: usize) -> Vec<(u32, u32, Hsl)> {
        let mut out = vec![];
        let mut iter = v.iter();
        for h in 0..height {
            for w in 0..width {
                out.push((
                    w as u32,
                    h as u32,
                    Hsl {
                        h: *iter.next().unwrap(),
                        s: 100,
                        l: 100,
                    },
                ));
            }
        }
        out
    }

    #[test]
    fn test_parse() {
        let pixels = create_pixels(vec![0, 1, 2, 3], 2, 2);
        let matrix = dbg!(super::parse(pixels));
        assert_eq!(matrix.matrix[0][0].value, 0);
        assert_eq!(matrix.matrix[0][1].value, 1);
        assert_eq!(matrix.matrix[1][0].value, 2);
        assert_eq!(matrix.matrix[1][1].value, 3);
    }
}
