mod apps;
mod components;
mod domain;
mod infrastructure;
mod services;

use apps::app::create_app;

fn main() {
    create_app();
}
