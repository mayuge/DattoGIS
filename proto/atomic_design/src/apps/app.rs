use gpui::*;
use crate::components::atoms::button::*;

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
            ButtonShape::Square,
            ButtonSize::Medium,
            None,
            None,
        )
        .render()
    }
}

pub fn create_app() {
    Application::new().run(|cx| {
        cx.open_window(
            WindowOptions::default(),
            |_window, cx| cx.new(|_| App),
        )
        .unwrap();
    });
}