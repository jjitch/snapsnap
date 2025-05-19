use image::{codecs::avif, ImageEncoder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = std::time::Instant::now();
    let frame = snapsnap_lib::snap::take_screenshot()?;
    let elapsed = now.elapsed();
    println!("Screenshot taken in: {:?}", elapsed);
    let mut file = std::fs::File::create("screenshot.avif")?;

    let now = std::time::Instant::now();
    avif::AvifEncoder::new_with_speed_quality(&mut file, 10, 20).write_image(
        &frame.captured_data,
        frame.width,
        frame.height,
        image::ExtendedColorType::Rgba8,
    )?;
    let elapsed = now.elapsed();
    println!("Image saved in: {:?}", elapsed);
    Ok(())
}
