use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_whitespace().map(|i| i.parse::<i32>().unwrap());

    let a = input.next().unwrap();
    let b = input.next().unwrap();

    println!("{}", a + b);
}
