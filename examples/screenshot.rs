use std::{
    io::{self, Write},
    time::Instant,
};

use windows_capture::{
    capture::{Context, GraphicsCaptureApiHandler},
    encoder::{AudioSettingsBuilder, ContainerSettingsBuilder, VideoEncoder, VideoSettingsBuilder},
    frame::Frame,
    graphics_capture_api::InternalCaptureControl,
    monitor::Monitor,
    settings::{ColorFormat, CursorCaptureSettings, DrawBorderSettings, Settings},
};

// Handles capture events.
struct Capture {
    // The video encoder that will be used to encode the frames.
    // encoder: Option<VideoEncoder>,
    // To measure the time the capture has been running
    start: Instant,
}

impl GraphicsCaptureApiHandler for Capture {
    // The type of flags used to get the values from the settings.
    type Flags = String;

    // The type of error that can be returned from `CaptureControl` and `start` functions.
    type Error = Box<dyn std::error::Error + Send + Sync>;

    // Function that will be called to create a new instance. The flags can be passed from settings.
    fn new(ctx: Context<Self::Flags>) -> Result<Self, Self::Error> {
        println!("Created with Flags: {}", ctx.flags);

        // let encoder = VideoEncoder::new(
        //     VideoSettingsBuilder::new(1920, 1080),
        //     AudioSettingsBuilder::default().disabled(true),
        //     ContainerSettingsBuilder::default(),
        //     "video.mp4",
        // )?;

        Ok(Self {
            // encoder: Some(encoder),
            start: Instant::now(),
        })
    }

    // Called every time a new frame is available.
    fn on_frame_arrived(
        &mut self,
        frame: &mut Frame,
        capture_control: InternalCaptureControl,
    ) -> Result<(), Self::Error> {
        print!(
            "\rRecording for: {} seconds",
            self.start.elapsed().as_secs()
        );
        io::stdout().flush()?;

        let mut data = frame.buffer()?;
        dbg!((
            data.as_raw_buffer().len(),
            data.width(),
            data.height(),
            (data.as_raw_buffer().len() / 4) / (data.width() as usize)
        ));
        data.save_as_image("screenshot.png", windows_capture::frame::ImageFormat::Png)
            .unwrap();

        capture_control.stop();

        Ok(())
    }

    // Optional handler called when the capture item (usually a window) closes.
    fn on_closed(&mut self) -> Result<(), Self::Error> {
        println!("Capture session ended");

        Ok(())
    }
}

fn main() {
    // Gets the foreground window, refer to the docs for other capture items
    let primary_monitor = Monitor::primary().expect("There is no primary monitor");

    let settings = Settings::new(
        // Item to capture
        primary_monitor,
        // Capture cursor settings
        CursorCaptureSettings::Default,
        // Draw border settings
        DrawBorderSettings::WithoutBorder,
        // The desired color format for the captured frame.
        ColorFormat::Rgba8,
        // Additional flags for the capture settings that will be passed to user defined `new` function.
        "Yea this works".to_string(),
    );

    if let Err(error) = Capture::start(settings) {
        dbg!(error.to_string());
    }
}
