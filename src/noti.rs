use std::io::{self, BufWriter, Write};
use std::net::TcpStream;
use std::process::{Child, Command, ExitStatus};
use std::{env, process, thread};

mod proc_info;
use proc_info::ProcInfo;

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

fn main() -> io::Result<()> {
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
        status: wait(proc).to_string(),
    };

    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    let mut writer = BufWriter::new(&mut stream);
    let content = serde_json::to_string(&proc_info)?;
    writer.write(content.as_bytes())?;

    Ok(())
}
