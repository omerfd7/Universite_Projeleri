use std::io::{self, Write};

fn main() {
    print!("1. Sayıyı girin: ");
    io::stdout().flush().unwrap();
    let mut g1 = String::new();
    io::stdin().read_line(&mut g1).unwrap();
    let sayi1: i32 = g1.trim().parse().unwrap();

    print!("2. Sayıyı girin: ");
    io::stdout().flush().unwrap();
    let mut g2 = String::new();
    io::stdin().read_line(&mut g2).unwrap();
    let sayi2: i32 = g2.trim().parse().unwrap();

    print!("3. Sayıyı girin: ");
    io::stdout().flush().unwrap();
    let mut g3 = String::new();
    io::stdin().read_line(&mut g3).unwrap();
    let sayi3: i32 = g3.trim().parse().unwrap();

    if sayi1 > sayi2 && sayi1 > sayi3 {
        println!("sayi1 daha büyük");
    } else if sayi2 > sayi1 && sayi2 > sayi3 {
        println!("sayi2 daha büyük");
    } else {
        println!("sayi3 daha büyük");
    }
}