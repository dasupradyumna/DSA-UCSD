fn main() {
    let mut input = String::new();
    read_lines(&mut input, 1);
    let mut input = input.split_whitespace().map(|i| i.parse::<i64>().unwrap());
    // main logic
}

fn read_lines(input: &mut String, n_lines: i32) {
    for _ in 0..n_lines {
        std::io::stdin().read_line(input).unwrap();
    }
}
