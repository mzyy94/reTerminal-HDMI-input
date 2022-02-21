use gtk::glib;
use gtk::prelude::*;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("reStream 0.1");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(1280, 720);

    let button = gtk::Button::from_icon_name(Some("window-close"), gtk::IconSize::Button);
    button.connect_clicked(glib::clone!(@weak window => move |_| {
        window.close();
    }));
    window.add(&button);

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

fn main() {
    let application = gtk::Application::new(Some("com.mzyy94.restream"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}
