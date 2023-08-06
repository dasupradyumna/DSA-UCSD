fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_whitespace().map(|i| i.parse::<i32>().unwrap());

    let a = input.next().unwrap();
    let b = input.next().unwrap();

    println!("{}", gcd(a, b));
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}
