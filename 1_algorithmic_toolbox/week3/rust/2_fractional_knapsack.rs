fn main() {
    let mut buffer = String::new();
    let [n, capacity] = line_reader(&mut buffer, 1).collect::<Vec<_>>()[..] else { panic!() };
    let items = get_rates_weights(&mut buffer, n as usize);

    println!("{:.4}", maximum_cost_capacity(capacity, items));
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

fn get_rates_weights(buffer: &mut String, n_lines: usize) -> Vec<(f32, i32)> {
    let mut input = line_reader(buffer, n_lines);
    let mut items = Vec::with_capacity(n_lines);
    let (mut rate, mut weight);
    for _ in 0..n_lines {
        rate = input.next().unwrap() as f32;
        weight = input.next().unwrap();
        if weight > 0 {
            rate /= weight as f32;
        }
        items.push((rate, weight));
    }

    items
}

fn maximum_cost_capacity(mut capacity: i32, mut items: Vec<(f32, i32)>) -> f32 {
    items.sort_by(|left, right| (&right.0).partial_cmp(&left.0).unwrap());

    let mut total = 0.;
    for (rate, weight) in items {
        if capacity <= 0 {
            break;
        }
        total += rate * std::cmp::min(weight, capacity) as f32;
        capacity -= weight
    }

    total
}
