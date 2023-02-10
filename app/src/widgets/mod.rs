pub struct View<'a> {
    pub(crate) root: Root<'a>,
}

impl<'a> View<'a> {
    pub fn new() -> Self {
        let root = Root::new();
        Self { root }
    }
    pub fn add_child(&mut self, child: impl Widget + 'a) -> &Self {
        self.root.add_child(child);
        self
    }
}

pub trait Widget {
    fn kind(&self) -> String {
        std::any::type_name::<Self>().to_string()
    }

    fn to_string(&self) -> String {
        std::any::type_name::<Self>().to_string()
    }
}

pub struct Root<'a> {
    pub(crate) children: Vec<Box<dyn Widget + 'a>>,
}

impl<'a> Root<'a> {
    pub fn new() -> Self {
        Self { children: vec![] }
    }
    fn add_child(&mut self, child: impl Widget + 'a) -> &Self {
        self.children.push(Box::new(child));
        self
    }
}

impl<'a> Widget for Root<'a> {}

pub struct Text {
    pub text: String,
}

impl Text {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self { text: text.into() }
    }
}

impl Widget for Text {
    fn to_string(&self) -> String {
        self.text.clone()
    }
}
