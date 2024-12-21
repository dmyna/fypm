////////////////////////////////////////////////////////////////////////////////
// fypm - The Dark Souls of productivity.
// Copyright (C) 2023-2024 Rikagaku <contact.rikagaku@gmail.com>
// Copyright (C) 2023-2024 Myna <contact@devmyna.xyz>
//
// fypm is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// fypm is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with fypm. If not, see <https://www.gnu.org/licenses/>.
//
////////////////////////////////////////////////////////////////////////////////

// use dialoguer::{console::Term, Confirm, Input};
// use std::io::Error;

use fypm_lib::values::err::FypmError;

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
