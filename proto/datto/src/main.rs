use gpui::*;
use gpui_component::{
    button::*,
    menu::{DropdownMenu, PopupMenuItem},
    *,
};

struct AppView;

impl Render for AppView {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .size_full()
            .v_flex()

            .child(
                div()
                    .h_10()
                    .h_flex()
                    .px_2()
                    .gap_1()
                    .border_b_1()

                    .child(
                        Button::new("file")
                            .ghost()
                            .label("File")

                            .dropdown_menu(|menu, _, _| {
                                menu

                                .item(
                                    PopupMenuItem::new("New")
                                    .on_click(|_, _, _| {
                                        println!("new");
                                    })
                                )

                                .item(
                                    PopupMenuItem::new("Open")
                                    .on_click(|_, _, _| {
                                        println!("open");
                                    })
                                )

                                .separator()

                                .item(
                                    PopupMenuItem::new("Exit")
                                    .on_click(|_, _, _| {
                                        std::process::exit(0);
                                    })
                                )
                            })
                    )

                    .child(
                        Button::new("edit")
                            .ghost()
                            .label("Edit")
                    )

                    .child(
                        Button::new("view")
                            .ghost()
                            .label("View")
                    )
            )

            .child(
                div()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .child("DattoGIS")
            )
    }
}

fn main() {
    let app =
        gpui_platform::application()
            .with_assets(
                gpui_component_assets::Assets
            );

    app.run(|cx| {

        gpui_component::init(cx);

        cx.spawn(async move |cx| {

            cx.open_window(
                WindowOptions::default(),
                |window, cx| {

                    let view =
                        cx.new(|_| AppView);

                    cx.new(|cx| {
                        Root::new(
                            view,
                            window,
                            cx
                        )
                    })
                }
            )
            .unwrap();

        })
        .detach();
    });
}