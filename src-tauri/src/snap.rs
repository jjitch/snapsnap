use super::types::*;

pub fn snap() -> ImageBin {
    let img_loadeded = std::fs::read("icons/128x128.png").unwrap_or_default();
    ImageBin::new(img_loadeded)
}

use std::{io::Write, sync::mpsc};

use windows_capture::{
    capture::{Context, GraphicsCaptureApiHandler},
    encoder::ImageEncoder,
    frame::{Frame, ImageFormat},
    graphics_capture_api::InternalCaptureControl,
    monitor::Monitor,
    settings::{ColorFormat, CursorCaptureSettings, DrawBorderSettings, Settings},
};

#[derive(Default)]
pub struct RgbaImage {
    pub captured_data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl RgbaImage {
    pub fn save_as_jpeg(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let file = std::fs::File::create(path)?;
        let encoder = ImageEncoder::new(ImageFormat::Jpeg, ColorFormat::Rgba8);
        let jpg_data = encoder.encode(&self.captured_data, self.width, self.height)?;
        let mut writer = std::io::BufWriter::new(file);
        writer.write(&jpg_data)?;
        writer.flush()?;
        Ok(())
    }
}

struct Capture {
    owned_frame: mpsc::Sender<RgbaImage>,
}

impl GraphicsCaptureApiHandler for Capture {
    type Flags = mpsc::Sender<RgbaImage>;
    type Error = Box<dyn std::error::Error + Send + Sync>;
    fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
        Ok(Self {
            owned_frame: ctx.flags,
        })
    }

    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        capture_control: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        self.owned_frame.send(RgbaImage {
            captured_data: frame.buffer()?.as_nopadding_buffer()?.to_vec(),
            width: frame.width(),
            height: frame.height(),
        })?;
        capture_control.stop();
        Ok(())
    }
}

pub fn take_screenshot() -> Result<RgbaImage, Box<dyn std::error::Error>> {
    let primary_monitor = Monitor::primary().expect("There is no primary monitor");
    let (tx, rx) = mpsc::channel();

    let settings = Settings::new(
        primary_monitor,
        CursorCaptureSettings::WithCursor,
        DrawBorderSettings::WithoutBorder,
        ColorFormat::Rgba8,
        tx,
    );
    Capture::start_free_threaded(settings)?;
    let owned_frame = rx.recv().expect("Failed to receive frame");
    Ok(owned_frame)
}
