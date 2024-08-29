use std::io::{self, BufReader, Read};
use std::net::{TcpListener, TcpStream};
use std::process::Command;
use std::fs;

mod proc_info;
use proc_info::ProcInfo;

fn notify(proc_info: ProcInfo) {
    fs::create_dir_all("proc").expect("Failed to create directory");

    let mut msg = String::new();
    msg += &format!("Command: {}\n", proc_info.command);
    msg += &format!("PID: {}\n", proc_info.pid);
    msg += &format!("Status: {}\n", proc_info.status);

    let file = format!("proc/{}", proc_info.pid.to_string());
    fs::write(&file, msg).expect("Failed to write message");

    let noti = Command::new("sh")
        .args(["scripts/send_mail.sh", &file])
        .status()
        .unwrap();
    assert!(noti.success());
}

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(&mut stream);
    let mut content = String::new();
    reader.read_to_string(&mut content).unwrap();

    let proc_info: ProcInfo = serde_json::from_str(&content).unwrap();
    notify(proc_info);
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    for stream in listener.incoming() {
        let stream = stream?;
        handle_connection(stream);
    }

    Ok(())
}
