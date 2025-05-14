use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = std::time::Instant::now();
    let bmp = snapsnap_lib::snap::take_screenshot()?;
    let elapsed = now.elapsed();
    println!("Screenshot taken in: {:?}", elapsed);
    let output_file = std::fs::File::create("screenshot.bmp")?;
    let mut writer = std::io::BufWriter::new(output_file);
    writer.write(&bmp)?;
    writer.flush()?;
    Ok(())
}
