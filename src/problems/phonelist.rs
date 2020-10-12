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

#[inline]
fn get_digit(char: char) -> u8 {
    match char {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => unsafe { unreachable_unchecked() },
    }
}

#[derive(Debug)]
struct Number {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    g: u8,
    h: u8,
    i: u8,
    j: u8,
}

impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut number = Self::new();
        let mut chars = s.chars();

        number.a = get_digit(unsafe { chars.next().unchecked_unwrap() });

        let c = match chars.next() {
            Some(c) => c,
            None => {
                number.a |= 0b1000_0000;
                return Ok(number);
            }
        };
        number.b = get_digit(c);

        let c = match chars.next() {
            Some(c) => c,
            None => {
                number.b |= 0b1000_0000;
                return Ok(number);
            }
        };
        number.c = get_digit(c);

        let c = match chars.next() {
            Some(c) => c,
            None => {
                number.c |= 0b1000_0000;
                return Ok(number);
            }
        };
        number.d = get_digit(c);

        let c = match chars.next() {
            Some(c) => c,
            None => {
                number.d |= 0b1000_0000;
                return Ok(number);
            }
        };
        number.e = get_digit(c);

        let c = match chars.next() {
            Some(c) => c,
            None => {
                number.e |= 0b1000_0000;
                return Ok(number);
            }
        };
        number.f = get_digit(c);

        let c = match chars.next() {
            Some(c) => c,
            None => {
                number.f |= 0b1000_0000;
                return Ok(number);
            }
        };
        number.g = get_digit(c);

        let c = match chars.next() {
            Some(c) => c,
            None => {
                number.g |= 0b1000_0000;
                return Ok(number);
            }
        };
        number.h = get_digit(c);

        let c = match chars.next() {
            Some(c) => c,
            None => {
                number.h |= 0b1000_0000;
                return Ok(number);
            }
        };
        number.i = get_digit(c);

        let c = match chars.next() {
            Some(c) => c,
            None => {
                number.i |= 0b1000_0000;
                return Ok(number);
            }
        };
        number.j = get_digit(c);

        Ok(number)
    }
}

impl Number {
    fn new() -> Number {
        Number {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            g: 0,
            h: 0,
            i: 0,
            j: 0,
        }
    }

    fn contains(self, other: &Self) -> bool {
        if self.a << 1 == other.a << 1 {
            if other.a >> 7 == 1 {
                return true;
            }
        } else {
            return false;
        }

        if self.b << 1 == other.b << 1 {
            if other.b >> 7 == 1 {
                return true;
            }
        } else {
            return false;
        }

        if self.c << 1 == other.c << 1 {
            if other.c >> 7 == 1 {
                return true;
            }
        } else {
            return false;
        }

        if self.d << 1 == other.d << 1 {
            if other.d >> 7 == 1 {
                return true;
            }
        } else {
            return false;
        }

        if self.e << 1 == other.e << 1 {
            if (other.e >> 7) == 1 {
                return true;
            }
        } else {
            return false;
        }

        if self.f << 1 == other.f << 1 {
            if (other.f >> 7) == 1 {
                return true;
            }
        } else {
            return false;
        }

        if self.g << 1 == other.g << 1 {
            if (other.g >> 7) == 1 {
                return true;
            }
        } else {
            return false;
        }

        if self.h << 1 == other.h << 1 {
            if (other.h >> 7) == 1 {
                return true;
            }
        } else {
            return false;
        }

        if self.i << 1 == other.i << 1 {
            if (other.i >> 7) == 1 {
                return true;
            }
        } else {
            return false;
        }

        if self.j << 1 == other.j << 1 {
            return true;
        }

        false
    }
}

fn phonelist(lines: &Vec<String>) {
    let stdout = io::stdout();
    let mut writer_out = BufWriter::new(stdout);

    let groups_count: u8 = unsafe { lines[0].parse().unchecked_unwrap() };
    let mut current_group: u8 = 1;
    let mut lines_pointer: usize = 1;

    loop {
        let numbers_count: usize =
            unsafe { lines[lines_pointer].parse::<usize>().unchecked_unwrap() };
        let mut group_pointer_checker: usize = 0;
        let mut group_pointer_checked: usize = 1;

        loop {
            if lines[group_pointer_checked].contains(&lines[group_pointer_checker]) {
                unsafe { writeln!(writer_out, "NO").unchecked_unwrap() };
                break;
            }

            if group_pointer_checked == numbers_count {
                if group_pointer_checker == numbers_count {
                    unsafe { writeln!(writer_out, "YES").unchecked_unwrap() };
                    break;
                } else {
                    group_pointer_checker += lines_pointer;
                    group_pointer_checked = group_pointer_checked + 1;
                }
            } else {
                group_pointer_checked += if group_pointer_checked == group_pointer_checker {
                    2
                } else {
                    1
                }
            }
        }

        if current_group == groups_count {
            break;
        }

        current_group += 1;
        lines_pointer += numbers_count;
    }
}

// https://open.kattis.com/problems/phonelist
fn main() {
    let stdin = io::stdin();

    let lines: Vec<String> = stdin
        .lock()
        .lines()
        .map(|line| unsafe { line.unchecked_unwrap() })
        .collect();

    phonelist(&lines);
}

#[cfg(test)]
mod tests {
    use crate::problems::phonelist::{phonelist, Number};

    #[test]
    fn from_str_test() {
        assert_eq!(
            "Number { a: 0, b: -6, c: 0, d: 0, e: 0, f: 0, g: 0, h: 0, i: 0, j: 0 }",
            format!("{:?}", "06".parse::<Number>().unwrap())
        );

        assert_eq!(
            "Number { a: 9, b: 6, c: 4, d: 5, e: 3, f: 2, g: 9, h: 8, i: 6, j: 7 }",
            format!("{:?}", "9645329867".parse::<Number>().unwrap())
        );
    }

    #[test]
    fn contains_test() {
        assert_eq!(
            "06".parse::<Number>()
                .unwrap()
                .contains(&"06".parse().unwrap()),
            true
        );
        assert_eq!(
            "06100"
                .parse::<Number>()
                .unwrap()
                .contains(&"061".parse().unwrap()),
            true
        );
        assert_eq!(
            "0123456789"
                .parse::<Number>()
                .unwrap()
                .contains(&"0123456789".parse().unwrap()),
            true
        );
    }

    #[test]
    fn phonelist_test() {
        let lines: Vec<String> = "2
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
