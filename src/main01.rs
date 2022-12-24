extern crate gtk;

use std::env::args;

use gtk::prelude::*;

use build_ui::build_ui;

mod dump_file;

mod build_ui;


fn main() {

    let application =
        gtk::Application::new(Some("org.photorotate108003"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());

}
