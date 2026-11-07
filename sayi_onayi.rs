use std::io::{self, Write};

fn main() {
    print!("Sayı girin: ");
    io::stdout().flush().unwrap();

    let mut girdi = String::new();
    io::stdin().read_line(&mut girdi).unwrap();
    let sayi: i32 = girdi.trim().parse().unwrap();

    if sayi > 10 && sayi % 2 == 0 {
        println!("Sayınız 10'dan büyük ve çift olduğu için onaylandınız.");
    } else {
        println!("Sayınız 10'dan büyük ve çift olma şartını sağlamadığı için onaylanmadınız.");
    }
}