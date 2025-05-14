use super::types::*;

pub fn snap() -> ImageBin {
    let img_loadeded = std::fs::read("icons/128x128.png").unwrap_or_default();
    ImageBin::new(img_loadeded)
}

use std::sync::mpsc;

use windows_capture::{
    capture::{Context, GraphicsCaptureApiHandler},
    encoder::ImageEncoder,
    frame::{Frame, ImageFormat},
    graphics_capture_api::InternalCaptureControl,
    monitor::Monitor,
    settings::{ColorFormat, CursorCaptureSettings, DrawBorderSettings, Settings},
};

struct Capture {
    encoder: Option<ImageEncoder>,
    output_channel: mpsc::Sender<Vec<u8>>,
}

impl GraphicsCaptureApiHandler for Capture {
    // The type of flags used to get the values from the settings.
    type Flags = mpsc::Sender<Vec<u8>>;

    // The type of error that can be returned from `CaptureControl` and `start` functions.
    type Error = Box<dyn std::error::Error + Send + Sync>;

    // Function that will be called to create a new instance. The flags can be passed from settings.
    fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
        let encoder = ImageEncoder::new(ImageFormat::Bmp, ColorFormat::Rgba8);

        Ok(Self {
            encoder: Some(encoder),
            output_channel: ctx.flags,
        })
    }

    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        capture_control: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        let width = frame.width();
        let height = frame.height();
        let encoded_data = self.encoder.as_mut().unwrap().encode(
            frame.buffer().unwrap().as_raw_buffer(),
            width,
            height,
        )?;
        capture_control.stop();
        self.output_channel
            .send(encoded_data)
            .map_err(|_| "Failed to send data to channel")?;
        Ok(())
    }
}

pub fn take_screenshot() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Gets the foreground window, refer to the docs for other capture items
    let primary_monitor = Monitor::primary().expect("There is no primary monitor");

    let (tx, rx) = mpsc::channel();

    let settings = Settings::new(
        primary_monitor,
        CursorCaptureSettings::WithCursor,
        DrawBorderSettings::WithoutBorder,
        ColorFormat::Rgba8,
        tx,
    );

    // Starts the capture and takes control of the current thread.
    // The errors from handler trait will end up here
    Capture::start(settings)?;

    Ok(rx.recv()?)
}
