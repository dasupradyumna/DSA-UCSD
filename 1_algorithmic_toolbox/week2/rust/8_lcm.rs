fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_whitespace().map(|i| i.parse::<i32>().unwrap());

    let a = input.next().unwrap();
    let b = input.next().unwrap();

    println!("{}", lcm(a, b));
}

fn lcm(mut a: i32, mut b: i32) -> i64 {
    let lcm = a as i64 * b as i64;
    while b > 0 {
        (a, b) = (b, a % b);
    }

    lcm / a as i64
}
