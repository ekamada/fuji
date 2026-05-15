
pub mod task_sql;

use rusqlite::{Result};
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {

    /// List all tasks in the database
    #[arg(short, long)]
    list : bool,

    /// Add a new task to the database
    #[arg(short, long)]
    add : bool,

    /// Delete a task from the database based on task ID
    #[arg(short, long, default_value_t = 0)]
    del  : i32,

    /// Change the status of a task from 'Open' to 'Closed'. Based on Task ID
    #[arg(short, long, default_value_t = 0)]
    close : i32
}


fn main() -> Result<()> {
    let fuji = task_sql::FujiTasks::new();
    let num_args = std::env::args().len();

    // Check to see if multiple arguments have been grouped together (e,g. -la instead of -l)
    let arg_len = match std::env::args().nth(1) {
        Some(input) => input.len(),
        None        => 0
    };

    println!("{num_args}");
    let args = Args::parse();

    if num_args == 1 { // Interactive Mode. Not Implemented
        println!("No Arguments!");

    // } else if (num_args == 2 && arg_len == 2) || (num_args==3 && arg_len==2) { // 1 Argument detected!!
    } else if num_args == 2 { // 1 Argument detected!!

        if arg_len == 2 {
            if args.list {
                println!("Listing tasks");
                fuji.list_tasks();
            } else if args.add {
                fuji.add_task();
            }

        } else if arg_len == 3 {
            if args.del != 0 {
                fuji.del_task(args.del);
            } else if args.close != 0 {
                fuji.close_task(args.close);
            } else {
                println!("Too Many Arguments! Try just one at a time");
            }

        } else {
            println!("Too Many Arguments! Try just one at a time");
        }
    } else if num_args == 3 { 
        if arg_len == 2 {
            if args.del != 0 {
                fuji.del_task(args.del);
            } else if args.close != 0 {
                fuji.close_task(args.close);
            } else {
                println!("Too Many Arguments! Try just one at a time");
            }

        } else {
            println!("Too Many Arguments! Try just one at a time");
        }
    }

        // if arg_len == 2 {
        //     if args.list {
        //         println!("Listing tasks");
        //         fuji.list_tasks();
        //     }
        //
        //     if args.add {
        //         fuji.add_task();
        //     }
        //
        //     if args.del != 0 {
        //         fuji.del_task(args.del);
        //     }
        //
        //     if args.close != 0 {
        //         fuji.close_task(args.close);
        //     }
        //
        // } else { println!("Too Many Arguments!"); }

    // } else if num_args == 3 { 
    //     println!("Too Many Arguments!");
    // }
    //
    // } else { // Too many arguments. No action
    //     println!("Too Many Arguments!");
    // }
    
    Ok(())

}
