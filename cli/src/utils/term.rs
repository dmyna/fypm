use colored::*;

/// Prints a full divisory line in the terminal, given the
/// terminal size if it could be determined, or a default
/// of 30 characters if it could not.
pub fn print_full_divisory() -> () {
    let divisory_char = '─';

    if let Some((terminal_size::Width(width), _)) = terminal_size::terminal_size() {
        for _ in 0..width {
            print!("{}", divisory_char.to_string().bright_black());
        }
    } else {
        for _ in 0..30 {
            print!("{}", divisory_char.to_string().bright_black());
        }
    }

    println!();
}
