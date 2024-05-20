use std::process::Stdio;

use tokio::io::{AsyncBufReadExt, BufReader, Lines};
use tokio::process::{ChildStdout, Command};

use hid_client_stdout::Messages;
use iced_futures::subscription::{self, Subscription};

pub enum State {
    Starting,
    Ready {
        reader: Lines<BufReader<ChildStdout>>,
        ready: bool,
    },
}

pub fn hid_worker(serial: Option<String>) -> Subscription<crate::Message> {
    use crate::Message;
    struct HidWorker;

    if serial.is_none() {
        println!("HIDIO: No serial specified");
        return Subscription::none();
    } else {
        subscription::unfold(
            std::any::TypeId::of::<HidWorker>(),
            State::Starting,
            move |state| {
                let serial = serial.clone();
                async move {
                    match state {
                        State::Starting => {
                            let mut cmd = match tokio::process::Command::new(
                                "/home/zion/programming/rust/hidiokb/target/release/hidiokb",
                            )
                            .arg("subscribe")
                            .arg("-s")
                            .arg(serial.as_ref().unwrap())
                            .stdout(Stdio::piped())
                            .spawn()
                            {
                                Ok(cmd) => cmd,
                                Err(_) => {
                                    return (Message::NAN, State::Starting);
                                }
                            };
                            let stdout = BufReader::new(cmd.stdout.take().unwrap()).lines();
                            return (
                                Message::NAN,
                                State::Ready {
                                    reader: stdout,
                                    ready: false,
                                },
                            );
                        }
                        State::Ready { mut reader, ready } => {
                            let mut ready = ready;
                            match reader.next_line().await {
                                Ok(Some(line)) => {
                                    let line = line.trim().to_owned();
                                    if ready {
                                        let msg = match Messages::try_from(line.as_str()) {
                                            Ok(msg) => msg,
                                            Err(_) => {
                                                println!("Decode Error: {}", line);
                                                return (
                                                    Message::NAN,
                                                    State::Ready { reader, ready },
                                                );
                                            }
                                        };
                                        println!("TRANSPOSE: {} *** {:?}", line, msg);
                                        (Message::Hid(msg), State::Ready { reader, ready })
                                    } else {
                                        if line == "READY" {
                                            ready = true;
                                            println!("READY: {}", line);
                                        } else {
                                            println!("!READY: {}", line);
                                        }
                                        (Message::NAN, State::Ready { reader, ready })
                                    }
                                }
                                Ok(None) => (Message::NAN, State::Ready { reader, ready }),
                                Err(_) => (Message::NAN, State::Ready { reader, ready }),
                            }
                        }
                    }
                }
            },
        )
    }
}
