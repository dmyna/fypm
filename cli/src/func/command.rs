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

use std::io::{Error, Write};
use std::process::{Command, ExitStatus, Stdio};

/// Write "all" to the command's stdin and wait for the process to finish.
///
/// # Errors
///
/// Returns an `Error` if the command fails to spawn or writing to stdin fails.
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
