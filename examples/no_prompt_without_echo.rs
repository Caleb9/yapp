use std::io;
use yapp::{PasswordReader, Yapp};

fn main() -> io::Result<()> {
    let mut yapp = Yapp::new();
    let password = yapp.read_password()?;
    println!("You typed: {password}");
    Ok(())
}
