extern crate gstreamer as gst;
use gtk::glib;

use gst::prelude::*;
use gtk::prelude::*;
use std::cell::RefCell;

fn build_ui(application: &gtk::Application, widget: &gtk::Widget) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("reStream 0.1");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(1280, 720);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    vbox.pack_start(widget, true, true, 0);
    let button = gtk::Button::from_icon_name(Some("window-close"), gtk::IconSize::Button);
    button.connect_clicked(glib::clone!(@weak window => move |_| {
        window.close();
    }));
    vbox.add(&button);

    window.add(&vbox);
    window.show_all();

    if let Some(screen) = window.screen() {
        let minotor = screen.monitor_geometry(0);
        let scale = screen.monitor_scale_factor(0);
        dbg!(minotor, scale);
        if minotor.width() * scale <= 1280 && minotor.height() * scale <= 720 {
            window.fullscreen();
            if let Some(window) = window.window() {
                let display = window.display();
                let cursor = gdk::Cursor::for_display(&display, gdk::CursorType::BlankCursor);
                window.set_cursor(cursor.as_ref());
            }
        }
    }
}

fn create_pipeline() -> (gst::Pipeline, gst::Bus, gtk::Widget) {
    let pipeline = gst::Pipeline::new(None);
    let bus = pipeline.bus().unwrap();

    let src = gst::ElementFactory::make("videotestsrc", None).unwrap();
    let capsfilter = gst::ElementFactory::make("capsfilter", None).unwrap();
    let (sink, widget) = if let Ok(gtkglsink) = gst::ElementFactory::make("gtkglsink", None) {
        let glsinkbin = gst::ElementFactory::make("glsinkbin", None).unwrap();
        glsinkbin.set_property("sink", &gtkglsink);
        let widget = gtkglsink.property::<gtk::Widget>("widget");
        (glsinkbin, widget)
    } else {
        let sink = gst::ElementFactory::make("gtksink", None).unwrap();
        let widget = sink.property::<gtk::Widget>("widget");
        (sink, widget)
    };

    src.set_property_from_str("pattern", "ball");
    src.set_property("is-live", true);

    let caps = gst::Caps::builder("video/x-raw")
        .field("width", 1280i32)
        .field("height", 720i32)
        .field("framerate", gst::Fraction::new(30, 1))
        .build();
    capsfilter.set_property("caps", &caps);

    pipeline.add_many(&[&src, &capsfilter, &sink]).unwrap();
    src.link(&capsfilter).unwrap();
    capsfilter.link(&sink).unwrap();

    (pipeline, bus, widget)
}

fn main() {
    gst::init().unwrap();
    gtk::init().unwrap();

    let application = gtk::Application::new(Some("com.mzyy94.restream"), Default::default());

    let (pipeline, bus, widget) = create_pipeline();
    let pipeline_weak = pipeline.downgrade();
    application.connect_startup(move |app| {
        let pipeline = match pipeline_weak.upgrade() {
            Some(pipeline) => pipeline,
            None => return,
        };
        build_ui(&app, &widget);

        pipeline
            .set_state(gst::State::Playing)
            .expect("Unable to set the pipeline to the `Playing` state");
    });

    let pipeline = RefCell::new(Some(pipeline));
    application.connect_shutdown(move |_| {
        if let Some(pipeline) = pipeline.borrow_mut().take() {
            pipeline
                .set_state(gst::State::Null)
                .expect("Unable to set the pipeline to the `Null` state");
            pipeline.bus().unwrap().remove_watch().unwrap();
        }
    });

    let app_weak = application.downgrade();
    bus.add_watch_local(move |_, msg| {
        use gst::MessageView;

        let app = match app_weak.upgrade() {
            Some(app) => app,
            None => return glib::Continue(false),
        };

        match msg.view() {
            MessageView::Eos(..) => app.quit(),
            MessageView::Error(err) => {
                println!(
                    "Error from {:?}: {} ({:?})",
                    err.src().map(|s| s.path_string()),
                    err.error(),
                    err.debug()
                );
                app.quit();
            }
            _ => (),
        };

        glib::Continue(true)
    })
    .expect("Failed to add bus watch");

    application.connect_activate(|_| {});
    application.run();
}
