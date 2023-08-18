#[derive(Debug, Clone)]
struct Period {
    start: i32,
    end: i32,
}

fn main() {
    let mut buffer = String::new();
    let n = line_reader::<i32>(&mut buffer, 1).next().unwrap() as usize;
    let mut input = line_reader(&mut buffer, n);
    let mut periods = Vec::with_capacity(n);
    let (mut start, mut end);
    for _ in 0..n {
        start = input.next().unwrap();
        end = input.next().unwrap();
        periods.push(Period { start, end });
    }

    let visits = minimum_visits_fast(periods);
    println!("{}", visits.len());
    for visit in visits {
        print!("{visit} ");
    }
    println!("");
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

fn minimum_visits_fast(mut periods: Vec<Period>) -> Vec<i32> {
    periods.sort_by_key(|period| period.end);

    let mut periods = periods.iter();
    let mut current_end = periods.next().unwrap().end;
    let mut visits = vec![current_end];
    while let Some(period) = periods.next() {
        if period.start > current_end {
            current_end = period.end;
            visits.push(current_end);
        }
    }

    visits
}

/* STRESS_TEST
fn main() {
    let mut rng = rand::thread_rng();
    let mut rng = stress_test::Rng::<i32>::new(&mut rng);
    rng.add_dist(1, 100);
    rng.add_dist(0, 1_000_000_000);
    let n_tests = 1e5 as u32;

    for t in 1..=n_tests {
        let n = rng.from_dist(0) as usize;
        let mut periods = Vec::with_capacity(n);
        let (mut start, mut end);
        for _ in 0..n {
            start = rng.from_dist(1);
            end = rng.from_dist(1);
            if start > end {
                (start, end) = (end, start);
            }
            periods.push(Period { start, end });
        }

        let slow = _minimum_visits_slow(periods.clone());
        let fast = minimum_visits_fast(periods.clone());

        if fast != slow {
            println!("\n\nTest failed!\nn = {n}\n{periods:?}");
            println!("Result:\n (slow) {slow:?}\n (fast) {fast:?}");
            break;
        }
        print!("\rTests passed: {t}");
    }
}
*/

fn _minimum_visits_slow(mut periods: Vec<Period>) -> Vec<i32> {
    periods.sort_by(|p1, p2| p2.end.cmp(&p1.end));

    let mut visits = vec![];
    while !periods.is_empty() {
        let current = periods.pop().unwrap();
        let mut overlap = current.end;
        periods.retain(|period| {
            let remove = current.start <= period.end && current.end >= period.start;
            if remove {
                overlap = std::cmp::min(overlap, period.end);
            }
            !remove
        });
        visits.push(overlap);
    }

    visits.sort();
    visits
}
