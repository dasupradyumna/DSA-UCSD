fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<i32>().unwrap();
    println!("{}", fibonacci(n));
}

fn fibonacci(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }

    let (mut a, mut b) = (0, 1);
    for _ in 0..n - 1 {
        (a, b) = (b, (a + b) % 10);
    }
    b
}
