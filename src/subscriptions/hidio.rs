use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{ChildStdout, Stdio};

use hid_client_stdout::Messages;
use iced_futures::subscription::{self, Subscription};

pub enum State {
    Starting,
    Ready {
        reader: BufReader<ChildStdout>,
        ready: bool,
    },
}

pub fn hid_worker() -> Subscription<crate::Message> {
    use crate::Message;
    struct HidWorker;

    subscription::unfold(
        std::any::TypeId::of::<HidWorker>(),
        State::Starting,
        move |state| async move {
            match state {
                State::Starting => {
                    let cmd = match std::process::Command::new(
                        "/home/zion/programming/rust/hid-io-ergoone/target/release/hid-io-ergoone",
                    )
                    .arg("000001")
                    .stdout(Stdio::piped())
                    .spawn()
                    {
                        Ok(cmd) => cmd,
                        Err(_) => {
                            return (Message::NAN, State::Starting);
                        }
                    };
                    let stdout = BufReader::new(
                        cmd.stdout
                            .ok_or_else(|| {
                                Error::new(ErrorKind::Other, "Could not capture standard output.")
                            })
                            .unwrap(),
                    );
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
                    println!("doing");
                    let mut line = String::new();
                    match reader.read_line(&mut line) {
                        Ok(_) => {
                            let line = line.trim().to_owned();
                            if ready {
                                let msg = match Messages::try_from(line.as_str()) {
                                    Ok(msg) => msg,
                                    Err(_) => {
                                        println!("Error: {}", line);
                                        return (Message::NAN, State::Ready { reader, ready });
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
                        Err(_) => (Message::NAN, State::Ready { reader, ready }),
                    }
                }
            }
        },
    )
}
