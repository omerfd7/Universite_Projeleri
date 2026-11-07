use std::io::{self, Write};

fn main() {
    print!("Süre (dk): ");
    io::stdout().flush().unwrap();

    let mut girdi = String::new();
    io::stdin().read_line(&mut girdi).unwrap();
    let sure: i32 = girdi.trim().parse().unwrap();

    let ucret;

    if sure < 60 {
        ucret = 0;
    } else if sure < 100 {
        ucret = 40;
    } else {
        ucret = 80;
    }

    println!("Ücret: {} TL", ucret);
}