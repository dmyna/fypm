use chrono::{DateTime, Local, Offset, ParseError};
use regex::Regex;

use super::action;
use crate::utils::{
    constants::CONTROL_TASK,
    enums::TimewAction,
    err::{FypmError, FypmErrorKind},
    get,
};

pub fn transform_dates_to_iso(received_time: String) -> Result<String, ParseError> {
    let transformed_time_str = Regex::new(r"(\d{4})(\d{2})(\d{2})T(\d{2})(\d{2})(\d{2})Z")
        .unwrap()
        .replace(&received_time, "$1-$2-$3 T $4:$5:$6 Z")
        .replace(" ", "")
        .to_string();
    // I put this space because dates the day case wasn't showing.
    // Dates like "20240101T000000Z" were showing as "2024-01-00:00:00".

    let parsed_time_str = DateTime::parse_from_rfc3339(&transformed_time_str.as_str())?;

    let local_offset = Local::now().offset().fix();

    let final_time = parsed_time_str
        .with_timezone(&local_offset)
        .format("%Y-%m-%dT%H:%M:%S")
        .to_string();

    Ok(final_time)
}
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
        },
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
