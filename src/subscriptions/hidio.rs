use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::process::{ChildStdout, Stdio};

use chrono::Datelike;
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
            let mut last_line: Option<chrono::NaiveDateTime> = None;

            loop {
                match &mut state {
                    State::Starting => {
                        let cmd = match std::process::Command::new("/home/zion/programming/rust/hid-io-ergoone/target/release/hid-io-ergoone")
                            .arg("000001")
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
                        println!("doing");
                        let lines = reader.lines();
                        for line in lines {
                            let mut line = line.unwrap();
                            let year = chrono::Utc::now().year();
                            let line_str = line[0..15].to_string();
                            let date_str = format!("{} {}", year.to_string(), line_str);
                            println!("date_str: \"{}\"", date_str);
                            let date = match chrono::NaiveDateTime::parse_from_str(
                                &date_str,
                                "%Y %b %d %H:%M:%S",
                            ) {
                                Ok(date) => date,
                                Err(e) => {
                                    println!("Error: {}", e);
                                    continue;
                                }
                            };
                            if date >= last_line.unwrap_or(date) {
                                println!("newer date");
                                last_line = Some(date);
                            } else {
                                println!("older date");
                                continue;
                            }
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
                                        continue;
                                    }
                                    Err(e) => {
                                        println!("ERROR: {}", e);
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
