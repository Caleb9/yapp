use super::{IsInteractive, PasswordReader, Yapp};
use console::Key;
use mocks::{StdOutMock, StdinMock, TermMock};

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
    let mut sut = Yapp::new();

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
    let mut sut = Yapp::new();

    let result = sut.read_password();

    assert!(result.is_ok());
    assert!(result.unwrap().is_empty());
}

#[test]
fn when_shell_is_not_interactive_password_reader_reads_from_stdin() {
    StdinMock::set_is_terminal(false);
    StdinMock::set_input("P455w0rd!");
    let mut sut = Yapp::new();

    let result = sut.read_password();

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "P455w0rd!");
}

#[test]
fn password_reader_prints_prompt() {
    StdinMock::set_is_terminal(true);
    TermMock::setup_keys(&[Key::Char('a'), Key::Char('b'), Key::Char('c'), Key::Enter]);
    let mut sut = Yapp::new();

    sut.read_password_with_prompt("Type a password: ").unwrap();

    let stdout_bytes = StdOutMock::get_output();
    let stdout_string = String::from_utf8_lossy(&stdout_bytes);
    assert_eq!(stdout_string, "Type a password: ");
}

#[test]
fn password_reader_prints_replacement_symbols() {
    StdinMock::set_is_terminal(true);
    TermMock::setup_keys(&[Key::Char('a'), Key::Char('b'), Key::Char('c'), Key::Enter]);
    let mut sut = Yapp::new().with_echo_symbol('*');

    sut.read_password().unwrap();

    let term_bytes = TermMock::get_output();
    let term_string = String::from_utf8_lossy(&term_bytes);
    assert_eq!(term_string, "***\n");
}

#[test]
fn when_stdin_is_terminal_then_password_reader_is_interactive() {
    StdinMock::set_is_terminal(true);

    assert!(Yapp::new().is_interactive());
}

#[test]
fn when_stdin_is_not_terminal_then_password_reader_is_not_interactive() {
    StdinMock::set_is_terminal(false);

    assert!(!Yapp::new().is_interactive());
}

pub(crate) mod mocks {
    use console::Key;
    use std::cell::RefCell;
    use std::io;
    use std::io::Write;

    thread_local! {
        static TERM_KEYS: RefCell<Vec<Key>> = const { RefCell::new(vec![]) };
        static TERM_OUTPUT: RefCell<Vec<u8>> = const { RefCell::new(vec![]) };
        static STDOUT_OUTPUT: RefCell<Vec<u8>> = const { RefCell::new(vec![]) };
        static IS_TERMINAL: RefCell<bool> = const { RefCell::new(true) };
        static STDIN_INPUT: RefCell<&'static str> = const { RefCell::new("") };
    }

    pub struct TermMock;

    impl TermMock {
        pub fn setup_keys(keys: &[Key]) {
            TERM_KEYS.with_borrow_mut(|term_keys| {
                term_keys.clear();
                term_keys.extend(keys.iter().rev().map(|k| k.to_owned()))
            })
        }

        pub fn get_output() -> Vec<u8> {
            TERM_OUTPUT.with_borrow(Vec::clone)
        }

        pub fn stdout() -> TermMock {
            TermMock
        }

        pub fn read_key(&self) -> io::Result<Key> {
            Ok(TERM_KEYS.with_borrow_mut(|term_keys| {
                term_keys.pop().expect("key sequence should not be empty")
            }))
        }

        pub fn clear_chars(&self, n: usize) -> io::Result<()> {
            TERM_OUTPUT.with_borrow_mut(|term_output| {
                for _ in 0..n {
                    term_output.pop();
                }
                Ok(())
            })
        }

        pub fn write_line(&self, s: &str) -> io::Result<()> {
            TERM_OUTPUT.with_borrow_mut(|term_output| {
                write_to(term_output, s.as_bytes())?;
                write_to(term_output, "\n".as_bytes())?;
                Ok(())
            })
        }
    }

    impl Write for TermMock {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            TERM_OUTPUT.with_borrow_mut(|term_output| write_to(term_output, buf))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    pub struct StdinMock;

    pub fn stdin() -> StdinMock {
        StdinMock
    }

    impl StdinMock {
        pub fn set_is_terminal(is_terminal: bool) {
            IS_TERMINAL.with_borrow_mut(|terminal| *terminal = is_terminal);
        }

        pub fn is_terminal(&self) -> bool {
            IS_TERMINAL.with_borrow(|is_terminal| *is_terminal)
        }

        pub fn set_input(input: &'static str) {
            STDIN_INPUT.with_borrow_mut(|stdin| *stdin = input)
        }

        pub fn read_line(&self, buf: &mut String) -> io::Result<usize> {
            STDIN_INPUT.with_borrow_mut(|stdin| {
                buf.push_str(stdin);
                Ok(stdin.len())
            })
        }
    }

    pub struct StdOutMock;

    pub fn stdout() -> StdOutMock {
        StdOutMock
    }

    impl StdOutMock {
        pub fn get_output() -> Vec<u8> {
            STDOUT_OUTPUT.with_borrow(Vec::clone)
        }
    }

    impl Write for StdOutMock {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            STDOUT_OUTPUT.with_borrow_mut(|stdout| write_to(stdout, buf))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn write_to(target: &mut Vec<u8>, buf: &[u8]) -> io::Result<usize> {
        target.extend(buf.to_vec());
        Ok(buf.len())
    }
}
