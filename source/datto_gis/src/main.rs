//現時点ではターミナルを出さないので、gui画面だけを出すための設定
#![windows_subsystem = "windows"]
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
