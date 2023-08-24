fn main() {
    let mut buffer = String::new();
    let sequence = line_reader(&mut buffer, 2).skip(1).collect();
    let queries = line_reader(&mut buffer, 2).skip(1);
    for query in queries {
        print!("{} ", binary_search_duplicates(&sequence, query));
    }
    println!();
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

use std::cmp::Ordering;

fn binary_search_duplicates(sequence: &Vec<i32>, query: i32) -> i32 {
    let (mut left, mut right) = (0, sequence.len());
    while left < right {
        let mid = left + (right - left) / 2;
        match sequence[mid].cmp(&query) {
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,

            Ordering::Equal => {
                if mid == 0 || sequence[mid - 1] != query {
                    return mid as i32;
                } else {
                    right = mid
                }
            }
        }
    }

    -1 // query not found
}
