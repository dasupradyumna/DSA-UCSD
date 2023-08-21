fn main() {
    let mut buffer = String::new();
    for _ in 0..2 {
        std::io::stdin().read_line(&mut buffer).unwrap();
    }
    let mut input = buffer.split_whitespace();
    input.next(); // skip length of sequence input

    let mut sequence = input.collect::<Vec<_>>();
    sequence.sort_by(|s1, s2| (s2.to_string() + s1).cmp(&(s1.to_string() + s2)));

    println!("{}", sequence[..].join(""));
}
