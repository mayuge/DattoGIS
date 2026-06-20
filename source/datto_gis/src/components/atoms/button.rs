use crate::domain::design_token_config::*;
use gpui::*;

/// ボタンの種類
#[derive(Debug, Clone, Copy)]
pub enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
    Warning,
    Success,
}

/// ボタンの形状
#[derive(Debug, Clone, Copy)]
pub enum ButtonShape {
    Rounded,
    Square,
    Circle,
}

/// ボタンのサイズ
#[derive(Debug, Clone, Copy)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

/// ボタンコンポーネント
#[derive(Debug, Clone)]
pub struct Button {
    pub text: String,
    pub variant: ButtonVariant,
    pub shape: ButtonShape,
    pub size: ButtonSize,
    pub icon_left: Option<String>,
    pub icon_right: Option<String>,
}

impl Button {
    pub fn new(
        text: String,
        variant: ButtonVariant,
        shape: ButtonShape,
        size: ButtonSize,
        icon_left: Option<String>,
        icon_right: Option<String>,
    ) -> Self {
        Self { text, variant, shape, size, icon_left, icon_right }
    }
    pub fn render(self) -> impl IntoElement {
        let (padding_x, padding_y) = match self.size {
            ButtonSize::Small => (SPACE_SM, SPACE_XS),
            ButtonSize::Medium => (SPACE_MD, SPACE_SM),
            ButtonSize::Large => (SPACE_LG, SPACE_MD),
        };

        let border_radius = match self.shape {
            ButtonShape::Rounded => px(BORDER_RADIUS_MD),
            ButtonShape::Square => px(0.0),
            ButtonShape::Circle => px(999.0),
        };

    let background = match self.variant {
        ButtonVariant::Primary => rgb(COLOR_PRIMARY),
        ButtonVariant::Secondary => rgb(COLOR_SECONDARY),
        ButtonVariant::Danger => rgb(COLOR_DANGER),
        ButtonVariant::Warning => rgb(COLOR_WARNING),
        ButtonVariant::Success => rgb(COLOR_SUCCESS),
    };

    let mut button = div()
        .flex()
        .items_center()
        .justify_center()
        .gap(px(SPACE_SM))
        .px(px(padding_x))
        .py(px(padding_y))
        .rounded(border_radius)
        .bg(background)
        .text_color(rgb(COLOR_TEXT));

        if let Some(icon) = self.icon_left {
            button = button.child(icon);
        }

        button = button.child(self.text);

        if let Some(icon) = self.icon_right {
            button = button.child(icon);
        }

        button
    }
}