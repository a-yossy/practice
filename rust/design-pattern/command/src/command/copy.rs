use super::Command;
use crate::AppContext;
use cursive::{Cursive, views::EditView};

#[derive(Default)]
pub struct CopyCommand;

impl Command for CopyCommand {
    fn execute(&mut self, app: &mut cursive::Cursive) -> bool {
        let editor = app.find_name::<EditView>("Editor").unwrap();
        let mut context = app.take_user_data::<AppContext>().unwrap();

        context.clipboard = editor.get_content().to_string();

        app.set_user_data(context);
        false
    }

    fn undo(&mut self, _: &mut cursive::Cursive) {}
}
