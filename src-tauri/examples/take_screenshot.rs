fn main() -> Result<(), Box<dyn std::error::Error>> {
    let now = std::time::Instant::now();
    let frame = snapsnap_lib::snap::take_screenshot()?;
    let elapsed = now.elapsed();
    println!("Screenshot taken in: {:?}", elapsed);
    frame.save_as_jpeg("screenshot.jpg")?;
    Ok(())
}
