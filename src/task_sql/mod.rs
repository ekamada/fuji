
// use rusqlite::{Connection,Result};
use rusqlite::{Connection, params};

const TASK_DB : &str = "tasklist";

pub struct FujiTasks {
    conn : Connection  
}

pub struct FujiData {
    id        : i32,
    name      : String,
    desc      : String,
    status    : String,
    created   : String,
    completed : String,

}


impl FujiTasks {
    pub fn new() -> FujiTasks {
        let conn = Connection::open("tasks.db").unwrap();
        let cmd_str = format!("
            create table if not exists {TASK_DB} (
                ID          INT PRIMARY KEY NOT NULL,
                NAME        Text            NOT NULL,
                DESC        Text            NOT NULL,
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
                desc      : row.get(2).unwrap(),
                status    : row.get(3).unwrap(),
                created   : row.get(4).unwrap(),
                completed : row.get(5).unwrap(),
            };
            println!("{} {} {} {} ", data.id, data.name, data.desc, data.status);
        }
        let count = self.num_tasks();
        println!("\n{}", count)
    }

    fn num_tasks(&self) ->i32 {
        let cmd_str = format!("select count(*) from {TASK_DB}");
        let mut stmt = self.conn.prepare(&cmd_str).unwrap();
        let mut query_data = stmt.query([]).unwrap();

        match query_data.next().unwrap() {
            Some(data) =>
               data.get(0).unwrap(),
            None =>
                panic!("unable to get number of items")
        }
    }

    pub fn add_task(&self, name : String, descr : String ) {
        let new_id = self.num_tasks()+1;
        let cmd_str = format!("insert into {TASK_DB} values (?1, ?2, ?3, ?4, ?5, ?6)");
        println!("New Task ID: {new_id}");

        self.conn.execute(&*cmd_str, params![new_id, name, descr, "Open", "N/A", "-"]).unwrap();

    }

    pub fn dummy_add(&self) {
        let name  :String = String::from("Dummy Task");
        let descr :String = String::from("Fake Description");

        self.add_task(name, descr);

    }
}




pub fn test_fn() {
    println!("Hello, world!");
}

