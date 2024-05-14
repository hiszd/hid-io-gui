use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{ChildStdout, Stdio};

use hid_client_stdout::Messages;
use iced_futures::futures::sink::SinkExt;
use iced_futures::subscription::{self, Subscription};

pub enum State {
    Starting,
    Ready(BufReader<ChildStdout>),
}

pub fn hid_worker() -> Subscription<crate::Message> {
    subscription::channel("hidio", 100, |mut output| async move {
        let mut state = State::Starting;

        loop {
            match &mut state {
                State::Starting => {
                    let cmd = match std::process::Command::new("journalctl")
                        .arg("-fu")
                        .arg("hid-io")
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
                                Error::new(ErrorKind::Other, "Could not capture standard output.")
                            })
                            .unwrap(),
                    );
                    state = State::Ready(stdout);
                }
                State::Ready(reader) => {
                    let lines = reader.lines();
                    for line in lines {
                        let line = line.unwrap();
                        let msg = match Messages::try_from(line.as_str()) {
                            Ok(msg) => msg,
                            Err(_) => {
                                println!("Error: {}", line);
                                return;
                            }
                        };
                        output.send(crate::Message::try_from(msg).unwrap()).await;
                    }
                    continue;
                }
            }
        }
    })
}
