use gpui::*;
use gpui_component::{Root, *};

struct MapApp {
    zoom: u8,
    cols: usize,
    rows: usize,
    tile_urls: Vec<String>,
}

impl MapApp {
    fn new() -> Self {
        let zoom = 3;
        let cols = 8;
        let rows = 8;
        let start_x = 0;
        let start_y = 0;

        let tile_urls = (0..rows)
            .flat_map(|row| {
                (0..cols).map(move |col| {
                    format!(
                        "https://tile.openstreetmap.org/{}/{}/{}.png",
                        zoom,
                        start_x + col as u32,
                        start_y + row as u32,
                    )
                })
            })
            .collect();

        Self {
            zoom,
            cols,
            rows,
            tile_urls,
        }
    }
}

impl Render for MapApp {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let mut root = div()
            .v_flex()
            .size_full()
            .gap_2()
            .paddings(16.0)
            .bg(rgb(0x0f0f11))
            .child(
                div()
                    .child(format!(
                        "OSM raster tiles · north-up · zoom {} · {}×{}",
                        self.zoom, self.cols, self.rows
                    ))
                    .text_color(rgb(0xf0f0f8))
                    .text_size(px(20.0)),
            );
        root.style().overflow.x = Some(Overflow::Scroll);
        root.style().overflow.y = Some(Overflow::Scroll);

        for row in 0..self.rows {
            let mut row_div = div().h_flex().gap_2();

            for col in 0..self.cols {
                let idx = row * self.cols + col;
                let tile_url = self.tile_urls[idx].clone();

                let tile = img(tile_url)
                    .size(px(256.0))
                    .border(px(1.0))
                    .border_color(rgb(0x292a2d))
                    .with_loading(|| "loading".into_any_element())
                    .with_fallback(|| "load error".into_any_element());

                row_div = row_div.child(tile);
            }

            root = root.child(row_div);
        }

        root
    }
}

fn main() {
    let app = gpui_platform::application().with_assets(gpui_component_assets::Assets);

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| MapApp::new());

                cx.new(|cx| Root::new(view, window, cx))
            })
            .expect("Failed to open window");
        })
        .detach();
    });
}
