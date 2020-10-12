use std::io;

fn abc() {
    let mut numbers = String::with_capacity(6);
    io::stdin().read_line(&mut numbers).unwrap();
    let mut numbers: Vec<u8> = numbers.split_whitespace().map(|word| word.parse().unwrap())
        .collect();
    numbers.sort_unstable();

    let mut order = String::with_capacity(4);
    io::stdin().read_line(&mut order).unwrap();
    let order: Vec<char> = order.chars().collect();

    match order[0] {
        'A' => print!("{} ", numbers[0]),
        'B' => print!("{} ", numbers[1]),
        'C' => print!("{} ", numbers[2]),
        _ => return,
    }
    match order[1] {
        'A' => print!("{} ", numbers[0]),
        'B' => print!("{} ", numbers[1]),
        'C' => print!("{} ", numbers[2]),
        _ => return,
    }
    match order[2] {
        'A' => print!("{}", numbers[0]),
        'B' => print!("{}", numbers[1]),
        'C' => print!("{}", numbers[2]),
        _ => return,
    }
}
