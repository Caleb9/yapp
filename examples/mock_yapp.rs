/// An example of how to mock the `PasswordReader` trait in user code.
///
/// This example demonstrates how to use the `mockall` library to create a mock implementation of
/// the `PasswordReader` trait. The mock implementation can be used to test the `_your_func`
/// function without requiring a real `PasswordReader` implementation.
///
/// The `_your_func` function takes a `PasswordReader` implementation and reads a password from it.
/// The function then returns a string that includes the password, but in real code the password
/// would probably be used for some other purpose.
///
/// The `tests` module includes a test that replaces the `PasswordReader` implementation with a
/// mock implementation. The test uses the `mockall` library to create a mock implementation of
/// the `PasswordReader` trait. The test then uses the mock implementation to test the `_your_func`
/// function.
use std::io;
use yapp::PasswordReader;

fn _your_func<P: PasswordReader>(yapp: &mut P) -> io::Result<String> {
    let password = yapp.read_password()?;
    Ok(format!("Sending \"{password}\" to ether"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use std::io;
    use yapp::PasswordReader;

    mock! {
        Yacc {}
        impl PasswordReader for Yacc {
            fn read_password(&mut self) -> io::Result<String>;
            fn read_password_with_prompt(&mut self, prompt: &str) -> io::Result<String>;
            fn set_echo_symbol<S: 'static + Into<Option<char>>>(&mut self, symbol: S);
        }
    }

    #[test]
    fn replace_yacc_with_mock() {
        let mut yacc_mock = MockYacc::new();
        yacc_mock
            .expect_read_password()
            .return_once(|| Ok(String::from("P455w0rd!")));

        let result = _your_func(&mut yacc_mock);

        assert!(result.is_ok());
        assert_eq!("Sending \"P455w0rd!\" to ether", result.unwrap())
    }
}
