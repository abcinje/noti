use std::env;
use std::fs;
use std::process;
use std::thread;

#[derive(Default)]
struct ProcInfo {
    command: Option<String>,
    pid: Option<u32>,
    status: Option<process::ExitStatus>,
}

fn launch(args: &[String]) -> process::Child {
    let cmd = &args[0];
    let args = &args[1..];

    process::Command::new(cmd)
                     .args(args)
                     .spawn()
                     .expect("Failed to execute command")
}

fn wait(mut proc: process::Child) -> process::ExitStatus {
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
    msg += &format!("Command: {}\n", proc_info.command.unwrap());
    msg += &format!("PID: {}\n", proc_info.pid.unwrap());
    msg += &format!("Status: {}\n", proc_info.status.unwrap());

    let file = format!("proc/{}", proc_info.pid.unwrap().to_string());
    fs::write(&file, msg).expect("Failed to write message");

    let noti = process::Command::new("sh")
                                .args(["send_mail.sh", &file])
                                .status()
                                .unwrap();
    assert!(noti.success());
}

fn main() {
    let mut proc_info = ProcInfo::default();

    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Usage: {} <cmd> [args]", args[0]);
        process::exit(1);
    }

    let args = &args[1..];
    proc_info.command = Some(args.join(" "));

    let proc = launch(&args);
    proc_info.pid = Some(proc.id());

    let status = wait(proc);
    proc_info.status = Some(status);

    notify(proc_info);
}
