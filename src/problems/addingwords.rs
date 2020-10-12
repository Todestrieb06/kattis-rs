use std::hint::unreachable_unchecked;
use std::io::{self, BufRead, BufWriter, Write};
use std::collections::HashMap;

fn addingwords() {
    let stdin = io::stdin();
    let stdin: Vec<String> = stdin.lock().lines().map(|l| l.unwrap()).collect();
    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout);
    let mut hashmap: HashMap<&str, i16> = HashMap::with_capacity(16);

    for line in stdin.iter() {
        let words: Vec<&str> = line.split_whitespace().collect();

        match words[0] {
            "def" => {
                hashmap.insert(words[1], words[2].parse().unwrap());
            }
            "calc" => 'a: loop {
                let mut value_total: i16 = match hashmap.get(words[1]) {
                    Some(v) => *v,
                    None => {
                        writeln!(output, "{} unknown", &line[5..line.len()]).unwrap();
                        break 'a;
                    }
                };
                let x = words.len() - 1;

                for i in (2..x).step_by(2) {
                    match hashmap.get(words[i + 1]) {
                        Some(&v) => {
                            if words[i] == "+" {
                                value_total += v;
                            } else {
                                value_total -= v;
                            }
                        }
                        None => {
                            writeln!(output, "{} unknown", &line[5..line.len()]).unwrap();
                            break 'a;
                        }
                    };
                }
                let v = match hashmap.iter().find(|&(_k, v)| v == &value_total) {
                    Some((k, _)) => k,
                    None => "unknown",
                };
                writeln!(output, "{} {}", &line[5..line.len()], v).unwrap();
                break 'a;
            },
            "clear" => {
                hashmap.clear();
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }
}
