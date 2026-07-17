mod apps;
mod components;
//ドメインフォルダ内は、未使用でもwarningを出さない
#[allow(dead_code)]
mod domain;
mod infrastructure;
mod services;

use apps::app::create_app;

fn main() {
    create_app();
}
