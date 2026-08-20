use crate::components::atoms::search_input::SearchInput;
use gpui::*;

pub struct SearchApp;

impl SearchApp {
    pub fn render(search_input: Entity<SearchInput>) -> impl IntoElement {
        div().flex().justify_center().child(search_input)
    }
}
