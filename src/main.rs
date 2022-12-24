extern crate gtk;

use gtk::prelude::*;

use build_ui::build_ui;

mod dump_file;

mod build_ui;


fn main() {

    let application =
        gtk::Application::new(Some("org.photorotate108004"), Default::default());

    application.connect_activate(build_ui);

    application.run();

}
