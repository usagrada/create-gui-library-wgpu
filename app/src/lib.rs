pub mod widgets;
mod window;
mod render;

pub use window::Window;

pub struct App {
    window: Window,
    pub view: widgets::View<'static>,
}

impl App {
    pub fn new() -> Self {
        let window = Window::new();
        Self { window, view: widgets::View::new() }
    }

    pub fn run(mut self) {
        // println!("{:#?}", self.view);
        self.window.add_state();
        self.window.run(self.view);
    }

    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }
}
