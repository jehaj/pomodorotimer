# Simple Pomodoro Timer 

Running the _help_ command shows the available commands.

# Development environment
It is a standard rust project, so as long as you have rust installed along with cargo, 
you can make changes and try them with 
```bash
cargo run
```

RustRover ([link](https://www.jetbrains.com/rust/)) is a good IDE if that is something you want.

## Dependencies
Cargo handles most of it, except

- SQLite

Depending on your distribution you can install the dependencies with the following
<details>
<summary>Fedora</summary>

```bash
dnf install libsq3-devel
```

</details>
