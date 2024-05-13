use std::io::BufRead;

pub fn exec_stream<P: AsRef<std::path::Path>>(binary: P, args: Vec<&'static str>) -> iced::futures::Stream {
    use std::process::{Command, Stdio};
    let mut cmd = Command::new(binary.as_ref())
        .args(&args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    {
        let stdout = cmd.stdout.as_mut().unwrap();
        let stdout_reader = std::io::BufReader::new(stdout);
        let stdout_lines = stdout_reader.lines();

        for line in stdout_lines {
            println!("Read: {:?}", line);
        }
    }
    let out = cmd.stdout.unwrap();
    out.
}
