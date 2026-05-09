use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};

mod input;

use input::{InputStepResult, InputTiming, UInputDevice, execute_input_steps};

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
    let runtime = Arc::new(Mutex::new(DaemonRuntime::default()));
    eprintln!("alc-daemon listening on {}", socket.display());
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let runtime = Arc::clone(&runtime);
                thread::spawn(move || {
                    if let Err(error) = handle_client(stream, runtime) {
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

fn handle_client(mut stream: UnixStream, runtime: Arc<Mutex<DaemonRuntime>>) -> Result<()> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    while reader.read_line(&mut line)? > 0 {
        let response = match serde_json::from_str::<alc_core::DaemonEnvelope>(line.trim_end()) {
            Ok(envelope) => handle_request(&runtime, envelope),
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

#[derive(Default)]
struct DaemonRuntime {
    input: Option<UInputDevice>,
}

fn handle_request(
    runtime: &Arc<Mutex<DaemonRuntime>>,
    envelope: alc_core::DaemonEnvelope,
) -> alc_core::DaemonResponse {
    let id = envelope.id.clone();
    match envelope.request {
        alc_core::DaemonRequest::Input { steps } => {
            let result = runtime
                .lock()
                .map_err(|error| anyhow::anyhow!("runtime lock poisoned: {error}"))
                .and_then(|mut runtime| runtime.input(&steps));
            match result {
                Ok(steps) => alc_core::DaemonResponse {
                    id,
                    ok: true,
                    result: Some(serde_json::json!({
                        "acted": "input",
                        "device": "uinput",
                        "steps": steps,
                    })),
                    error: None,
                },
                Err(error) => alc_core::DaemonResponse {
                    id,
                    ok: false,
                    result: None,
                    error: Some(format!("{error:#}")),
                },
            }
        }
        _ => alc_core::handle_daemon_request(envelope),
    }
}

impl DaemonRuntime {
    fn input(&mut self, steps: &[alc_core::InputStep]) -> Result<Vec<InputStepResult>> {
        let timing = InputTiming::runtime();
        if self.input.is_none() {
            self.input = Some(UInputDevice::create(timing)?);
        }
        let input = self.input.as_mut().expect("input initialized");
        execute_input_steps(input, steps, timing)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct RecordingInput {
        events: Vec<String>,
    }

    impl input::InputDevice for RecordingInput {
        fn move_rel(&mut self, dx: i32, dy: i32) -> Result<()> {
            self.events.push(format!("move:{dx},{dy}"));
            Ok(())
        }

        fn scroll(&mut self, vertical: i32, horizontal: i32) -> Result<()> {
            self.events.push(format!("scroll:{vertical},{horizontal}"));
            Ok(())
        }

        fn key(&mut self, code: u16, down: bool) -> Result<()> {
            self.events.push(format!("key:{code}:{down}"));
            Ok(())
        }
    }

    #[test]
    fn input_executor_batches_move_click_and_hotkey() {
        let mut input = RecordingInput::default();
        let steps = vec![
            alc_core::InputStep::Move { dx: 2, dy: -3 },
            alc_core::InputStep::Click {
                button: None,
                x: None,
                y: None,
                delay_ms: None,
            },
            alc_core::InputStep::Hotkey {
                chord: "ctrl+l".to_string(),
            },
        ];
        let result = execute_input_steps(&mut input, &steps, InputTiming::zero()).unwrap();

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].action, "move");
        assert_eq!(
            input.events,
            vec![
                "move:2,-3",
                "key:272:true",
                "key:272:false",
                "key:29:true",
                "key:38:true",
                "key:38:false",
                "key:29:false",
            ]
        );
    }
}
