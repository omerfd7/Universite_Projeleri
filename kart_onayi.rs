use std::io::{self, Write};

fn main() {
    print!("Kart geçerli mi (1 veya 0): ");
    io::stdout().flush().unwrap();
    let mut g1 = String::new();
    io::stdin().read_line(&mut g1).unwrap();
    let kart_secim = g1.trim();

    if kart_secim != "1" && kart_secim != "0" {
        println!("Hatalı giriş! Kart durumu sadece 1 veya 0 olabilir.");
        return;
    }

    let kart_gecerli = kart_secim == "1";

    print!("Bakiyenizi girin: ");
    io::stdout().flush().unwrap();
    let mut g2 = String::new();
    io::stdin().read_line(&mut g2).unwrap();
    
    let bakiye: i32 = match g2.trim().parse() {
        Ok(sayi) => sayi,
        Err(_) => {
            println!("Hatalı bakiye girdiniz!");
            return;
        }
    };

    if kart_gecerli && bakiye >= 25 {
        println!("Geçiş onaylandı.");
    } else {
        println!("Geçiş onaylanmadı.");
    }
}