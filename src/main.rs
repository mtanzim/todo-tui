use colored::Colorize;
use rusqlite::Connection;
use std::env;

#[derive(Debug)]
struct Task {
    id: i32,
    name: String,
    completed: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd_opt = args.get(1);
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

    let get_valid_ids = |task_ids: &[String]| -> Vec<i32> {
        task_ids
            .iter()
            .map(|raw_id| match raw_id.parse::<i32>() {
                Ok(num) => num,
                _ => -1,
            })
            .filter(|i| *i > 0)
            .collect()
    };

    let read_back = || {
        let mut stmt = conn
            .prepare("SELECT id, name, completed FROM tasks ORDER BY completed")
            .expect("could not read back tasks");
        let tasks_iter = stmt
            .query_map([], |row| {
                let completed = match row.get(2) {
                    Ok(1) => true,
                    _ => false,
                };
                Ok(Task {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    completed: completed,
                })
            })
            .expect("could not unmarshal to tasks");

        for task in tasks_iter {
            match task {
                Ok(t) => {
                    println!(
                        "{}\t{}",
                        t.id,
                        if t.completed {
                            t.name.green()
                        } else {
                            t.name.red()
                        }
                    )
                }
                Err(_) => {}
            }
        }
    };

    match cmd_opt {
        Some(cmd) => match cmd.as_str() {
            "view" => read_back(),
            "add" if args.len() > 2 => {
                println!("adding tasks");
                let tasks = &args[2..];
                for t in tasks {
                    match conn.execute(
                        "INSERT INTO tasks (name, completed) VALUES (?1, ?2)",
                        (t.to_owned(), 0),
                    ) {
                        Ok(_) => {}
                        Err(_) => {
                            println!("cannot insert task: {}", t)
                        }
                    }
                }
                read_back();
            }
            "add" => {
                println!("no tasks to add");
            }
            "remove" if args.len() > 2 => {
                let task_ids = &args[2..];
                let valid_ids = get_valid_ids(task_ids);
                println!("removing tasks {:?}", valid_ids);
                for id in valid_ids {
                    match conn.execute("DELETE FROM tasks WHERE id=?1", (id,)) {
                        Ok(_) => {}
                        Err(_) => {
                            println!("cannot delete task id: {}", id)
                        }
                    }
                }
                read_back();
            }
            "remove" => {
                println!("no tasks to remove");
            }
            "done" if args.len() > 2 => {
                println!("completing tasks");
                let task_ids = &args[2..];
                let valid_ids = get_valid_ids(task_ids);
                for id in valid_ids {
                    match conn.execute("UPDATE tasks SET completed = 1 WHERE id=?1", (id,)) {
                        Ok(_) => {}
                        Err(_) => {
                            println!("cannot update task id: {}", id)
                        }
                    }
                }
                read_back();
            }
            "done" => {
                println!("no tasks to done");
            }
            "clear" => {
                println!("clearing all tasks");
                match conn.execute("DELETE FROM tasks WHERE id > 0", ()) {
                    Ok(_) => {}
                    Err(_) => {
                        println!("cannot clear all tasks")
                    }
                }
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
