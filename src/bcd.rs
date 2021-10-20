use std::convert::From;

pub struct BCD {
    pub bcd: Vec<i8>,
}

impl From<String> for BCD {
    fn from(s: String) -> Self {
        BCD {
            bcd: s
                .chars()
                .rev()
                .map(|x| x.to_digit(10).expect("Invalid string") as i8)
                .collect(),
        }
    }
}

impl BCD {
    fn print(&self) {
        for i in self.bcd.iter().rev() {
            print!("{}", i);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::bcd::*;

    #[test]
    fn test_from_str() {
        assert!(BCD::from(String::from("123")).bcd == vec![3, 2, 1])
    }
}
