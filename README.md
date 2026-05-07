
# Fuji

This is a command-line application meant for task tracking. The purpose of this
is primarly to get exposure to Rust Programming.

Tasks are stored in and sqlite database so they are not tied to the application
actively running.


## Rust Crates Used
- Clap 
- mysqlite


## Testing Command Line Arguments
The easiest way to do this is not through cargo test. Instead, I would recommend
running cargo build, then executing the application with the desired arguments:

```
    ./target/debug/fuji -[arg]
```

