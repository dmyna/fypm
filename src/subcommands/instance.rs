// use dialoguer::{console::Term, Confirm, Input};
// use std::io::Error;

use crate::utils::err::FypmError;

#[derive(Debug)]
struct Instance {
    name: String,
    description: String,
    timew: bool,
    task: bool,
}
pub struct InstanceHandler;
//#endregion
//#region           Implementation
impl InstanceHandler {
    pub fn add() -> Result<(), FypmError> {
        todo!();

        // let date_format = "%H:%M";
        // let term = Term::stdout();

        // let name = Input::<String>::new()
        //     .with_prompt("Write a name for your instance")
        //     .interact_text()
        //     .unwrap();
        // term.clear_last_lines(1).unwrap();

        // let description = Input::<String>::new()
        //     .with_prompt("Write a description for your instance")
        //     .interact_text()
        //     .unwrap();
        // term.clear_last_lines(1).unwrap();

        // let timew = Confirm::new()
        //     .with_prompt("Do you want to create a instance (new timeline) for timewarrior? (y/n)")
        //     .interact()
        //     .unwrap();
        // term.clear_last_lines(1).unwrap();

        // let task = Confirm::new()
        //     .with_prompt(
        //         "Do you want to create a instance (new tasks database) for taskwarrior? (y/n)",
        //     )
        //     .interact()
        //     .unwrap();
        // term.clear_last_lines(1).unwrap();

        // let instance = Instance {
        //     name,
        //     description,
        //     task,
        //     timew,
        // };

        // if timew {}
        // if task {}

        // Ok(())
    }
}

pub fn match_action(action: &str, actionargs: &Vec<String>) -> Result<(), FypmError> {
    match action {
        "add" => InstanceHandler::add(),
        "remove" => todo!(),
        "edit" => todo!(),
        "list" => todo!(),
        _ => panic!("Action not found!"),
    }
}
