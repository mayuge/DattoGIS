use crate::components::atoms::footer::Footer;
use crate::domain::params::map_config::DATA_PROJ_EPSG;
use crate::domain::traits::coordinate_transformer_trait::CoordinateTransformer;
use crate::infrastructure::coordinate::proj_core_coordinate_transformer::ProjCoreCoordinateTransformer;
use crate::services::map::use_map_instance::MapInstance;
use gpui::*;

pub struct FooterApp;

impl FooterApp {
    pub fn render(map: MapInstance) -> impl IntoElement {
        Footer::new(String::new(), Self::map_status_text(map)).render()
    }

    fn map_status_text(map: MapInstance) -> String {
        ProjCoreCoordinateTransformer
            .web_mercator_to_epsg_coordinate(map.center, DATA_PROJ_EPSG)
            .map(|coordinate| {
                format!(
                    "{:.4}, {:.4} | Zoom: {:.1} | EPSG: {}",
                    coordinate.x, coordinate.y, map.zoom_level, coordinate.epsg,
                )
            })
            .unwrap_or_else(|_| "座標変換エラー".to_string())
    }
}
