use crate::domain::params::design_token_config::{
    BORDER_WEIGHT, COLOR_COMPONENT_BASE, COLOR_GRAY_60, FOOTER_HEIGHT,
};
use crate::domain::params::map_config::DATA_PROJ_EPSG;
use crate::domain::traits::coordinate_transformer_trait::CoordinateTransformer;
use crate::infrastructure::coordinate::proj_core_coordinate_transformer::ProjCoreCoordinateTransformer;
use crate::services::map::use_map_instance::MapInstance;
use gpui::*;

pub struct Footer {
    map: MapInstance,
}

impl Footer {
    pub fn new(map: MapInstance) -> Self {
        Self { map }
    }

    pub fn render(&self) -> impl IntoElement {
        let transformer = ProjCoreCoordinateTransformer;
        let text = transformer
            .web_mercator_to_epsg_coordinate(self.map.center, DATA_PROJ_EPSG)
            .map(|coordinate| {
                SharedString::from(format!(
                    "{:.4}, {:.4} | Zoom: {:.1} | EPSG: {}",
                    coordinate.x, coordinate.y, self.map.zoom_level, coordinate.epsg,
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
            .child(div().absolute().text_xs().right(px(2.0)).child(text))
    }
}
