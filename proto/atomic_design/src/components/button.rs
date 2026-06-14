
struct ButtonVariant {
    primary: String,
    secondary: String,
    danger: String,
    warning: String,
    success: String,
}

struct ButtonShape {
    rounded: String,
    square: String,
    circle: String,
}

struct ButtonSize {
    small: String,
    medium: String,
    large: String,
}

struct Button{
    text: String,
    variant: ButtonVariant,
    shape: ButtonShape,
    size: ButtonSize,
    icon_left: String,
    icon_right: String,
}