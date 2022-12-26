use std::env;
use std::process;
use std::thread;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Usage: {} <cmd> [args]", args[0]);
        process::exit(1);
    }

    let cmd = &args[1];
    let args = &args[2..];

    let mut proc = process::Command::new(cmd)
                                    .args(args)
                                    .spawn()
                                    .expect("Failed to execute command");
    println!("Process started with pid: {}", proc.id());

    loop {
        match proc.try_wait() {
            Ok(Some(status)) => {
                println!("Process exited with: {status}");
                break status;
            }
            Err(error) => {
                eprintln!("Error attempting to wait: {error}");
                process::exit(1);
            }
            _ => thread::yield_now(),
        };
    };

    let noti = process::Command::new("sh")
                                .args(["send_mail.sh", cmd])
                                .status()
                                .unwrap();
    assert!(noti.success());
}
