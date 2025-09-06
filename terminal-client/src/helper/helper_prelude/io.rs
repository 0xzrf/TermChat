use std::io::{self, Write};
use unicode_width::UnicodeWidthStr;

pub fn print_right(msg: &str) {
    if let Some(size) = termsize::get() {
        let width = size.cols as usize;
        let msg_width = UnicodeWidthStr::width(msg); // correct display width
        if msg_width < width {
            print!("{:>width$}\r\n", msg, width = width);
        } else {
            println!("{msg}"); // fallback if msg too long
        }
    } else {
        println!("{msg}"); // fallback if can't detect size
    }
    io::stdout().flush().unwrap();
}

pub fn print_center(msg: &str) {
    if let Some(size) = termsize::get() {
        let width = size.cols as usize;
        let msg_width = UnicodeWidthStr::width(msg); // correct display width

        if msg_width < width {
            let padding = (width.saturating_sub(msg_width)) / 2;
            // print spaces first, then the message
            println!("{:padding$}{}", "", msg, padding = padding);
        } else {
            println!("{msg}"); // fallback if too long
        }
    } else {
        println!("{msg}"); // fallback if can't detect size
    }
    io::stdout().flush().unwrap();
}

pub fn get_input() -> String {
    let mut input = String::from("");
    let stdin = std::io::stdin();

    stdin.read_line(&mut input).unwrap();
    input
}
