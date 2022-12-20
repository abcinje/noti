use std::env;
use std::process;
use std::process::Command;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <cmd> [args]", args[0]);
        process::exit(1);
    }

    let cmd = &args[1];
    let args = &args[2..];

    let mut process = Command::new(cmd)
                              .args(args)
                              .spawn()
                              .expect("Failed to execute command");
    println!("Process started with pid: {}", process.id());

    loop {
        match process.try_wait() {
            Ok(Some(status)) => {
                println!("Process exited with: {status}");
                break;
            }
            Err(error) => {
                eprintln!("Error attempting to wait: {error}");
                process::exit(1);
            }
            _ => thread::yield_now(),
        };
    }
}
