//#region           Crates
use std::process::Command;
use std::str;
//#endregion
//#region           Implementation
pub fn annotate(command: &str, id: &String, annotation: &String) {
    let execute = Command::new("timew")
        .args([command, id, annotation])
        .output()
        .unwrap();

    println!("{}", str::from_utf8(&execute.stdout).unwrap());
}
//#endregion
