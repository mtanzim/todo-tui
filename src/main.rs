use std::env;

enum Commands {
    Add(Vec<String>),
    Done(Vec<String>),
    Remove(Vec<String>),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd_opt = args.get(1);
    match cmd_opt {
        Some(cmd) => match cmd.as_str() {
            "add" if args.len() > 2 => {
                println!("adding tasks");
                let tasks = &args[2..];
                dbg!(tasks);
            }
            "add" => {
                println!("no tasks to add");
            }
            "remove" if args.len() > 2 => {
                println!("removing tasks");
                let tasks = &args[2..];
                dbg!(tasks);
            }
            "remove" => {
                println!("no tasks to remove");
            }
            "done" if args.len() > 2 => {
                println!("completing tasks");
                let tasks = &args[2..];
                dbg!(tasks);
            }
            "done" => {
                println!("no tasks to done");
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
