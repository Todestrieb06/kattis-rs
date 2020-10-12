use std::io;
use std::io::BufRead;

fn aaah() {
    let stdin = io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|line| line.unwrap())
        .map(|line| line.parse().unwrap()).collect();

    if lines[0].len() < lines[1].len() {
        println!("no");
    } else {
        println!("go")
    }
}
