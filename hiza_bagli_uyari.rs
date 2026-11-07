use std::io::{self, Write};

fn main() {
    print!("Hızı girin: ");
    io::stdout().flush().unwrap();

    let mut girdi = String::new();
    io::stdin().read_line(&mut girdi).unwrap();
    let hiz: i32 = girdi.trim().parse().unwrap();

    if hiz > 90 {
        println!("kırmızı uyarı");
    } else if hiz > 50 {
        println!("sarı uyarı");
    } else {
        println!("uyarı yok");
    }
}