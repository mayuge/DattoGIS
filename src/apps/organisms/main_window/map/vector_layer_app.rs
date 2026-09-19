use gpui::{PathBuilder, Styled, canvas, point, px, rgb};

use crate::domain::params::design_token_config::COLOR_WARNING;
use crate::domain::traits::vector_layer_service_trait::ScreenGeometry;
use crate::domain::types::map_layer_type::VectorStyle;

pub struct VectorLayerApp;

impl VectorLayerApp {
    pub fn render(layers: Vec<(VectorStyle, f32, Vec<ScreenGeometry>)>) -> impl gpui::IntoElement {
        let prepaint_layers = layers.clone();
        canvas(
            move |_, _, _| prepaint_layers,
            move |_, layers, window, _| {
                for (style, opacity, geometries) in layers {
                    let color = parse_color(&style.fill_color).alpha(opacity);
                    for geometry in geometries {
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
