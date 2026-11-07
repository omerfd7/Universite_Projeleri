use std::io;

fn ebob(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let k = a % b;
        a = b;
        b = k;
    }
    a
}

fn main() {
    let mut g1 = String::new();
    let mut g2 = String::new();

    io::stdin().read_line(&mut g1).unwrap();
    io::stdin().read_line(&mut g2).unwrap();

    let x = g1.trim().parse().unwrap();
    let y = g2.trim().parse().unwrap();

    println!("{}", ebob(x, y));
}