use super::action;
use crate::commands;
use crate::handlers;

use crate::commands::TimewAction;
use crate::utils::get;
use fypm_lib::utils::parser::transform_dates_to_iso;
use fypm_lib::values::{
    constants::CONTROL_TASK,
    err::{FypmError, FypmErrorKind},
};

/// Verify the existence of some control tasks.
///
/// This functions is a factory for multiple verify scripts.
///
/// # Errors
///
/// Returns a `FypmError` if any of the verify script fails.
pub fn match_verify_script(script: &commands::VerifyScripts) -> Result<(), FypmError> {
    match script {
        commands::VerifyScripts::Aliases => handlers::aliases::verify_aliases_tasks(),
    }
}
/// Matches a special alias filter to its corresponding task identifier.
///
/// This function takes a filter string and returns a specific task UUID or control task based
/// on predefined aliases. It handles special cases such as "last" to fetch the last task,
/// and predefined single-letter filters for various activities. If the filter does not match
/// any known alias, it is returned as is.
///
/// # Parameters
/// - `filter`: A string reference representing the alias filter to match.
///
/// # Returns
/// A string representing the task identifier corresponding to the matched alias.
pub fn match_special_aliases(filter: &String) -> String {
    match filter.as_str() {
        // Last Task
        "last" => {
            let get_task = action::receive_last_task();

            match get_task {
                Ok(task) => task,
                Err(error) => {
                    if error.kind() == std::io::ErrorKind::NotFound {
                        CONTROL_TASK.to_string()
                    } else {
                        panic!("{}", error);
                    }
                }
            }
        }
        // Time without specific use
        "t" => CONTROL_TASK.to_string(),
        // Lost time
        "l" => "1469ac5d-78ab-463d-bf77-f56a9f042f48".to_string(),
        // Rest and breaks
        "d" => "309d9b37-cd99-4b2c-b3c7-a9c60cb1754f".to_string(),
        // Hygiene and Selfcare
        "h" => "a371cb4e-6fad-452f-a22c-abc932f0a83f".to_string(),
        // Singing
        "s" => "2d5d97b5-fe43-415f-8501-045aca46cdbb".to_string(),
        // Active Thought || DNM
        "p" => "dd67efbb-f010-42c7-b84c-5d0da1936e57".to_string(),
        // Calisthenics and Stretching
        "e" => "7806d5f7-db60-4841-ba83-97c2106499d3".to_string(),
        // Chess Practice
        "x" => "100372a8-5ca2-493a-b6f3-4b74195c8848".to_string(),
        // House Maintening
        "hm" => "ef5dbc2c-326e-4443-b0dc-b2595de6e012".to_string(),
        // Workflow Maintening
        "wm" => "b719a399-0b21-4fed-9118-017096466073".to_string(),
        // Tasks Maintening
        "tm" => "8980c7be-1fda-4888-b45a-1a2e52345947".to_string(),
        _ => filter.to_string(),
        // Need to implement a filter to prevent cases like "r", "ab", etc.
        // Now, if I write "r", it will pass and break
    }
}
/// This function receives a special id string that looks like "@id.action" (or "@id.property"), where "action" can be "start", "end" or "s" and "e" respectively, and "property" can be any property of the taskwarrior task.
///
/// If the id is not a special id, it will return an error.
///
/// If the id is a special id, it will return the value of the property that was asked.
pub fn match_special_timing_properties(id: &String) -> Result<String, FypmError> {
    if id.starts_with("@") {
        let properties = id.split(".").clone();

        if properties.clone().count() == 2 {
            let id = properties.clone().nth(0).unwrap();
            let received_action = properties.clone().nth(1).unwrap();
            let action: TimewAction;

            if received_action == "start" || received_action == "s" {
                action = TimewAction::Start;
            } else if received_action == "end" || received_action == "e" {
                action = TimewAction::End;
            } else {
                return Err(FypmError {
                    message: "You are trying to access a wrong property!".to_string(),
                    kind: FypmErrorKind::InvalidInput,
                });
            }

            let received_time = get::get_timew_time(&id.to_string(), &action);

            let parsed_time = transform_dates_to_iso(received_time).unwrap();

            Ok(parsed_time)
        } else if properties.count() > 2 {
            Err(FypmError {
                message: concat!(
                    "You are trying to access properties that doesn't exist (maybe... yet)! ",
                    "Try to use one property (like '@id.start' instead '@id.start.this_is_wrong')."
                )
                .to_string(),
                kind: FypmErrorKind::InvalidInput,
            })
        } else {
            Err(FypmError {
                message: "Why are you trying to access an id here? You need to write a property!"
                    .to_string(),
                kind: FypmErrorKind::InvalidInput,
            })
        }
    } else {
        Err(FypmError {
            message: "You are trying to access timew properties from a taskwarrior task!"
                .to_string(),
            kind: FypmErrorKind::InvalidInput,
        })
    }
}
