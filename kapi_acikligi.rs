use std::io::{self, Write};

fn main() {
    print!("Kapı açık mı (1 veya 0): ");
    io::stdout().flush().unwrap();
    let mut g = String::new();
    io::stdin().read_line(&mut g).unwrap();
    let secim = g.trim();

    if secim == "1" {
        println!("Kapı: açık");
    } else if secim == "0" {
        println!("Kapı: kapalı");
    } else {
        println!("Hatalı giriş yaptınız, sadece 1 veya 0 girebilirsiniz.");
    }
}