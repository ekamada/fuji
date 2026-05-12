
# Fuji

This is a command-line application meant for task tracking. This is serving as
an opportunity for me to create something I have been thinking about for a
while, as well as gain some exposure to rust programming and relational
databases.

Tasks are stored in and sqlite database so they are not tied to the application
actively running. This is important as i wanted the state to be preserved
between instances.


## Rust Crates Used
- Clap      -> Command line arugments 
- mysqlite  -> Interfacing with sqlite database
- padStr    -> String formatting
- chrono    -> Get time/date

## Commmand Line Arguments
- List
- Add 
- Delete
- Close

## Testing Command Line Arguments
The easiest way to do this is not through cargo test. Instead, I would recommend
running cargo build, then executing the application with the desired arguments:

```
    ./target/debug/fuji -[arg]
```

