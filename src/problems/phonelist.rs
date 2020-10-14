use std::hint::unreachable_unchecked;
use std::io::{self, BufRead, BufWriter, Write};
use std::str::FromStr;

pub trait UncheckedUnwrap<T> {
    unsafe fn unchecked_unwrap(self) -> T;
}

impl<T, E> UncheckedUnwrap<T> for Result<T, E> {
    #[inline]
    unsafe fn unchecked_unwrap(self) -> T {
        if let Ok(t) = self {
            t
        } else {
            unreachable_unchecked()
        }
    }
}

impl<T> UncheckedUnwrap<T> for Option<T> {
    #[inline]
    unsafe fn unchecked_unwrap(self) -> T {
        if let Some(t) = self {
            t
        } else {
            unreachable_unchecked()
        }
    }
}

fn get_digit_left(char: char) -> u8 {
    match char {
        '0' => 0b0000_0000,
        '1' => 0b0001_0000,
        '2' => 0b0010_0000,
        '3' => 0b0011_0000,
        '4' => 0b0100_0000,
        '5' => 0b0101_0000,
        '6' => 0b0110_0000,
        '7' => 0b0111_0000,
        '8' => 0b1000_0000,
        '9' => 0b1001_0000,
        _ => unsafe { unreachable_unchecked() },
    }
}

fn get_digit_right(char: char) -> u8 {
    match char {
        '0' => 0b0000_0000,
        '1' => 0b0000_0001,
        '2' => 0b0000_0010,
        '3' => 0b0000_0011,
        '4' => 0b0000_0100,
        '5' => 0b0000_0101,
        '6' => 0b0000_0110,
        '7' => 0b0000_0111,
        '8' => 0b0000_1000,
        '9' => 0b0000_1001,
        _ => unsafe { unreachable_unchecked() },
    }
}

#[derive(Debug)]
struct Number {
    length: u8,
    data: [u8; 5],
    _padding: u16,
}

impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let mut number = Self::with_length(1);

        number.data[0] = get_digit_left(unsafe { chars.next().unchecked_unwrap() });

        number.data[0] += match chars.next() {
            Some(c) => get_digit_right(c),
            None => {
                return Ok(number);
            }
        };

        number.data[1] = match chars.next() {
            Some(c) => get_digit_left(c),
            None => {
                number.length = 2;
                return Ok(number);
            }
        };

        number.data[1] += match chars.next() {
            Some(c) => get_digit_right(c),
            None => {
                number.length = 3;
                return Ok(number);
            }
        };

        number.data[2] = match chars.next() {
            Some(c) => get_digit_left(c),
            None => {
                number.length = 4;
                return Ok(number);
            }
        };

        number.data[2] += match chars.next() {
            Some(c) => get_digit_right(c),
            None => {
                number.length = 5;
                return Ok(number);
            }
        };

        number.data[3] = match chars.next() {
            Some(c) => get_digit_left(c),
            None => {
                number.length = 6;
                return Ok(number);
            }
        };

        number.data[3] += match chars.next() {
            Some(c) => get_digit_right(c),
            None => {
                number.length = 7;
                return Ok(number);
            }
        };

        number.data[4] = match chars.next() {
            Some(c) => get_digit_left(c),
            None => {
                number.length = 8;
                return Ok(number);
            }
        };

        number.data[4] += match chars.next() {
            Some(c) => get_digit_right(c),
            None => {
                number.length = 9;
                return Ok(number);
            }
        };

        number.length = 10;
        Ok(number)
    }
}

impl Number {
    pub fn new() -> Number {
        Number {
            length: 0,
            data: [0, 0, 0, 0, 0],
            _padding: 0,
        }
    }

    pub fn with_length(length: u8) -> Number {
        Number {
            length,
            data: [0, 0, 0, 0, 0],
            _padding: 0,
        }
    }

    pub fn mutual_contains(&self, other: &Self) -> bool {
        if self.length < other.length {
            return other.containing(&self);
        }

        self.containing(other)
    }

    pub fn contains(self, other: &Self) -> bool {
        if self.length < other.length {
            return false;
        }

        self.containing(other)
    }

