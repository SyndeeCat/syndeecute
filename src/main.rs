use gtk::{prelude::*, Application, ApplicationWindow, Box, Button, Orientation};

fn draw_executor(application: &Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("");

    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(750, 500);
    window.set_border_width(3);

    window.set_resizable(false);
    window.set_keep_above(true);

    window.show_all()
}

fn main() {
    let application = gtk::Application::builder()
        .application_id("org.syndee.SyndeeCute")
        .build();

    application.connect_activate(|app| {
        draw_executor(app);
    });

    application.run();
}
