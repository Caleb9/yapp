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
use mocks::{stdin, stdout, TermMock as Term};

#[cfg(test)]
mod mocks;

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
            write!(stdout(), "{}\n", format!("{s}").repeat(input.len()))?;
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

#[cfg(test)]
mod tests {
    use super::{
        mocks::{StdinMock, TermMock},
        PasswordReader,
    };
    use crate::mocks::StdOutMock;
    use console::Key;

    #[test]
    fn when_shell_is_interactive_password_reader_intercepts_keystrokes() {
        StdinMock::set_is_terminal(true);
        TermMock::setup_keys(&[
            Key::Char('a'),
            Key::Unknown,
            Key::Char('b'),
            Key::Char('z'),
            Key::Backspace,
            Key::Char('c'),
            Key::Enter,
        ]);
        let mut sut = super::new();

        let result = sut.read_password();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "abc");
    }

    #[test]
    fn when_shell_is_interactive_password_reader_correctly_handles_backspace() {
        StdinMock::set_is_terminal(true);
        TermMock::setup_keys(&[
            Key::Char('a'),
            Key::Char('b'),
            Key::Char('c'),
            Key::Backspace,
            Key::Backspace,
            Key::Backspace,
            Key::Backspace,
            Key::Enter,
        ]);
        let mut sut = super::new();

        let result = sut.read_password();

        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn when_shell_is_not_interactive_password_reader_reads_from_stdin() {
        StdinMock::set_is_terminal(false);
        StdinMock::set_input("P455w0rd!");
        let mut sut = super::new();

        let result = sut.read_password();

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "P455w0rd!");
    }

    #[test]
    fn password_reader_prints_prompt() {
        StdinMock::set_is_terminal(true);
        TermMock::setup_keys(&[Key::Char('a'), Key::Char('b'), Key::Char('c'), Key::Enter]);
        let mut sut = super::new();

        sut.read_password_with_prompt("Type a password: ").unwrap();

        let stdout_bytes = StdOutMock::get_output();
        let stdout_string = String::from_utf8_lossy(&stdout_bytes);
        assert_eq!(stdout_string, "Type a password: ");
    }

    #[test]
    fn password_reader_prints_replacement_symbols() {
        StdinMock::set_is_terminal(true);
        TermMock::setup_keys(&[Key::Char('a'), Key::Char('b'), Key::Char('c'), Key::Enter]);
        let mut sut = super::new();

        sut.set_echo_symbol('*');
        sut.read_password().unwrap();

        let term_bytes = TermMock::get_output();
        let term_string = String::from_utf8_lossy(&term_bytes);
        assert_eq!(term_string, "***\n");
    }
}
