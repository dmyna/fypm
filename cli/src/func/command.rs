use std::io::{Error, Write};
use std::process::{Command, ExitStatus, Stdio};

pub fn stdin_all(command: &mut Command) -> Result<ExitStatus, Error> {
    let mut child = command.stdin(Stdio::piped()).spawn().unwrap();

    child
        .stdin
        .take()
        .unwrap()
        .write_all("all\n".as_bytes())
        .unwrap();
    child.wait()
}
