//! ### Yet Another Password Prompt
//!
//! [![Crates.io Version](https://img.shields.io/crates/v/yapp)](https://crates.io/crates/yapp)
//!
//! A library for reading passwords from the user.
//!
//! This library provides a `PasswordReader` trait for reading passwords from the user, and
//! `IsInteractive` trait for checking if terminal is attended by the user or e.g. ran as a
//! background service.
//! It includes a default implementation of the `PasswordReader` and `IsInteractive` traits, `Yapp`,
//! which can read passwords from an interactive terminal or from standard input.
//!
//! ### Features
//!
//! * Reads user passwords from the input, optionally with a prompt and
//!   echoing replacement symbols (`*`, or another of your choice).
//! * Reads passwords interactively or non-interactively (e.g. when input is redirected through
//!   a pipe).
//! * Using the `PasswordReader` (optionally `PasswordReader + IsInteractive`) trait in your code
//!   allows for mocking the entire library in tests
//!   (see an [example1](https://github.com/Caleb9/yapp/blob/main/examples/mock_yapp.rs) and
//!   [example2](https://github.com/Caleb9/yapp/blob/main/examples/mock_yapp_with_is_interactive.rs))
//! * Thanks to using the `console` library underneath, it handles Unicode
//!   correctly (tested on Windows and Linux).
//!
//! ### Usage Example
//!
//! ```rust
//! use yapp::{PasswordReader, Yapp};
//!
//! fn my_func<P: PasswordReader>(yapp: &mut P) {
//!     let password = yapp.read_password_with_prompt("Type your password: ").unwrap();
//!     println!("You typed: {password}");
//! }
//!
//! fn my_func_dyn(yapp: &mut dyn PasswordReader) {
//!     let password = yapp.read_password_with_prompt("Type your password: ").unwrap();
//!     println!("You typed: {password}");
//! }
//!
//! fn main() {
//!     let mut yapp = Yapp::new().with_echo_symbol('*');
//!     my_func(&mut yapp);
//!     my_func_dyn(&mut yapp);
//! }
//! ```
//!
//! See [examples](https://github.com/Caleb9/yapp/tree/main/examples) for more.

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
pub trait PasswordReader {
    /// Reads a password from the user.
    fn read_password(&mut self) -> io::Result<String>;

    /// Reads a password from the user with a prompt.
    fn read_password_with_prompt(&mut self, prompt: &str) -> io::Result<String>;
}

/// A trait providing interactivity check for `Yapp`
pub trait IsInteractive {
    /// Checks if the terminal is interactive.
    ///
    /// A terminal is not interactive when stdin is redirected, e.g. another process
    /// pipes its output to this process' input.
    fn is_interactive(&self) -> bool;
}

/// An implementation of the `PasswordReader` trait.
#[derive(Debug, Default, Copy, Clone)]
pub struct Yapp {
    echo_symbol: Option<char>,
}

impl PasswordReader for Yapp {
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
}

impl IsInteractive for Yapp {
    fn is_interactive(&self) -> bool {
        stdin().is_terminal()
    }
}

impl Yapp {
    /// Create new Yapp instance without echo symbol
    pub const fn new() -> Self {
        Yapp { echo_symbol: None }
    }

    /// Sets the echoed replacement symbol for the password characters.
    ///
    /// Set to None to not echo any characters
    pub fn with_echo_symbol<C>(mut self, s: C) -> Self
    where
        C: Into<Option<char>>,
    {
        self.echo_symbol = s.into();
        self
    }

    /// Reads a password from a non-interactive terminal.
    fn read_non_interactive(&self) -> io::Result<String> {
        let mut input = String::new();
        let stdin = stdin();
        stdin.read_line(&mut input)?;
        if let Some(s) = self.echo_symbol {
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
                    if let Some(s) = self.echo_symbol {
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
