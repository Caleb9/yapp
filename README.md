# YAPP

Yet Another Password Prompt

`yapp` is a small library for Rust based on
[console](https://github.com/console-rs/console) to provide simple,
testable password prompt for CLI apps.

## Features

* Reads user passwords from the input, optionally with a prompt and
  echoing replacement symbols (`*`, or another of your choice).
* Reads passwords interactively:
  ```bash
  cargo run --example simple
  ```
* Reads passwords non-interactively:
  ```bash
  echo "P@55w0rd\n" | cargo run --example simple
  ```
* Using the `PasswordReader` trait in your code allows for mocking the
  entire library in tests (see an [example](examples/mock_yapp))
* Thanks to using `console` underneath, it handles unicode correctly
  (tested on Windows and Linux).

## Usage Example

```rust
use std::io;
use yapp::PasswordReader;

fn my_func<P: PasswordReader>(yapp: &mut P) {
    let password = yapp.read_password_with_prompt("Type your password: ").unwrap();
    println!("You typed: {password}");
}

fn main() {
    let mut yapp = yapp::new();
    yapp.set_echo_symbol('*');
    my_func(&mut yapp);
}
```

See [examples](examples/) for more.
