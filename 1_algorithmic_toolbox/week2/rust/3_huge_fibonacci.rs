fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut input = input.split_whitespace().map(|i| i.parse::<i64>().unwrap());

    let n = input.next().unwrap();
    let m = input.next().unwrap() as i32;

    println!("{}", fibonacci_mod_m(n, m));
}

fn fibonacci_mod_m(n: i64, m: i32) -> i32 {
    let mut pisano_period = vec![0];
    let (mut a, mut b) = (1, 1);
    while !(a == 0 && b == 1) {
        pisano_period.push(a);
        (a, b) = (b, (a + b) % m);
    }

    pisano_period[n as usize % pisano_period.len()]
}
