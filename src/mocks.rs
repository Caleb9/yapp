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
            Ok(stdin.as_bytes().len())
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
