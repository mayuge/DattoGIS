mod apps;
mod components;
mod domain;
mod services;

use apps::app::create_app;

fn main() {
    create_app();
}
