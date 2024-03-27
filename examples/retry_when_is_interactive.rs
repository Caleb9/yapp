use std::io;
use yapp::{IsInteractive, PasswordReader};

fn main() -> io::Result<()> {
    let mut yapp = yapp::new().with_echo_symbol('*');
    loop {
        let password = yapp.read_password_with_prompt("Type something and press ENTER: ")?;
        if !yapp.is_interactive() || !password.is_empty() {
            println!("You typed: {password}");
            break Ok(());
        }
        eprintln!("You didn't type anything, try again!");
    }
}
