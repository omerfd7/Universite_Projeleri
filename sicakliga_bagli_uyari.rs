use std::io::{self, Write};

fn main() {
    print!("Sıcaklık değerini girin: ");
    io::stdout().flush().unwrap();

    let mut girdi = String::new();
    io::stdin().read_line(&mut girdi).unwrap();
    let sicaklik: i32 = girdi.trim().parse().unwrap();

    if sicaklik >= 35 {
        println!("KRİTİK: sistemi kapat.");
    } else if sicaklik >= 28 {
        println!("UYARI: soğutmayı artır.");
    } else {
        println!("Sistem normal çalışıyor.");
    }
}