use gpui::SharedString;

/// Google Material Icon
#[derive(Debug, Clone)]
pub struct Icon {
    pub name: SharedString,
    pub size: u32,
    pub is_dark: bool,
}

impl Icon {
    pub fn new(name: impl Into<SharedString>, size: u32, is_dark: bool) -> Self {
        Self { name: name.into(), size, is_dark }
    }

    pub fn svg_url(&self) -> String {
        format!(
            "https://fonts.gstatic.com/s/i/materialicons/{}/v6/{}px.svg",
            self.name,
            self.size
        )
    }
}