pub enum Direction {
    Left,
    Right,
    Center,
}

pub fn pad_string(s: String, len: usize, dir: Direction) -> String {
    let len = len - s.len();
    match dir {
        Direction::Left => "".to_string(),
        Direction::Right => "".to_string(),
        Direction::Center => {
            if len % 2 == 0 {
                let half = len / 2;
                let mut half_string = String::new();
                while half_string.len() < half {
                    half_string.push_str(" ");
                }
                format!("{}{}{}", half_string, s, half_string)
            } else {
                let first = (len + 1) / 2;
                let mut first_string = String::new();
                while first_string.len() < first {
                    first_string.push_str(" ");
                }
                let last = (len + 1) / 2 - 1;
                let mut last_string = String::new();
                while last_string.len() < last {
                    last_string.push_str(" ");
                }
                format!("{}{}{}", first_string, s, last_string)
            }
        }
    }
}
