pub enum Direction {
    Left,
    Right,
    Center,
}

pub fn pad_string(s: String, len: usize, dir: Direction) -> String {
    let len = len - s.len();
    const PUSHCHAR: &str = "-";
    match dir {
        Direction::Left => {
            format!("{}{}", s, PUSHCHAR.repeat(len))
        }
        Direction::Right => {
            format!("{}{}", PUSHCHAR.repeat(len), s)
        }
        Direction::Center => {
            if len % 2 == 0 {
                let half = len / 2;
                let half_string = String::from(PUSHCHAR.repeat(half));
                format!("{}{}{}", half_string, s, half_string)
            } else {
                let first = (len + 1) / 2 - 1;
                let first_string = String::from(PUSHCHAR.repeat(first));
                let last = (len + 1) / 2;
                let last_string = String::from(PUSHCHAR.repeat(last));
                format!("{}{}{}", first_string, s, last_string)
            }
        }
    }
}
