use std::time::Duration;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use gtk::prelude::*;
use gtk::glib;

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(app);

    window.set_title("Input text");
    window.set_default_width(480);
    window.connect_delete_event(|window,_| {
        window.close();
        Inhibit(false)
    });

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let entry = gtk::Entry::new();

    entry.connect_activate(glib::clone!(@weak window => move |entry| {
        let text = entry.text().to_string();
        entry.set_text("");
        print!("{}", text);
        window.close();
    }));

    vbox.set_spacing(0);
    vbox.pack_start(&entry, true, true, 0);

    window.add(&vbox);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new(
        Some("org.unknownplace.text_copy_widget"),
        gtk::gio::ApplicationFlags::NON_UNIQUE,
    );

    application.connect_startup(build_ui);
    application.connect_activate(|_| {});

    application.run();
}
