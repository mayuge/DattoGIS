use gpui::*;
mod domain;
mod components;
use components::atoms::button::*;

struct App;

impl Render for App {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        Button::new(
            String::from("保存"),
            ButtonVariant::Primary,
            ButtonShape::Circle,
            ButtonSize::Medium,
            None,
            None,
        )
        .render()
    }
}

fn main() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions::default(),
            |_window, cx| cx.new(|_| App),
        )
        .unwrap();
    });
}