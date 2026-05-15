
pub mod fuji_status;
use fuji_status::FujiStatus;

use std::{fmt,io::{self, Write}};

use rusqlite::{Connection, params};
use pad::PadStr;
use chrono::prelude::*;


const TASK_DB : &str = "tasklist";

pub struct FujiTasks {
    conn : Connection  
}

pub struct FujiData {
    id        : i32,
    name      : String,
    status    : FujiStatus,
    created   : String,
    completed : String,

}

impl fmt::Display for FujiData {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let id_str = self.id.to_string().pad_to_width(3);
        let name_str = self.name.pad_to_width(20);
        let stat_str = self.status.to_string().pad_to_width(8);
        let start_str = self.created.pad_to_width(15);
        let complete_str = self.completed.pad_to_width(15);

        write!(f, " {}| {}| {}| {}| {}", id_str, name_str, stat_str, start_str, complete_str)
    }
}

impl FujiTasks {
    pub fn new() -> FujiTasks {
        let conn = Connection::open("tasks.db").unwrap();
        let cmd_str = format!("
            create table if not exists {TASK_DB} (
                ID          INT PRIMARY KEY NOT NULL,
                NAME        Text            NOT NULL,
                STATUS      Text            NOT NULL,
                CREATED     Text            NOT NULL,
                COMPLETED   Text            NOT NULL
                )
            ");


        let _cmd = conn.execute(&cmd_str, ());
        FujiTasks { conn : conn }
    }

    pub fn list_tasks (&self) {
        let cmd_str  = format!("SELECT * from {TASK_DB}");
        let mut stmt = self.conn.prepare(&cmd_str).unwrap();
        let mut rows = stmt.query([]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            let data = FujiData {
                id        : row.get(0).unwrap(),
                name      : row.get(1).unwrap(),
                status    : row.get(2).unwrap(),
                created   : row.get(3).unwrap(),
                completed : row.get(4).unwrap(),
            };
            println!("{}",data);
        }
        let count = self.num_tasks();
        println!("\nNumber of Tasks: {}", count)
    }

    fn num_tasks(&self) ->i32 {
        let cmd_str = format!("select count(*) from {TASK_DB}");
        self.get_single_int(cmd_str)
    }

    fn get_max(&self) -> i32 {
        let num_tasks = self.num_tasks();
        if num_tasks>0 {
            let cmd_str = format!("select MAX(ID) from {TASK_DB}");
            self.get_single_int(cmd_str)
        } else {
            0
        }
    }

    fn get_single_int(&self, cmd_str: String) -> i32 {
        let mut stmt = self.conn.prepare(&cmd_str).unwrap();
        let mut query_data = stmt.query([]).unwrap();

        match query_data.next().unwrap() {
            Some(data) =>
               data.get(0).unwrap(),
            None =>
                panic!("unable to get number of items")
        }
    }

    pub fn add_task(&self) {
        let new_id = self.get_max()+1;
        let local_time : DateTime<Local> = Local::now();
        let date_str = local_time.format("%d-%m-%Y").to_string();
        let cmd_str = format!("insert into {TASK_DB} values (?1, ?2, ?3, ?4, ?5)");

        // print!() needs to be followed by stdout flush as it may not appear if a \n is not
        // inlcuded in the string. This is important as we want to enter our text on the same line
        // as our prompt.
        print!("Enter a name for your new task: ");
        io::stdout().flush().expect("Failed to flush stdout");
        
        let mut name_str = String::new();
        io::stdin().read_line(&mut name_str).unwrap();

        self.conn.execute(&*cmd_str, params![new_id, name_str.trim(), "Open", date_str, "-"]).unwrap();

        println!("\nNew Task ID: {new_id}");
    }

    pub fn close_task(&self, id: i32) {
        println!("Closing Task!");
        let status_cmd = format!("update {TASK_DB} set status = ?1 where id = ?2");
        let local_time : DateTime<Local> = Local::now();
        let date_str = local_time.format("%d-%m-%Y").to_string();
        let date_cmd = format!("update {TASK_DB} set completed = ?1 where id = ?2");
        self.conn.execute(&*status_cmd, params![FujiStatus::Closed,id]).unwrap();
        self.conn.execute(&*&date_cmd, params![date_str,id]).unwrap();

    }

    pub fn del_task(&self, id: i32) {
        let cmd_str = format!("delete from {TASK_DB} where ID = ?1");

        self.conn.execute(&*cmd_str, params![id]).unwrap();
    }
}




pub fn test_fn() {
    println!("Hello, world!");
}

