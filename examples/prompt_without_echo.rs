use std::io;
use yapp::PasswordReader;

fn main() -> io::Result<()> {
    let mut yapp = yapp::new();
    let password = yapp.read_password_with_prompt("Type something and press ENTER: ")?;
    println!("You typed: {password}");
    Ok(())
}
