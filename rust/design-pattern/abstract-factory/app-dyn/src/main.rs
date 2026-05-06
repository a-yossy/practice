use gui::GuiFactoryDynamic;
use macos_gui::MacFactory;
use windows_gui::WindowsFactory;

pub fn render(factory: &dyn GuiFactoryDynamic) {
    let button1 = factory.create_button();
    let button2 = factory.create_button();
    let checkbox1 = factory.create_checkbox();
    let checkbox2 = factory.create_checkbox();

    button1.press();
    button2.press();
    checkbox1.switch();
    checkbox2.switch();
}

fn main() {
    let windows = false;

    let factory: &dyn GuiFactoryDynamic = if windows {
        &WindowsFactory
    } else {
        &MacFactory
    };

    let button = factory.create_button();
    button.press();

    render(factory);
}
