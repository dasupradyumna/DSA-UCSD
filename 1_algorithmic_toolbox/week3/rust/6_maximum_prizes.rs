fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    let mut n: i32 = buffer.trim().parse().unwrap();

    let mut a = 1;
    while n >= a {
        n -= a;
        a += 1;
    }
    a -= 1;

    println!("{a}");
    for i in 1..a {
        print!("{i} ");
    }
    println!("{}", a + n);
}
