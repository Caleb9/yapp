use std::io;
use yapp::{PasswordReader, Yapp};

fn main() -> io::Result<()> {
    let mut yapp = Yapp::new().with_echo_symbol('*');
    let password = yapp.read_password_with_prompt("Type something and press ENTER: ")?;
    println!("You typed: {password}");
    Ok(())
}
