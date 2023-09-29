use rusqlite::Connection;
use std::env;

fn main() {
    let conn = Connection::open("tasks.db").expect("could not open connection to tasks database");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id    INTEGER PRIMARY KEY,
            name  TEXT NOT NULL,
            completed  INTEGER
        )",
        (),
    )
    .expect("could not create table");

    let args: Vec<String> = env::args().collect();
    let cmd_opt = args.get(1);
    match cmd_opt {
        Some(cmd) => match cmd.as_str() {
            "add" if args.len() > 2 => {
                println!("adding tasks");
                let tasks = &args[2..];
                for t in tasks {
                    conn.execute("INSERT INTO tasks (name) VALUES (?1)", (t.to_owned(),))
                        .expect("cannot enter task");
                }
                dbg!(tasks);
            }
            "add" => {
                println!("no tasks to add");
            }
            "remove" if args.len() > 2 => {
                println!("removing tasks");
                let task_ids = &args[2..];
                dbg!(task_ids);
            }
            "remove" => {
                println!("no tasks to remove");
            }
            "done" if args.len() > 2 => {
                println!("completing tasks");
                let task_ids = &args[2..];
                dbg!(task_ids);
            }
            "done" => {
                println!("no tasks to done");
            }
            "clear" => {
                println!("clearing all tasks");
            }
            _ => {
                println!("invalid command supplied")
            }
        },
        None => {
            println!("{}", "no command provided");
        }
    }
}
