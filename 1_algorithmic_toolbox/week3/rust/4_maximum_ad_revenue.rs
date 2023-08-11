fn main() {
    let mut buffer = String::new();
    let mut input = line_reader(&mut buffer, 3);
    let n = input.next().unwrap() as usize;
    let mut input = input.collect::<Vec<_>>();
    let (prices, clicks) = input.split_at_mut(n);

    println!("{}", maximum_ad_revenue(prices, clicks));
}

fn line_reader<'a, T>(buffer: &'a mut String, n_lines: usize) -> Box<dyn Iterator<Item = T> + 'a>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    buffer.clear();
    for _ in 0..n_lines {
        std::io::stdin().read_line(buffer).unwrap();
    }

    Box::new(buffer.split_whitespace().map(|i| i.parse().unwrap()))
}

fn maximum_ad_revenue(prices: &mut [i32], clicks: &mut [i32]) -> i64 {
    prices.sort();
    clicks.sort();
    prices
        .iter()
        .zip(clicks)
        .map(|(price, click)| (*price as i64, *click as i64))
        .fold(0, |total, (p, c)| total + p * c)
}
