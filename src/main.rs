
pub mod task_sql;

use rusqlite::{Result};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {

    #[arg(short, long, default_value_t=String::from(""))]
    add : String,

    #[arg(short, long)]
    list : bool
}

fn main() -> Result<()> {
    println!("Hello, world!");

    let args  = Args::parse();

    let fuji = task_sql::FujiTasks::new();
    
    println!("{}", args.add);

    if args.list {
        println!("Listing tasks");
        fuji.list_tasks();
    }



    Ok(())
}
