pub fn open_in_browser(url: &str) {
    webbrowser::open(url).expect("error ao abrir browser");
}
