use gst::prelude::*;
use gst::{element_error, glib};

use anyhow::Error;
use derive_more::{Display, Error};

use std::thread;

use iced::image;
use single_value_channel::{channel_starting_with, Receiver, Updater};

#[derive(Debug, Display, Error)]
#[display(fmt = "Missing element {}", _0)]
struct MissingElement(#[error(not(source))] &'static str);

pub struct Stream {
    pipeline: gst::Pipeline,
    frame_ch: (Receiver<iced::image::Handle>, Updater<iced::image::Handle>),
    sound_ch: (Receiver<(f32, f32)>, Updater<(f32, f32)>),
}

impl Default for Stream {
    fn default() -> Self {
        Stream::new()
    }
}

macro_rules! element {
    ($factoryname:expr) => {
        gst::ElementFactory::make($factoryname, None).map_err(|_| MissingElement($factoryname))
    };
    ($factoryname:expr, $name:expr) => {
        gst::ElementFactory::make($factoryname, $name).map_err(|_| MissingElement($factoryname))
    };
}

impl Stream {
    pub fn new() -> Self {
        gst::init().unwrap();

        let pipeline = gst::Pipeline::new(None);
        let frame_ch = channel_starting_with(image::Handle::from_pixels(1, 1, vec![0; 4]));
        let sound_ch = channel_starting_with((0f32, 0f32));

        Stream {
            pipeline,
            frame_ch,
            sound_ch,
        }
    }

    pub fn get_frame(&mut self) -> &image::Handle {
        self.frame_ch.0.latest()
    }

    pub fn get_levels(&mut self) -> &(f32, f32) {
        self.sound_ch.0.latest()
    }

    pub fn create_videopipeline(self) -> Result<Self, Error> {
        #[cfg(feature = "nativesrc")]
        let src = element!("v4l2src")?;
        #[cfg(feature = "testsrc")]
        let src = element!("videotestsrc")?;
        let srccapsfilter = element!("capsfilter")?;
        let upload = element!("glupload")?;
        let mixer = element!("glvideomixer", Some("mix"))?;
        let colorconvert = element!("glcolorconvert")?;
        let download = element!("gldownload")?;
        let sinkcapsfilter = element!("capsfilter")?;
        let sink = element!("appsink")?;

        self.pipeline.add_many(&[
            &src,
            &srccapsfilter,
            &upload,
            &mixer,
            &colorconvert,
            &download,
            &sinkcapsfilter,
            &sink,
        ])?;
        gst::Element::link_many(&[
            &src,
            &srccapsfilter,
            &upload,
            &mixer,
            &colorconvert,
            &download,
            &sinkcapsfilter,
            &sink,
        ])?;

        let appsink = sink
            .dynamic_cast::<gst_app::AppSink>()
            .expect("Sink element is expected to be an appsink!");

        let caps = gst::Caps::builder("video/x-raw")
            .field("width", 1280i32)
            .field("height", 720i32)
            .field("framerate", gst::Fraction::new(30, 1))
            .field("format", gst_video::VideoFormat::Uyvy.to_str())
            .build();
        srccapsfilter.set_property("caps", &caps);

        let caps = gst::Caps::builder("video/x-raw")
            .field("width", 1280i32)
            .field("height", 720i32)
            .field("framerate", gst::Fraction::new(30, 1))
            .field("format", gst_video::VideoFormat::Bgra.to_str())
            .build();
        sinkcapsfilter.set_property("caps", &caps);

        let frame_tx = self.frame_ch.1.clone();
        appsink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                .new_sample(move |appsink| {
                    let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Eos)?;

                    let buffer = sample
                        .buffer()
                        .and_then(|buffer| buffer.map_readable().ok())
                        .ok_or_else(|| {
                            element_error!(
                                appsink,
                                gst::ResourceError::Failed,
                                ("Failed to get buffer readable")
                            );

                            gst::FlowError::Error
                        })?;

                    let frame = image::Handle::from_pixels(1280, 720, buffer.to_vec());
                    frame_tx.update(frame).unwrap();

                    Ok(gst::FlowSuccess::Ok)
                })
                .build(),
        );

        Ok(self)
    }

    pub fn create_audiopipeline(self) -> Result<Self, Error> {
        #[cfg(feature = "nativesrc")]
        let src = element!("alsasrc")?;
        #[cfg(feature = "testsrc")]
        let src = element!("audiotestsrc")?;
        let convert = element!("audioconvert")?;
        let capsfilter = element!("capsfilter")?;
        let level = element!("level")?;
        let sink = element!("fakesink")?;

        level.set_property("post-messages", true);
        level.set_property("interval", 30_000_000u64);
        sink.set_property("sync", true);

        self.pipeline
            .add_many(&[&src, &convert, &capsfilter, &level, &sink])?;
        gst::Element::link_many(&[&src, &convert, &capsfilter, &level, &sink])?;

        let caps = gst::Caps::builder("audio/x-raw")
            .field("channels", 2i32)
            .build();

        capsfilter.set_property("caps", &caps);

        Ok(self)
    }

    pub fn run_loop(self) -> Result<Self, Error> {
        self.pipeline.set_state(gst::State::Playing)?;

        let bus = self
            .pipeline
            .bus()
            .expect("Pipeline without bus. Shouldn't happen!");

        let pipeline = self.pipeline.downgrade();
        let sound_tx = self.sound_ch.1.clone();

        thread::spawn(move || {
            let pipeline = pipeline.upgrade().unwrap();
            let mut last_levels = vec![0f32; 2];
            for msg in bus.iter_timed(gst::ClockTime::NONE) {
                use gst::MessageView;

                match msg.view() {
                    MessageView::Element(_) => {
                        match msg.structure() {
                            Some(e) => {
                                let rms: glib::ValueArray = e.value("rms").unwrap().get().unwrap();

                                let get_rms = |ch: usize| {
                                    let value: f64 = rms.nth(ch as u32).unwrap().get().unwrap();
                                    let rms = 10f64.powf(value / 20f64) as f32;
                                    rms.max(last_levels[ch] * 0.95)
                                };

                                let levels = (get_rms(0), get_rms(1));
                                last_levels[0] = levels.0;
                                last_levels[1] = levels.1;

                                sound_tx.update(levels).unwrap();
                            }
                            None => (),
                        };
                    }
                    MessageView::Eos(..) => break,
                    MessageView::Error(err) => {
                        pipeline.set_state(gst::State::Null).unwrap();
                        panic!(
                            "Error {}: {}",
                            msg.src()
                                .map(|s| String::from(s.path_string()))
                                .unwrap_or_else(|| String::from("None")),
                            err.error().to_string(),
                        );
                    }
                    _ => (),
                }
            }

            pipeline.set_state(gst::State::Null).unwrap();
        });

        Ok(self)
    }
}
