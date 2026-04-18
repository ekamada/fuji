
// use rusqlite::{Connection,Result};
use rusqlite::{Connection, params};

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

        let _cmd = conn.execute("
            create table if not exists tasklist (
                ID          INT PRIMARY KEY NOT NULL,
                NAME        Text            NOT NULL,
                DESC        Text            NOT NULL,
                STATUS      Text            NOT NULL,
                CREATED     Text            NOT NULL,
                COMPLETED   Text            NOT NULL
                )
            ", ());

        FujiTasks { conn : conn }
    }

    pub fn list_tasks (&self) {
        let mut stmt = self.conn.prepare("SELECT * from tasklist").unwrap();
        let mut rows = stmt.query([]).unwrap();

        while let Some(row) = rows.next().unwrap() {
            // let test : i32 = row.get(0).unwrap();
            let data = FujiData {
                id        : row.get(0).unwrap(),
                name      : row.get(1).unwrap(),
                desc      : row.get(2).unwrap(),
                status    : row.get(3).unwrap(),
                created   : row.get(4).unwrap(),
                completed : row.get(5).unwrap(),
            };
            println!("{} {} {} {} ", data.id, data.name, data.desc, data.status)
        }
    }

    pub fn add_task(&self,task : FujiData) {
        println!("Hello World");

        let _ = self.conn.execute("INSERT INTO tasklist values (?1, ?2, ?3 ?4, ?5)", 
            params![task.id, task.name, task.desc, task.status, task.created, task.completed]);

    }
}

pub fn test_fn() {
    println!("Hello, world!");
}

// // pub fn open_db() -> Result<Connection> {
// pub fn open_db() -> Connection {
//
//     let conn = Connection::open("tasks.db").unwrap();
//
//     let _cmd = conn.execute("
//         create table if not exists tasklist (
//             ID          INT PRIMARY KEY NOT NULL,
//             NAME        Text            NOT NULL,
//             DESC        Text            NOT NULL,
//             STATUS      Text            NOT NULL,
//             CREATED     Text            NOT NULL,
//             COMPLETED   Text            NOT NULL
//             )
//         ", ());
//
//     return conn;
// }
//
// pub fn list_tasks(conn : Connection) -> String {
//
//
//
//     String::from("Hello World!")
// }
