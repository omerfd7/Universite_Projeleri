use std::io::{self, Write};

fn main() {
    print!("Paket (kg): ");
    io::stdout().flush().unwrap();

    let mut girdi = String::new();
    io::stdin().read_line(&mut girdi).unwrap();
    let agirlik: f64 = girdi.trim().parse().unwrap();

    let ucret;

    if agirlik <= 2.0 {
        ucret = 60.0;
    } else {
        ucret = 60.0 + (agirlik - 2.0) * 15.0;
    }

    println!("Kargo ücreti: {} TL", ucret);
}