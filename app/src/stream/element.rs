use anyhow::Error;
use derive_more::{Display, Error};
use gst::prelude::*;

#[derive(Debug, Display, Error)]
#[display(fmt = "Missing element {}", _0)]
pub struct MissingElement(#[error(not(source))] pub &'static str);

macro_rules! element {
    ($factoryname:expr) => {
        gst::ElementFactory::make($factoryname, None).map_err(|_| MissingElement($factoryname))
    };
    ($factoryname:expr, $name:expr) => {
        gst::ElementFactory::make($factoryname, $name).map_err(|_| MissingElement($factoryname))
    };
}

pub fn add_link(pipeline: &gst::Pipeline, elms: &[&gst::Element]) -> Result<(), Error> {
    pipeline.add_many(elms)?;
    gst::Element::link_many(elms)?;
    Ok(())
}

pub fn remove_many(pipeline: &gst::Pipeline, elms: &[&gst::Element]) -> Result<(), Error> {
    for elm in elms.iter() {
        elm.set_state(gst::State::Null)?;
    }

    pipeline.remove_many(elms)?;
    Ok(())
}

pub(crate) use element;
