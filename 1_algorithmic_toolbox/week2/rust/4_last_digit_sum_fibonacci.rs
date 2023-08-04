const MODULO: i64 = 10;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n = input.trim().parse::<i64>().unwrap();
    println!("{}", last_digit_sum_fibonacci(n));
}

fn last_digit_sum_fibonacci(n: i64) -> i64 {
    let mut cumulated_pisano = vec![0];
    let (mut a, mut b, mut sum) = (1, 1, 1);
    while !(a == 0 && b == 1) {
        cumulated_pisano.push(sum);
        (a, b) = (b, (a + b) % MODULO);
        sum = (sum + a) % MODULO;
    }

    let p_sz = cumulated_pisano.len() as i64;
    let (q, r) = (n / p_sz, n % p_sz);
    (q * cumulated_pisano[p_sz as usize - 1] + cumulated_pisano[r as usize]) % MODULO
}
