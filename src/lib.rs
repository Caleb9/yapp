//! A library for reading passwords from the user.
//!
//! This library provides a `PasswordReader` trait for reading passwords from the user.
//! It includes a default implementation of the `PasswordReader` trait, `PasswordReaderImpl`,
//! which can read passwords from an interactive terminal or from standard input.
//! The library also includes a `new` function for creating a new `PasswordReader` instance.

use console::Key;
use std::io::{self, Write};

#[cfg(not(test))]
use {
    console::Term,
    std::io::{stdin, stdout, IsTerminal},
};

#[cfg(test)]
use tests::mocks::{stdin, stdout, TermMock as Term};

#[cfg(test)]
mod tests;

/// A trait for reading passwords from the user.
///
/// Use the `new` function to obtain a new instance
pub trait PasswordReader {
    /// Reads a password from the user.
    fn read_password(&mut self) -> io::Result<String>;

    /// Reads a password from the user with a prompt.
    fn read_password_with_prompt(&mut self, prompt: &str) -> io::Result<String>;

    /// Sets the replacement symbol for the password characters.
    ///
    /// Set to None to not echo any characters
    fn set_echo_symbol<S: 'static + Into<Option<char>>>(&mut self, symbol: S);
}

/// Creates a new password reader.
pub fn new() -> impl PasswordReader {
    PasswordReaderImpl {
        replacement_symbol: None,
    }
}

/// An implementation of the `PasswordReader` trait.
#[derive(Debug)]
struct PasswordReaderImpl {
    replacement_symbol: Option<char>,
}

impl PasswordReader for PasswordReaderImpl {
    fn read_password(&mut self) -> io::Result<String> {
        if self.is_interactive() {
            self.read_interactive()
        } else {
            self.read_non_interactive()
        }
    }

    fn read_password_with_prompt(&mut self, prompt: &str) -> io::Result<String> {
        write!(stdout(), "{prompt}")?;
        stdout().flush()?;
        self.read_password()
    }

    fn set_echo_symbol<S: Into<Option<char>>>(&mut self, symbol: S) {
        self.replacement_symbol = symbol.into()
    }
}

impl PasswordReaderImpl {
    /// Checks if the terminal is interactive.
    fn is_interactive(&self) -> bool {
        stdin().is_terminal()
    }

    /// Reads a password from a non-interactive terminal.
    fn read_non_interactive(&self) -> io::Result<String> {
        let mut input = String::new();
        let stdin = stdin();
        stdin.read_line(&mut input)?;
        if let Some(s) = self.replacement_symbol {
            writeln!(stdout(), "{}", format!("{s}").repeat(input.len()))?;
        }
        Ok(input)
    }

    /// Reads a password from an interactive terminal.
    fn read_interactive(&self) -> io::Result<String> {
        let mut term = Term::stdout();
        let mut input = String::new();
        loop {
            let key = term.read_key()?;
            match key {
                Key::Char(c) => {
                    input.push(c);
                    if let Some(s) = self.replacement_symbol {
                        write!(term, "{s}")?;
                    }
                }
                Key::Backspace if !input.is_empty() => {
                    input.pop();
                    term.clear_chars(1)?;
                }
                Key::Enter => {
                    term.write_line("")?;
                    break;
                }
                _ => {}
            }
        }
        Ok(input)
    }
}
