//現時点ではターミナルを出さないので、gui画面だけを出すための設定

mod apps;
//ドメインフォルダ内は、未使用でもwarningを出さない
//#[allow(dead_code)]
mod domain;
mod infrastructure;

use apps::app::create_app;

fn main() {
    create_app();
}
