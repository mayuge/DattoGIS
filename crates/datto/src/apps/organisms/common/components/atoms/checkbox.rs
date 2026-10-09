use crate::domain::params::design_token_config::HEADER_ICON_SIZE;
use gpui::*;
use std::path::PathBuf;

pub struct Checkbox {
    checked: bool,
}

pub struct CheckboxChanged(pub bool);

impl EventEmitter<CheckboxChanged> for Checkbox {}

impl Checkbox {
    /// 初期値を持つチェックボックスを生成する。
    pub fn new(checked: bool) -> Self {
        Self { checked }
    }

    /// 現在の状態を返す。
    pub fn checked(&self) -> bool {
        self.checked
    }

    /// チェック状態を切り替え、変更イベントを発行する。
    pub fn toggle(&mut self, cx: &mut Context<Self>) {
        self.checked = !self.checked;
        cx.emit(CheckboxChanged(self.checked));
        cx.notify();
    }
}

impl Render for Checkbox {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let image_path = if self.checked {
            PathBuf::from("assets/components/check_box_true.svg")
        } else {
            PathBuf::from("assets/components/check_box_false.svg")
        };
        let checkbox = cx.entity().clone();

        div()
            .size(px(HEADER_ICON_SIZE))
            .flex()
            .items_center()
            .justify_center()
            .child(img(image_path).size(px(HEADER_ICON_SIZE)))
            .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                checkbox.update(cx, |checkbox, cx| checkbox.toggle(cx));
            })
    }
}
