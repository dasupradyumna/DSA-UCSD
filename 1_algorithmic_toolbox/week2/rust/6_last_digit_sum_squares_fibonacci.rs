const MODULO: i64 = 10;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<i64>().unwrap();
    println!("{}", last_digit_sum_squares_fibonacci(n));
}

fn last_digit_sum_squares_fibonacci(n: i64) -> i64 {
    let mut pisano10 = vec![0];
    let (mut a, mut b) = (1, 1);
    while !(a == 0 && b == 1) {
        pisano10.push(a);
        (a, b) = (b, (a + b) % MODULO);
    }

    let p_sz = pisano10.len();
    (pisano10[n as usize % p_sz] * pisano10[(n as usize + 1) % p_sz]) % MODULO
}
