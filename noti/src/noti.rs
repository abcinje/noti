use std::process::{Child, Command, ExitStatus};
use std::{env, fs, process, thread};

struct ProcInfo {
    command: String,
    pid: u32,
    status: ExitStatus,
}

fn launch(args: &[String]) -> Child {
    let cmd = &args[0];
    let args = &args[1..];

    Command::new(cmd)
        .args(args)
        .spawn()
        .expect("Failed to execute command")
}

fn wait(mut proc: Child) -> ExitStatus {
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
    }
}

fn notify(proc_info: ProcInfo) {
    fs::create_dir_all("proc").expect("Failed to create directory");

    let mut msg = String::new();
    msg += &format!("Command: {}\n", proc_info.command);
    msg += &format!("PID: {}\n", proc_info.pid);
    msg += &format!("Status: {}\n", proc_info.status);

    let file = format!("proc/{}", proc_info.pid.to_string());
    fs::write(&file, msg).expect("Failed to write message");

    let noti = Command::new("sh")
        .args(["send_mail.sh", &file])
        .status()
        .unwrap();
    assert!(noti.success());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Usage: {} <cmd> [args]", args[0]);
        process::exit(1);
    }

    let args = &args[1..];
    let proc = launch(&args);

    let proc_info = ProcInfo {
        command: args.join(" "),
        pid: proc.id(),
        status: wait(proc),
    };

    notify(proc_info);
}
