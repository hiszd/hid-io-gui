use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{ChildStdout, Stdio};

use hid_client_stdout::Messages;
use iced_futures::futures::sink::SinkExt;
use iced_futures::subscription::{self, Subscription};

pub enum State {
    Starting,
    Ready(BufReader<ChildStdout>),
}

pub fn hid_worker() -> Subscription<hid_client_stdout::Messages> {
    struct HidWorker;

    subscription::channel(
        std::any::TypeId::of::<HidWorker>(),
        100,
        |mut output| async move {
            let mut state = State::Starting;
            let mut ready = false;

            loop {
                match &mut state {
                    State::Starting => {
                        let cmd = match std::process::Command::new("journalctl")
                            .arg("-fu")
                            .arg("hid-io-ergoone")
                            .stdout(Stdio::piped())
                            .spawn()
                        {
                            Ok(cmd) => cmd,
                            Err(_) => {
                                continue;
                            }
                        };
                        let stdout = BufReader::new(
                            cmd.stdout
                                .ok_or_else(|| {
                                    Error::new(
                                        ErrorKind::Other,
                                        "Could not capture standard output.",
                                    )
                                })
                                .unwrap(),
                        );
                        state = State::Ready(stdout);
                    }
                    State::Ready(reader) => {
                        let lines = reader.lines();
                        for line in lines {
                            let mut line = line.unwrap();
                            let end = line.find("]:");
                            if end.is_some() {
                                line = line[end.unwrap() + 3..].to_string();
                            }
                            if ready {
                                let msg = match Messages::try_from(line.as_str()) {
                                    Ok(msg) => msg,
                                    Err(_) => {
                                        println!("Error: {}", line);
                                        continue;
                                    }
                                };
                                println!("TRANSPOSE: {} *** {:?}", line, msg);
                                match output.send(msg).await {
                                    Ok(_) => {
                                        println!("sent");
                                    }
                                    Err(_) => {
                                        continue;
                                    }
                                }
                            } else {
                                if line == "READY" {
                                    ready = true;
                                    println!("READY: {}", line);
                                } else {
                                    println!("!READY: {}", line);
                                }
                            }
                        }
                    }
                }
            }
        },
    )
}
