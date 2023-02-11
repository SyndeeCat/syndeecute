use gtk::prelude::*;
use gtk::{cairo, gdk};
use gtk::{Application, ApplicationWindow};

const WINDOW_DIMENTIONS: (i32, i32) = (750, 50);

fn draw_executor(application: &Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("");

    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(WINDOW_DIMENTIONS.0, WINDOW_DIMENTIONS.1);
    window.set_border_width(3);

    window.set_resizable(false);
    window.set_keep_above(true);
    window.set_app_paintable(true);

    window.connect_screen_changed(set_visual);
    window.connect_draw(draw);

    let win_title = gtk::Label::new(None);
    win_title.set_markup("<span foreground=\"white\"><big>Write a program name</big></span>");

    let completion_countries = gtk::EntryCompletion::new();
    completion_countries.set_text_column(0);
    completion_countries.set_minimum_key_length(2);
    completion_countries.set_popup_completion(true);
    completion_countries.set_popup_set_width(true);
    completion_countries.set_inline_completion(true);
    completion_countries.set_inline_selection(true);

    let ls = create_list_model();
    completion_countries.set_model(Some(&ls));

    let input_field = gtk::Entry::new();
    input_field.set_completion(Some(&completion_countries));

    let row = gtk::Box::new(gtk::Orientation::Vertical, 5);
    row.add(&win_title);
    row.pack_start(&input_field, false, false, 10);

    // window.add(&win_title);
    window.add(&row);

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

struct Data {
    name: String,
    exec: String,
    description: String,
}

fn create_list_model() -> gtk::ListStore {
    let col_types: [glib::Type; 3] = [glib::Type::STRING; 3];

    let a = gio::AppInfo::all()
        .iter()
        .map(|a| {
            (
                format!("{}", a.name()),
                format!(
                    "{}",
                    match a.description() {
                        Some(d) => d,
                        None => "No description.".into(),
                    }
                ),
                format!("{}", a.executable().to_str().unwrap()),
            )
        })
        .collect::<Vec<(String, String, String)>>();

    let mut data: Vec<Data> = Vec::new();
    for n in a {
        data.push(Data {
            name: n.0,
            description: n.1,
            exec: n.2,
        });
    }

    let store = gtk::ListStore::new(&col_types);
    for d in data.iter() {
        let values: [(u32, &dyn ToValue); 3] = [(0, &d.name), (1, &d.description), (2, &d.exec)];
        store.set(&store.append(), &values);
    }
    store
}

fn set_visual(window: &ApplicationWindow, _screen: Option<&gdk::Screen>) {
    if let Some(screen) = GtkWindowExt::screen(window) {
        if let Some(ref visual) = screen.rgba_visual() {
            window.set_visual(Some(visual));
        }
    }
}

fn draw(_window: &ApplicationWindow, ctx: &cairo::Context) -> Inhibit {
    ctx.set_source_rgba(0.1, 0.1, 0.1, 0.85); // almost black + light transparency
    ctx.set_operator(cairo::Operator::Screen);
    ctx.paint().expect("Invalid cairo surface state");
    Inhibit(false)
}
