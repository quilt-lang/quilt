use crate::hsl::Hsl;
use anyhow::Result;
use image::io::Reader as ImageReader;

pub fn parse() -> Result<()> {
    let img = ImageReader::open("examples/hello_world.png")?.decode()?;
    let x = img.into_rgb8();
    for p in x.pixels() {
        let p_hsl: Hsl = (*p).into();
        dbg!(p, p_hsl);
        break
    }
    Ok(())
}
