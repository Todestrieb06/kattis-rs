use std::io;

fn main() {
    let mut input: String = String::with_capacity(15);
    io::stdin().read_line(&mut input).unwrap();

    println!("{} {} {}", input, input, input);
}
