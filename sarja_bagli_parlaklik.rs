use std::io::{self, Write};

fn main() {
    print!("Batarya yüzdesini girin: ");
    io::stdout().flush().unwrap();

    let mut girdi = String::new();
    io::stdin().read_line(&mut girdi).unwrap();
    let batarya: i32 = girdi.trim().parse().unwrap();

    if batarya < 20 {
        println!("Şarj cihazını takın");
        println!("Parlaklık azaltıldı");
    }

    println!("Kontrol tamam");
}