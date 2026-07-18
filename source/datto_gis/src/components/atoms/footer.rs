use crate::domain::design_token_config::*;
use crate::domain::map_coordinate::WebMercatorCoordinate;
use crate::domain::traits::coordinate_transformer_trait::CoordinateTransformer;
use crate::infrastructure::coordinate::proj_core_coordinate_transformer::ProjCoreCoordinateTransformer;
use gpui::*;

pub struct Footer {
    map_center: WebMercatorCoordinate,
}

impl Footer {
    pub fn new(map_center: WebMercatorCoordinate) -> Self {
        Self { map_center }
    }

    pub fn render(&self) -> impl IntoElement {
        let transformer = ProjCoreCoordinateTransformer;
        let text = transformer
            .web_mercator_to_wgs84(self.map_center)
            .map(|coordinate| {
                SharedString::from(format!(
                    "{:.4}, {:.4}",
                    coordinate.latitude_deg, coordinate.longitude_deg
                ))
            })
            .unwrap_or_else(|_| SharedString::from("座標変換エラー"));

        div()
            .absolute()
            .bottom_0()
            .left_0()
            .h(px(FOOTER_HEIGHT))
            .w_full()
            .bg(rgb(COLOR_COMPONENT_BASE))
            .border_t(px(BORDER_WEIGHT))
            .border_color(rgb(COLOR_GRAY_60))
            .flex()
            .items_center()
            .child(
                div()
                    .absolute()
                    .text_xs()
                    .right(px(2.0))
                    .child(text),
            )
    }
}
