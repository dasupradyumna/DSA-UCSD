fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<i32>().unwrap();

    println!("{}", (n / 10) + (n % 10 >= 5) as i32 + (n % 5));
}
