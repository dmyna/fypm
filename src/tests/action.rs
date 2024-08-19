#[cfg(test)]
use crate::values;
#[cfg(test)]
use crate::func;

#[test]
fn verify_if_wt_is_allday() {
    let json = values::structs::TaskWarriorExported {
        id: 1,
        description: "test".to_string(),
        tags: Some(vec!["Test_Tag".to_string()]),
        project: None,
        state: "Time".to_string(),
        r#type: "Task".to_string(),
        wt: "AllDay".to_string(),
        inforelat: None,
        seq_current: None,
        seq_next: None,
        seq_prev: None,
        alias: None,
        style: None,
        entry: "2023-08-22T00:00:00Z".to_string(),
        modified: "2023-08-22T00:00:00Z".to_string(),
        status: "pending".to_string(),
        uuid: "1".to_string(),
        annotations: None,
        urgency: 0.0,
    };

    let exec = func::action::verify_if_wt_is_allday(&json);

    assert!(exec.is_err());
}
