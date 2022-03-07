use gst::element_error;
use gst::prelude::*;

use byte_slice_cast::*;

use anyhow::Error;
use derive_more::{Display, Error};

use iced::image;
use std::sync::mpsc;

#[derive(Debug, Display, Error)]
#[display(fmt = "Missing element {}", _0)]
struct MissingElement(#[error(not(source))] &'static str);

#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {}: {} (debug: {:?})", src, error, debug)]
struct ErrorMessage {
    src: String,
    error: String,
    debug: Option<String>,
    source: glib::Error,
}

pub struct Stream {
    pipeline: gst::Pipeline,
}

impl Stream {
    pub fn new() -> Self {
        gst::init().unwrap();

        let pipeline = gst::Pipeline::new(None);
        Stream { pipeline }
    }

    pub fn create_videopipeline(self, sender: mpsc::Sender<image::Handle>) -> Result<Self, Error> {
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

                    let handle = image::Handle::from_pixels(1280, 720, samples.to_vec());
                    sender.send(handle).unwrap();

                    Ok(gst::FlowSuccess::Ok)
                })
                .build(),
        );

        Ok(self)
    }

    pub fn main_loop(self) -> Result<(), Error> {
        self.pipeline.set_state(gst::State::Playing)?;

        let bus = self
            .pipeline
            .bus()
            .expect("Pipeline without bus. Shouldn't happen!");

        for msg in bus.iter_timed(gst::ClockTime::NONE) {
            use gst::MessageView;

            match msg.view() {
                MessageView::Eos(..) => break,
                MessageView::Error(err) => {
                    self.pipeline.set_state(gst::State::Null)?;
                    return Err(ErrorMessage {
                        src: msg
                            .src()
                            .map(|s| String::from(s.path_string()))
                            .unwrap_or_else(|| String::from("None")),
                        error: err.error().to_string(),
                        debug: err.debug(),
                        source: err.error(),
                    }
                    .into());
                }
                _ => (),
            }
        }

        self.pipeline.set_state(gst::State::Null)?;

        Ok(())
    }
}

pub enum State {
    Create,
    Running(mpsc::Receiver<image::Handle>),
}
