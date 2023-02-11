use gtk::{prelude::*, Application, ApplicationWindow};
fn draw_executor(application: &Application) {
    let window = ApplicationWindow::new(application);

    window.set_title("");

    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(750, 500);
    window.set_border_width(3);

    window.set_resizable(false);
    window.set_keep_above(true);

    let win_title = gtk::Label::new(None);
    win_title.set_markup("<big>Write a program name</big>");

    // Create an EntryCompletion widget
    let completion_countries = gtk::EntryCompletion::new();
    // Use the first (and only) column available to set the autocompletion text
    completion_countries.set_text_column(0);
    // how many keystrokes to wait before attempting to autocomplete?
    completion_countries.set_minimum_key_length(1);
    // whether the completions should be presented in a popup window
    completion_countries.set_popup_completion(true);

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
