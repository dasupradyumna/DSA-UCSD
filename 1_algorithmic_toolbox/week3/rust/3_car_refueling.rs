fn main() {
    let mut buffer = String::new();
    let mut input = line_reader::<i32>(&mut buffer, 4);

    let distance = input.next().unwrap();
    let mileage = input.next().unwrap();
    input.next().unwrap(); // ignore number of stops; vector length is sufficient
    let mut stops = vec![0];
    stops.extend(input);
    stops.push(distance);

    println!("{}", minimum_refills(mileage, stops));
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

fn minimum_refills(mileage: i32, stops: Vec<i32>) -> i32 {
    let mut current_range = 0;
    let mut n_refills = -1;
    for i in 0..stops.len() - 1 {
        if stops[i + 1] > current_range {
            current_range = stops[i] + mileage;
            if stops[i + 1] > current_range {
                return -1; // quit if distance between 2 stop is more than mileage
            }
            n_refills += 1;
        }
    }

    n_refills
}
