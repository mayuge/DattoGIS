# GPUI + GPUI Component 環境構築手順（Windows / Rust）

## 概要
この手順では以下を構築します。

- Rust開発環境
- GPUI
- GPUI Component
- デフォルトアイコンアセット
- Hello World起動

完成するとネイティブデスクトップアプリが起動します。

---

# 1. Rust をインストール

Rust未導入ならインストール:

```bash
winget install Rustlang.Rustup
```

または:

https://www.rust-lang.org/tools/install

確認:

```bash
rustc --version
cargo --version
```

---

# 2. Visual Studio Build Tools インストール

GPUIはネイティブ依存があるため必要。

インストール:

https://visualstudio.microsoft.com/ja/downloads/

「Build Tools for Visual Studio」を選択。

ワークロード:

- C++ によるデスクトップ開発

追加推奨:

- Windows SDK
- CMake

インストール後PC再起動。

---

# 3. 新規プロジェクト作成

```bash
cargo new gpui-test
```

移動:

```bash
cd gpui-test
```

---

# 4. Cargo.toml 編集

既存内容を以下へ置換:

```toml
[package]
name = "gpui-component-test"
version = "0.1.0"
edition = "2024"

[dependencies]
anyhow = "1"

[dependencies.gpui]
git = "https://github.com/zed-industries/zed"

[dependencies.gpui_platform]
git = "https://github.com/zed-industries/zed"
features = ["font-kit"]

[dependencies.gpui-component]
git = "https://github.com/longbridge/gpui-component"

[dependencies.gpui-component-assets]
git = "https://github.com/longbridge/gpui-component"
```

---

# 5. src/main.rs 編集

```rust
use gpui::*;
use gpui_component::{button::*, *};

pub struct HelloWorld;

impl Render for HelloWorld {
    fn render(
        &mut self,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child("Hello, World!")
            .child(
                Button::new("ok")
                    .primary()
                    .label("Let's Go!")
                    .on_click(|_,_,_| {
                        println!("Clicked!")
                    })
            )
    }
}

fn main() {
    let app =
        gpui_platform::application()
        .with_assets(
            gpui_component_assets::Assets
        );

    app.run(move |cx| {

        // 超重要
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(
                WindowOptions::default(),
                |window,cx| {
                    let view=
                        cx.new(|_| HelloWorld);

                    cx.new(|cx| {
                        Root::new(
                            view,
                            window,
                            cx
                        )
                    })
                }
            )
            .expect("Failed");
        })
        .detach();
    });
}
```

重要:

```rust
gpui_component::init(cx);
```

これを入れないとテーマやコンポーネント機能が正常動作しません。

---

# 6. 起動

初回:

```bash
cargo run
```

Git依存を大量取得するので数分かかります。

成功すると:

- Hello World
- ボタン

が表示されたネイティブアプリが起動。

---

# 7. アイコン追加

GPUI ComponentにはSVGは含まれません。

Lucideを使用する例:

```rust
use gpui_component::{Icon,IconName};

Icon::new(IconName::Search)
```

必要SVGを配置。

---

# 8. よくあるエラー

## linker error

Visual Studio Build Tools不足。

C++ Desktop Developmentを追加。

---

## font-kit エラー

Cargo.toml:

```toml
features=["font-kit"]
```

を確認。

---

## 起動しても真っ白

Root忘れ:

```rust
Root::new(...)
```

---

## テーマが反映されない

```rust
gpui_component::init(cx)
```

忘れ。

---

# 次のおすすめ

入力欄:

```rust
InputState
```

ダイアログ:

```rust
Dialog
```

テーブル:

```rust
DataTable
```

サイドバー:

```rust
Sidebar
```

かなりElectronより軽量なデスクトップアプリをRustで作れる。

