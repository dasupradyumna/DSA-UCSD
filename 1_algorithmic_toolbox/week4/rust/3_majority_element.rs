fn main() {
    let mut buffer = String::new();
    let sequence = line_reader(&mut buffer, 2).skip(1).collect();
    println!("{}", majority_element(sequence) as i32);
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

fn majority_element(mut sequence: Vec<i32>) -> bool {
    let n = sequence.len() as i32;
    let (mut n_maj, mut curr, mut n_curr) = (-1, -1, -1);

    sequence.sort();
    sequence.into_iter().for_each(|e| {
        if e != curr {
            n_maj = std::cmp::max(n_maj, n_curr);
            (curr, n_curr) = (e, 0);
        }
        n_curr += 1;
    });

    std::cmp::max(n_maj, n_curr) > n / 2
}