    fn containing(&self, other: &Self) -> bool {
        let mut length = other.length;
        let mut mask = 0b1111_1111;

        if other.length % 2 == 1 {
            length -= 1;
            mask = 0b1111_0000;
        }

        length = length / 2;

        if length > 1 {
            length = length - 1;
        }

        let mut copy_data = self.data;

        copy_data[length as usize] &= mask;
        copy_data[..=length as usize] == other.data[..=length as usize]
    }
}

#[inline]
fn phonelist(lines: &Vec<String>) {
    let stdout = io::stdout();
    let mut writer_out = BufWriter::new(stdout);

    let groups_count: u8 = unsafe { lines[0].parse().unchecked_unwrap() };
    let mut current_group: u8 = 1;
    let mut lines_pointer: usize = 1;

    loop {
        let group_target =
            unsafe { lines[lines_pointer].parse::<usize>().unchecked_unwrap() + lines_pointer };

        let mut group_pointer_checked = lines_pointer + 1;
        let mut group_pointer_checker = lines_pointer + 2;

        loop {
            if unsafe {
                lines[group_pointer_checked]
                    .parse::<Number>()
                    .unchecked_unwrap()
                    .mutual_contains(
                        &lines[group_pointer_checker]
                            .parse::<Number>()
                            .unchecked_unwrap(),
                    )
            } {
                unsafe { writeln!(writer_out, "NO").unchecked_unwrap() };
                break;
            }

            if group_pointer_checker == group_target {
                if group_pointer_checked == group_target - 1 {
                    unsafe { writeln!(writer_out, "YES").unchecked_unwrap() };
                    break;
                } else {
                    group_pointer_checked += 1;
                    group_pointer_checker = group_pointer_checked + 1;
                }
            } else {
                group_pointer_checker += 1;
            }
        }

        if current_group == groups_count {
            break;
        }

        current_group += 1;
        lines_pointer = group_target + 1;
    }
}

// https://open.kattis.com/problems/phonelist
fn main() {
    let lines: Vec<String> = {
        let stdin = io::stdin();

        stdin
            .lock()
            .lines()
            .map(|line| unsafe { line.unchecked_unwrap() })
            .collect()
    };

    phonelist(&lines);
}

#[cfg(test)]
mod tests {
    use crate::problems::phonelist::{phonelist, Number};

    #[test]
    fn from_str_test() {
        assert_eq!(
            "Number { length: 2, data: [6, 0, 0, 0, 0], _padding: 0 }",
            format!("{:?}", "06".parse::<Number>().unwrap())
        );

        assert_eq!(
            "Number { length: 3, data: [6, 96, 0, 0, 0], _padding: 0 }",
            format!("{:?}", "066".parse::<Number>().unwrap())
        );

        assert_eq!(
            "Number { length: 10, data: [150, 69, 50, 152, 103], _padding: 0 }",
            format!("{:?}", "9645329867".parse::<Number>().unwrap())
        );
    }

    #[test]
    fn mutual_contains_test() {
        assert_eq!(
            "0".parse::<Number>()
                .unwrap()
                .mutual_contains(&"6".parse().unwrap()),
            false
        );

        assert_eq!(
            "0".parse::<Number>()
                .unwrap()
                .mutual_contains(&"06".parse().unwrap()),
            true
        );

        assert_eq!(
            "5".parse::<Number>()
                .unwrap()
                .mutual_contains(&"5".parse().unwrap()),
            true
        );

        assert_eq!(
            "06".parse::<Number>()
                .unwrap()
                .mutual_contains(&"06".parse().unwrap()),
            true
        );

        assert_eq!(
            "06100"
                .parse::<Number>()
                .unwrap()
                .mutual_contains(&"061".parse().unwrap()),
            true
        );

        assert_eq!(
            "0123456789"
                .parse::<Number>()
                .unwrap()
                .mutual_contains(&"0123456789".parse().unwrap()),
            true
        );
    }

    #[test]
    fn phonelist_test() {
        let lines: Vec<String> = "4
3
911
97625999
91125426
5
113
12340
123440
12345
98346
3
911
97625999
91125426
5
113
12340
123440
12345
98346"
            .lines()
            .map(|line| line.to_string())
            .collect();

        phonelist(&lines);
    }
}
