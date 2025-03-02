use std::io;
use yapp::{IsInteractive, PasswordReader};

fn _tested_func_with_retry_logic<P>(yapp: &mut P) -> io::Result<String>
where
    P: PasswordReader + IsInteractive,
{
    let mut password: String = String::from("non empty string");
    if yapp.is_interactive() {
        while password.is_empty() {
            password = yapp.read_password()?;
        }
    } else {
        password = yapp.read_password()?;
    }
    Ok(format!("Sending \"{password}\" to ether"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::mock;
    use std::io;
    use yapp::{IsInteractive, PasswordReader};

    mock! {
        Yacc {}
        impl PasswordReader for Yacc {
            fn read_password(&mut self) -> io::Result<String>;
            fn read_password_with_prompt(&mut self, prompt: &str) -> io::Result<String>;
        }

        // Optionally add `IsInteractive` if you need to mock `is_interactive` method.
        // In this example the `_tested_func_does_not_retry_when_not_interactive` depends on
        // `is_interactive`.
        impl IsInteractive for Yacc {
            fn is_interactive(&self) -> bool;
        }
    }

    #[test]
    fn _tested_func_does_not_retry_when_not_interactive() {
        let mut yacc_mock = MockYacc::new();
        yacc_mock.expect_is_interactive().return_const(false);
        yacc_mock
            .expect_read_password()
            .return_once(|| Ok(String::new()));

        let result = _tested_func_with_retry_logic(&mut yacc_mock);

        assert!(result.is_ok());
        assert_eq!("Sending \"\" to ether", result.unwrap())
    }
}
