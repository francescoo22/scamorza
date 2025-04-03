use regex::Regex;

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
        let from_col = (bytes[0] - b'a') as usize;
        let from_row = (bytes[1] - b'1') as usize;
        let to_col = (bytes[2] - b'a') as usize;
        let to_row = (bytes[3] - b'1') as usize;
        Move {
            from: (from_row, from_col),
            to: (to_row, to_col),
        }
    }

    pub fn to_uci_string(&self) -> String {
        let from_row = char::from(self.from.0 as u8 + b'1');
        let from_col = char::from(self.from.1 as u8 + b'a');
        let to_row = char::from(self.to.0 as u8 + b'1');
        let to_col = char::from(self.to.1 as u8 + b'a');
        format!("{}{}{}{}", from_col, from_row, to_col, to_row)
    }
}
