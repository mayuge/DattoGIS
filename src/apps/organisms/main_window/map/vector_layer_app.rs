use gpui::{PathBuilder, Styled, canvas, point, px, rgb};

use crate::apps::organisms::common::services::map::use_map_instance::MapInstance;
use crate::apps::organisms::common::services::map::use_map_world_pixel::WorldPixel;
use crate::domain::traits::coordinate_transformer_trait::CoordinateTransformer;
use crate::domain::traits::world_pixel_trait::WorldPixelTrait;
use crate::domain::types::map_coordinate_type::EpsgCoordinate;
use crate::domain::types::map_layer_type::{VectorFeature, VectorLayer};
use crate::infrastructure::coordinate::proj_core_coordinate_transformer::ProjCoreCoordinateTransformer;

pub struct VectorLayerApp;

impl VectorLayerApp {
    pub fn render(
        map: &MapInstance,
        width: f32,
        height: f32,
        layers: Vec<(VectorLayer, Vec<VectorFeature>)>,
    ) -> impl gpui::IntoElement {
        let zoom_level = map.zoom_level.round() as u32;
        let center_pixel = WorldPixel::convert_coordinate_to_pixel(&map.center, zoom_level);
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
                for (style, opacity, features) in layers {
                    let color = parse_color(&style.fill_color).alpha(opacity);
                    for feature in features {
                        let Some((longitude, latitude)) = read_point_wkb(&feature.geometry_wkb)
                        else {
                            continue;
                        };
                        let Some(coordinate) = ProjCoreCoordinateTransformer
                            .epsg_coordinate_to_web_mercator(EpsgCoordinate {
                                x: longitude,
                                y: latitude,
                                epsg: 4326,
                            })
                            .ok()
                        else {
                            continue;
                        };
                        let pixel =
                            WorldPixel::convert_coordinate_to_pixel(&coordinate, zoom_level);
                        let x = (center_x + pixel.pixel_x - center_pixel.pixel_x) as f32;
                        let y = (center_y + pixel.pixel_y - center_pixel.pixel_y) as f32;
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
                }
            },
        )
        .size_full()
    }
}

fn read_point_wkb(wkb: &[u8]) -> Option<(f64, f64)> {
    if wkb.len() < 21 || wkb[0] != 1 {
        return None;
    }
    let geometry_type = u32::from_le_bytes(wkb[1..5].try_into().ok()?);
    if geometry_type != 1 {
        return None;
    }
    let x = f64::from_le_bytes(wkb[5..13].try_into().ok()?);
    let y = f64::from_le_bytes(wkb[13..21].try_into().ok()?);
    Some((x, y))
}

fn parse_color(value: &str) -> gpui::Rgba {
    let value = value.trim_start_matches('#');
    let value = u32::from_str_radix(value, 16).unwrap_or(0xf59e0b);
    rgb(value)
}
