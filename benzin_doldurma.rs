use std::io::{self, Write};

fn main() {
    print!("Benzin miktarını girin: ");
    io::stdout().flush().unwrap();

    let mut girdi = String::new();
    io::stdin().read_line(&mut girdi).unwrap();
    let benzin: i32 = girdi.trim().parse().unwrap();

    if benzin >= 50 {
        println!("Doluma gerek yok.");
    } else {
        println!("Depo dolduruluyor.");
    }

    println!("Kontrol tamam.");
}