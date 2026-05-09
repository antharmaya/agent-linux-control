use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::thread;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "alc-daemon",
    version,
    about = "Rust agent-linux-control daemon prototype"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Serve newline-delimited JSON requests on a Unix socket.
    Serve {
        #[arg(long, default_value = "/tmp/agent-linux-control.sock")]
        socket: PathBuf,
    },
    /// Send one JSON request to the daemon and print one JSON response.
    Call {
        #[arg(long, default_value = "/tmp/agent-linux-control.sock")]
        socket: PathBuf,
        request: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Serve { socket } => serve(socket),
        Command::Call { socket, request } => call(socket, &request),
    }
}

fn serve(socket: PathBuf) -> Result<()> {
    if socket.exists() {
        fs::remove_file(&socket)
            .with_context(|| format!("remove stale socket {}", socket.display()))?;
    }
    let listener =
        UnixListener::bind(&socket).with_context(|| format!("bind socket {}", socket.display()))?;
    eprintln!("alc-daemon listening on {}", socket.display());
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(error) = handle_client(stream) {
                        eprintln!("client error: {error:#}");
                    }
                });
            }
            Err(error) => eprintln!("accept error: {error:#}"),
        }
    }
    Ok(())
}

fn call(socket: PathBuf, request: &str) -> Result<()> {
    let mut stream = UnixStream::connect(&socket)
        .with_context(|| format!("connect socket {}", socket.display()))?;
    stream.write_all(request.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;
    let mut line = String::new();
    if BufReader::new(stream).read_line(&mut line)? == 0 {
        bail!("daemon closed without a response");
    }
    print!("{line}");
    Ok(())
}

fn handle_client(mut stream: UnixStream) -> Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        let response = match serde_json::from_str::<alc_core::DaemonEnvelope>(line.trim_end()) {
            Ok(envelope) => alc_core::handle_daemon_request(envelope),
            Err(error) => alc_core::DaemonResponse {
                id: None,
                ok: false,
                result: None,
                error: Some(error.to_string()),
            },
        };
        serde_json::to_writer(&mut stream, &response)?;
        stream.write_all(b"\n")?;
        stream.flush()?;
        line.clear();
    }
    Ok(())
}
