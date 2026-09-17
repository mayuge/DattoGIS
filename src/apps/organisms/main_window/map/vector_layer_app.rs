use gpui::{PathBuilder, Styled, canvas, point, px, rgb};

use crate::apps::organisms::common::services::map::use_map_instance::MapInstance;
use crate::apps::organisms::common::services::map::vector_layer_service::VectorLayerService;
use crate::domain::params::design_token_config::COLOR_WARNING;
use crate::domain::traits::vector_layer_service_trait::{ScreenGeometry, VectorLayerServiceTrait};
use crate::domain::types::map_layer_type::{VectorFeature, VectorLayer};

pub struct VectorLayerApp;

impl VectorLayerApp {
    pub fn render(
        map: &MapInstance,
        width: f32,
        height: f32,
        layers: Vec<(VectorLayer, Vec<VectorFeature>)>,
    ) -> impl gpui::IntoElement {
        let center = map.center;
        let zoom_level = map.zoom_level.round() as u32;
        let mut visible_layers: Vec<_> = layers
            .into_iter()
            .filter(|(layer, _)| layer.visible)
            .collect();
        visible_layers.sort_by_key(|(layer, _)| layer.z_index);

        canvas(
            move |_, _, _| {
                visible_layers
                    .into_iter()
                    .map(|(layer, features)| (layer.style.clone(), layer.opacity, features.clone()))
                    .collect::<Vec<_>>()
            },
            move |bounds, layers, window, _| {
                let center_x = f64::from(bounds.origin.x) + f64::from(width) / 2.0;
                let center_y = f64::from(bounds.origin.y) + f64::from(height) / 2.0;
                let service = VectorLayerService;
                for (style, opacity, features) in layers {
                    let color = parse_color(&style.fill_color).alpha(opacity);
                    for geometry in service.screen_geometries(
                        &features,
                        center,
                        zoom_level,
                        (center_x, center_y),
                    ) {
                        match geometry {
                            ScreenGeometry::Point((x, y)) => {
                                let radius = px(4.0);
                                let mut path = PathBuilder::fill();
                                path.move_to(point(px(x) + radius, px(y)));
                                path.arc_to(
                                    point(radius, radius),
                                    px(0.0),
                                    false,
                                    false,
                                    point(px(x) - radius, px(y)),
                                );
                                path.arc_to(
                                    point(radius, radius),
                                    px(0.0),
                                    false,
                                    false,
                                    point(px(x) + radius, px(y)),
                                );
                                if let Ok(path) = path.build() {
                                    window.paint_path(path, color);
                                }
                            }
                            ScreenGeometry::LineString(positions) if positions.len() >= 2 => {
                                let mut path = PathBuilder::stroke(px(style.stroke_width));
                                let mut projected = positions.into_iter();
                                let Some((first_x, first_y)) = projected.next() else {
                                    continue;
                                };
                                path.move_to(point(px(first_x), px(first_y)));
                                for (x, y) in projected {
                                    path.line_to(point(px(x), px(y)));
                                }
                                if let Ok(path) = path.build() {
                                    window.paint_path(
                                        path,
                                        parse_color(&style.stroke_color).alpha(opacity),
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                }
            },
        )
        .size_full()
    }
}

fn parse_color(value: &str) -> gpui::Rgba {
    let value = value.trim_start_matches('#');
    let value = u32::from_str_radix(value, 16).unwrap_or(COLOR_WARNING);
    rgb(value)
}
