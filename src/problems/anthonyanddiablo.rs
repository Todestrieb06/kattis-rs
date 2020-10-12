use std::io;
use std::f32::consts::PI;

fn anthonyanddiablo() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: Vec<f32> = input.split_whitespace().map(|word| word.parse().unwrap())
        .collect();
    let available = input[1] / (PI * 2.0);
    let size = PI * (available.powf(2.0));

    if size >= input[0] {
        println!("Diablo is happy!");
    } else {
        println!("Need more materials!");
    }
}
