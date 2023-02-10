fn main() {
    let mut app = app::App::new();
    app.set_title("Hello World!!");
    app.view.add_child(app::widgets::Text::new("Hello World!!"));
    app.view.add_child(app::widgets::Text::new("Hello World2!!"));
    app.run();
}
