const MODULO: i64 = 10;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_whitespace().map(|i| i.parse::<i64>().unwrap());

    let m = input.next().unwrap();
    let n = input.next().unwrap();
    println!("{}", last_digit_partial_sum_fibonacci(m, n));
}

fn last_digit_partial_sum_fibonacci(m: i64, n: i64) -> i64 {
    let mut cp = vec![0]; // cumulated pisano modulos
    let (mut a, mut b, mut sum) = (1, 1, 1);
    while !(a == 0 && b == 1) {
        cp.push(sum);
        (a, b) = (b, (a + b) % MODULO);
        sum = (sum + a) % MODULO;
    }

    if m == 0 {
        last_digit_sum(n, &cp)
    } else {
        (last_digit_sum(n, &cp) - last_digit_sum(m - 1, &cp) + MODULO) % MODULO
    }
}

fn last_digit_sum(n: i64, cp: &Vec<i64>) -> i64 {
    let p_sz = cp.len() as i64;
    let (q, r) = (n / p_sz, n % p_sz);
    (q * cp[p_sz as usize - 1] + cp[r as usize]) % MODULO
}
