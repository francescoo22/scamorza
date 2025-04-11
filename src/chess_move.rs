use regex::Regex;
use std::ops::Not;

#[derive(Copy, Clone, Debug)]
pub struct Move {
    pub from: (usize, usize),
    pub to: (usize, usize),
}

impl Move {
    pub fn from_uci_string(s: &str) -> Self {
        let re = Regex::new(r"^[a-h][1-8][a-h][1-8]$").unwrap();
        if !re.is_match(s) {
            panic!("Invalid UCI string");
        }

        let bytes = s.as_bytes();
        let from_col = (b'h' - bytes[0]) as usize;
        let from_row = (bytes[1] - b'1') as usize;
        let to_col = (b'h' - bytes[2]) as usize;
        let to_row = (bytes[3] - b'1') as usize;
        Move {
            from: (from_row, from_col),
            to: (to_row, to_col),
        }
    }

    pub fn to_uci_string(&self) -> String {
        let from_row = char::from(self.from.0 as u8 + b'1');
        let from_col = char::from(b'h' - self.from.1 as u8);
        let to_row = char::from(self.to.0 as u8 + b'1');
        let to_col = char::from(b'h' - self.to.1 as u8);
        format!("{}{}{}{}", from_col, from_row, to_col, to_row)
    }
}

impl Not for Move {
    type Output = Move;

    fn not(self) -> Self {
        Move {
            from: self.to,
            to: self.from,
        }
    }
}
