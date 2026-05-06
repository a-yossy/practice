use gui::{Button, Checkbox, GuiFactory, GuiFactoryDynamic};

pub struct WindowsButton;
impl Button for WindowsButton {
    fn press(&self) {
        println!("Windows button has pressed");
    }
}

pub struct WindowsCheckbox;
impl Checkbox for WindowsCheckbox {
    fn switch(&self) {
        println!("Windows checkbox has switched");
    }
}

pub struct WindowsFactory;

impl GuiFactory for WindowsFactory {
    type B = WindowsButton;
    type C = WindowsCheckbox;

    fn create_button(&self) -> Self::B {
        WindowsButton
    }

    fn create_checkbox(&self) -> Self::C {
        WindowsCheckbox
    }
}

impl GuiFactoryDynamic for WindowsFactory {
    fn create_button(&self) -> Box<dyn Button> {
        Box::new(WindowsButton)
    }

    fn create_checkbox(&self) -> Box<dyn Checkbox> {
        Box::new(WindowsCheckbox)
    }
}
