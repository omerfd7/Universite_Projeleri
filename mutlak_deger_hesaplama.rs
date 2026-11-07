use std::io::{self, Write};

fn main() {
    print!("Bir sayı girin: ");
    io::stdout().flush().unwrap();
    let mut g = String::new();
    io::stdin().read_line(&mut g).unwrap();
    
    let sayi: i32 = match g.trim().parse() {
        Ok(s) => s,
        Err(_) => {
            println!("Hatalı sayı girdiniz!");
            return;
        }
    };

    let mutlak = if sayi >= 0 { sayi } else { -sayi };
    println!("Mutlak değer: {}", mutlak);
}