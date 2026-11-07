use std::io::{self, Write};

fn main() {
    print!("a kenarını girin: ");
    io::stdout().flush().unwrap();
    let mut g1 = String::new();
    io::stdin().read_line(&mut g1).unwrap();
    let a: i32 = g1.trim().parse().unwrap();

    print!("b kenarını girin: ");
    io::stdout().flush().unwrap();
    let mut g2 = String::new();
    io::stdin().read_line(&mut g2).unwrap();
    let b: i32 = g2.trim().parse().unwrap();

    print!("c kenarını girin: ");
    io::stdout().flush().unwrap();
    let mut g3 = String::new();
    io::stdin().read_line(&mut g3).unwrap();
    let c: i32 = g3.trim().parse().unwrap();

    // Üçgen oluşturma kuralı kontrolü
    if a + b > c && a + c > b && b + c > a {
        if a == b && b == c {
            println!("Eşkenar üçgen");
        } else if a == b || b == c || a == c {
            println!("İkizkenar üçgen");
        } else {
            println!("Çeşitkenar üçgen");
        }
    } else {
        println!("Bu kenarlardan bir üçgen oluşmaz");
    }
}