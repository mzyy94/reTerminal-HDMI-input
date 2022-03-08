use gst::element_error;
use gst::prelude::*;

use byte_slice_cast::*;

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
    frame_tx: Updater<iced::image::Handle>,
    pub frame_rx: Receiver<iced::image::Handle>,
    sound_left_tx: Updater<f32>,
    pub sound_left_rx: Receiver<f32>,
}

impl Default for Stream {
    fn default() -> Self {
        Stream::new()
    }
}

impl Stream {
    pub fn new() -> Self {
        gst::init().unwrap();

        let pipeline = gst::Pipeline::new(None);
        let (frame_rx, frame_tx) =
            channel_starting_with(image::Handle::from_pixels(1, 1, vec![0; 4]));
        let (sound_left_rx, sound_left_tx) = channel_starting_with(0f32);

        Stream {
            pipeline,
            frame_tx,
            frame_rx,
            sound_left_tx,
            sound_left_rx,
        }
    }

    pub fn create_videopipeline(self) -> Result<Self, Error> {
        #[cfg(feature = "nativesrc")]
        let src =
            gst::ElementFactory::make("v4l2src", None).map_err(|_| MissingElement("v4l2src"))?;
        #[cfg(feature = "testsrc")]
        let src = gst::ElementFactory::make("videotestsrc", None)
            .map_err(|_| MissingElement("videotestsrc"))?;
        let upload =
            gst::ElementFactory::make("glupload", None).map_err(|_| MissingElement("glupload"))?;
        let colorconvert = gst::ElementFactory::make("glcolorconvert", None)
            .map_err(|_| MissingElement("glcolorconvert"))?;
        let download = gst::ElementFactory::make("gldownload", None)
            .map_err(|_| MissingElement("gldownload"))?;
        let capsfilter = gst::ElementFactory::make("capsfilter", None)
            .map_err(|_| MissingElement("capsfilter"))?;
        let sink =
            gst::ElementFactory::make("appsink", None).map_err(|_| MissingElement("appsink"))?;

        self.pipeline
            .add_many(&[&src, &upload, &colorconvert, &download, &capsfilter, &sink])?;
        gst::Element::link_many(&[&src, &upload, &colorconvert, &download, &capsfilter, &sink])?;

        let appsink = sink
            .dynamic_cast::<gst_app::AppSink>()
            .expect("Sink element is expected to be an appsink!");

        let caps = gst::Caps::builder("video/x-raw")
            .field("width", 1280i32)
            .field("height", 720i32)
            .field("framerate", gst::Fraction::new(30, 1))
            .field("format", gst_video::VideoFormat::Bgra.to_str())
            .build();
        capsfilter.set_property("caps", &caps);

        let frame_tx = self.frame_tx.clone();
        appsink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                .new_sample(move |appsink| {
                    let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Eos)?;

                    let buffer = sample.buffer().ok_or_else(|| {
                        element_error!(
                            appsink,
                            gst::ResourceError::Failed,
                            ("Failed to get buffer from appsink")
                        );

                        gst::FlowError::Error
                    })?;

                    let map = buffer.map_readable().map_err(|_| {
                        element_error!(
                            appsink,
                            gst::ResourceError::Failed,
                            ("Failed to map buffer readable")
                        );

                        gst::FlowError::Error
                    })?;

                    let samples = map.as_slice_of::<u8>().map_err(|_| {
                        element_error!(
                            appsink,
                            gst::ResourceError::Failed,
                            ("Failed to interprete buffer as BGRA format")
                        );

                        gst::FlowError::Error
                    })?;

                    let frame = image::Handle::from_pixels(1280, 720, samples.to_vec());
                    frame_tx.update(frame).unwrap();

                    Ok(gst::FlowSuccess::Ok)
                })
                .build(),
        );

        Ok(self)
    }

    pub fn create_audiopipeline(self) -> Result<Self, Error> {
        #[cfg(feature = "nativesrc")]
        let src =
            gst::ElementFactory::make("alsasrc", None).map_err(|_| MissingElement("alsasrc"))?;
        #[cfg(feature = "testsrc")]
        let src = gst::ElementFactory::make("audiotestsrc", None)
            .map_err(|_| MissingElement("audiotestsrc"))?;
        let convert = gst::ElementFactory::make("audioconvert", None)
            .map_err(|_| MissingElement("audioconvert"))?;
        let sink =
            gst::ElementFactory::make("appsink", None).map_err(|_| MissingElement("appsink"))?;

        self.pipeline.add_many(&[&src, &convert, &sink])?;
        gst::Element::link_many(&[&src, &convert, &sink])?;

        let appsink = sink
            .dynamic_cast::<gst_app::AppSink>()
            .expect("Sink element is expected to be an appsink!");

        appsink.set_caps(Some(
            &gst::Caps::builder("audio/x-raw")
                .field("format", gst_audio::AUDIO_FORMAT_S16.to_str())
                .field("layout", "interleaved")
                .field("channels", 1i32)
                .field("rate", gst::IntRange::<i32>::new(1, i32::MAX))
                .build(),
        ));

        let mut last_level = 0f32;

        let sound_left_tx = self.sound_left_tx.clone();
        appsink.set_callbacks(
            gst_app::AppSinkCallbacks::builder()
                .new_sample(move |appsink| {
                    let sample = appsink.pull_sample().map_err(|_| gst::FlowError::Eos)?;

                    let buffer = sample.buffer().ok_or_else(|| {
                        element_error!(
                            appsink,
                            gst::ResourceError::Failed,
                            ("Failed to get buffer from appsink")
                        );

                        gst::FlowError::Error
                    })?;

                    let map = buffer.map_readable().map_err(|_| {
                        element_error!(
                            appsink,
                            gst::ResourceError::Failed,
                            ("Failed to map buffer readable")
                        );

                        gst::FlowError::Error
                    })?;

                    let samples = map.as_slice_of::<i16>().map_err(|_| {
                        element_error!(
                            appsink,
                            gst::ResourceError::Failed,
                            ("Failed to interprete buffer as S16 PCM")
                        );
                        gst::FlowError::Error
                    })?;

                    let sum: f32 = samples
                        .iter()
                        .map(|sample| {
                            let f = f32::from(*sample) / f32::from(i16::MAX);
                            f * f
                        })
                        .sum();
                    let rms = (sum / (samples.len() as f32)).sqrt();

                    let rms = rms.max(last_level * 0.95);

                    sound_left_tx.update(rms).unwrap();

                    last_level = rms;

                    Ok(gst::FlowSuccess::Ok)
                })
                .build(),
        );

        Ok(self)
    }

    pub fn run_loop(self) -> Result<Self, Error> {
        self.pipeline.set_state(gst::State::Playing)?;

        let bus = self
            .pipeline
            .bus()
            .expect("Pipeline without bus. Shouldn't happen!");

        let pipeline = self.pipeline.downgrade();

        thread::spawn(move || {
            let pipeline = pipeline.upgrade().unwrap();
            for msg in bus.iter_timed(gst::ClockTime::NONE) {
                use gst::MessageView;

                match msg.view() {
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
