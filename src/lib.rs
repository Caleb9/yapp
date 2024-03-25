//! ### Yet Another Password Prompt
//!
//! [![Crates.io Version](https://img.shields.io/crates/v/yapp)](https://crates.io/crates/yapp)
//!
//! A library for reading passwords from the user.
//!
//! This library provides a `PasswordReader` trait for reading passwords from the user.
//! It includes a default implementation of the `PasswordReader` trait, `Yapp`,
//! which can read passwords from an interactive terminal or from standard input.
//! The library also includes a `new` function for creating a new `PasswordReader` instance.
//!
//! ### Features
//!
//! * Reads user passwords from the input, optionally with a prompt and
//!   echoing replacement symbols (`*`, or another of your choice).
//! * Reads passwords interactively or non-interactively (e.g. when input is redirected through
//!   a pipe).
//! * Using the `PasswordReader` trait in your code allows for mocking the
//!   entire library in tests
//!   (see an [example](https://github.com/Caleb9/yapp/blob/main/examples/mock_yapp.rs))
//! * Thanks to using the `console` library underneath, it handles unicode
//!   correctly (tested on Windows and Linux).
//!
//! ### Usage Example
//!
//! ```rust
//! use yapp::PasswordReader;
//!
//! fn my_func<P: PasswordReader>(yapp: &mut P) {
//!     let password = yapp.read_password_with_prompt("Type your password: ").unwrap();
//!     println!("You typed: {password}");
//! }
//!
//! fn main() {
//!     let mut yapp = yapp::new().with_echo_symbol('*');
//!     my_func(&mut yapp);
//! }
//! ```
//!
//! The `yapp::new()` function returns an instance of `PasswordReader`
//! trait. Alternatively, instantiate with `Yapp::default()` to use a
//! concrete struct type.
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
///
/// Use the `new` function to obtain a new instance
pub trait PasswordReader {
    /// Reads a password from the user.
    fn read_password(&mut self) -> io::Result<String>;

    /// Reads a password from the user with a prompt.
    fn read_password_with_prompt(&mut self, prompt: &str) -> io::Result<String>;

    /// Sets the echoed replacement symbol for the password characters.
    ///
    /// Set to None to not echo any characters
    fn with_echo_symbol<C>(self, c: C) -> Self
    where
        C: 'static + Into<Option<char>>;
}

/// Creates a new password reader. Returns an instance of `PasswordReader` trait.
pub fn new() -> impl PasswordReader {
    Yapp::default()
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

    fn with_echo_symbol<C>(mut self, s: C) -> Self
    where
        C: 'static + Into<Option<char>>,
    {
        self.echo_symbol = s.into();
        self
    }
}

impl Yapp {
    /// Checks if the terminal is interactive.
    fn is_interactive(&self) -> bool {
        stdin().is_terminal()
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
