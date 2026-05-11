
pub mod task_sql;

use rusqlite::{Result};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {

    // #[arg(short, long, default_value_t=String::from(""))]
    // add : String,
    #[arg(short, long)]
    add : bool,

    #[arg(short, long, default_value_t = 0)]
    del  : i32,

    #[arg(short, long, default_value_t = 0)]
    close : i32,

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

    if args.add {
        // fuji.dummy_add();
        fuji.add_task();
    }

    if args.del != 0 {
        fuji.del_task(args.del);
    }

    if args.close != 0 {
        fuji.close_task(args.close);

    }


    Ok(())
}
