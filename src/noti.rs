use std::io::{self, BufWriter, Write};
use std::net::TcpStream;
use std::process::{Child, Command};
use std::{env, process};

mod proc_info;
use proc_info::ProcInfo;

fn launch(args: &[String]) -> io::Result<Child> {
    let cmd = &args[0];
    let args = &args[1..];
    Command::new(cmd).args(args).spawn()
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        eprintln!("Usage: {} <cmd> [args]", args[0]);
        process::exit(1);
    }

    let mut stream = TcpStream::connect("127.0.0.1:7878")?;

    let args = &args[1..];
    let mut proc = launch(&args)?;
    let proc_info = ProcInfo {
        command: args.join(" "),
        pid: proc.id(),
        is_term: true,
        status: proc.wait()?.to_string(),
    };

    let mut writer = BufWriter::new(&mut stream);
    let content = serde_json::to_string(&proc_info)?;
    writer.write(content.as_bytes())?;

    Ok(())
}
