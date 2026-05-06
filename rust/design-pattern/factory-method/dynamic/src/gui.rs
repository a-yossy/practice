pub trait Button {
    fn render(&self);
    fn on_click(&self);
}

pub trait Dialog {
    fn create_button(&self) -> Box<dyn Button>;

    fn render(&self) {
        let button = self.create_button();
        button.render();
    }

    fn refresh(&self) {
        println!("Dialog - Refresh");
    }
}
