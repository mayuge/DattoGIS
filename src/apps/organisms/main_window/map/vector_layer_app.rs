use gpui::{FillOptions, FillRule, PathBuilder, PathStyle, Styled, canvas, point, px, rgb};

use crate::apps::organisms::common::services::map::map_render_service::VectorGeometryCacheEntry;
use crate::domain::params::design_token_config::COLOR_WARNING;
use crate::domain::traits::vector_layer_service_trait::ScreenGeometry;

pub struct VectorLayerApp;

impl VectorLayerApp {
    pub fn render(layers: Vec<VectorGeometryCacheEntry>) -> impl gpui::IntoElement {
        let prepaint_layers = layers.clone();
        canvas(
            move |_, _, _| prepaint_layers,
            move |_, layers, window, _| {
                for (_, style, opacity, geometries) in layers {
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
                            ScreenGeometry::Polygon(rings) => {
                                let mut fill_path = PathBuilder::fill().with_style(
                                    PathStyle::Fill(
                                        FillOptions::default().with_fill_rule(FillRule::EvenOdd),
                                    ),
                                );
                                let mut stroke_path = PathBuilder::stroke(px(style.stroke_width));
                                for ring in &rings {
                                    let Some((first_x, first_y)) = ring.first().copied() else {
                                        continue;
                                    };
                                    let start = point(px(first_x), px(first_y));
                                    fill_path.move_to(start);
                                    stroke_path.move_to(start);
                                    for &(x, y) in &ring[1..] {
                                        let position = point(px(x), px(y));
                                        fill_path.line_to(position);
                                        stroke_path.line_to(position);
                                    }
                                    fill_path.close();
                                    stroke_path.close();
                                }
                                if let Ok(path) = fill_path.build() {
                                    window.paint_path(path, color);
                                }
                                if let Ok(path) = stroke_path.build() {
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
