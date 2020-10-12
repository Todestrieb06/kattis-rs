use std::io;

fn anewalphabet() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: String = input.parse().unwrap();
    let mut output: String = String::with_capacity(input.len() * 2);

    for c in input.to_ascii_lowercase().chars() {
        match c {
            'a' => {
                output.push('@');
            }
            'b' => {
                output.push('8');
            }
            'c' => {
                output.push('(');
            }
            'd' => {
                output.push_str("|)");
            }
            'e' => {
                output.push('3');
            }
            'f' => {
                output.push('#');
            }
            'g' => {
                output.push('6');
            }
            'h' => {
                output.push_str("[-]");
            }
            'i' => {
                output.push('|');
            }
            'j' => {
                output.push_str("_|");
            }
            'k' => {
                output.push_str("|<");
            }
            'l' => {
                output.push('1');
            }
            'm' => {
                output.push_str("[]\\/[]");
            }
            'n' => {
                output.push_str("[]\\[]");
            }
            'o' => {
                output.push('0');
            }
            'p' => {
                output.push_str("|D");
            }
            'q' => {
                output.push_str("(,)");
            }
            'r' => {
                output.push_str("|Z");
            }
            's' => {
                output.push('$');
            }
            't' => {
                output.push_str("']['");
            }
            'u' => {
                output.push_str("|_|");
            }
            'v' => {
                output.push_str("\\/");
            }
            'w' => {
                output.push_str("\\/\\/");
            }
            'x' => {
                output.push_str("}{");
            }
            'y' => {
                output.push_str("`/");
            }
            'z' => {
                output.push('2');
            }
            _ => {
                output.push(c);
            }
        };
    }

    println!("{}", output);
}
