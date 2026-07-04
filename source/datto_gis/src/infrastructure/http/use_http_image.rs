enum ImageState {
    NotLoaded,
    Loading,
    Loaded(Vec<u8>),
}
struct HttpImage {
    url: String,
    width: f32,
    height: f32,
    state: ImageState,
}
