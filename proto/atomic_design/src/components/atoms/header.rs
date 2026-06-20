pub struct Header {
    title: String,
    pub on_close: Arc<dyn Fn()>,
    pub on_maximize: Arc<dyn Fn()>,
    pub on_minimize: Arc<dyn Fn()>,
}