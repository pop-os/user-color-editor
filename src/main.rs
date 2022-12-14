mod application;
#[rustfmt::skip]
mod config;
mod components;
mod localize;
mod util;
mod window;

use gtk4::{gio, glib};

use self::application::ExampleApplication;
use localize::localize;

fn main() {
    let _ = libcosmic::init();

    // Initialize logger
    pretty_env_logger::init();

    // Prepare i18n
    localize();

    gio::resources_register_include!("compiled.gresource").unwrap();

    glib::set_application_name(&fl!("app-name"));

    let app = ExampleApplication::new();
    app.run();
}
