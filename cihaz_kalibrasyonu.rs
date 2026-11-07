use std::io::{self, Write};

fn main() {
    print!("Cihazın adını girin: ");
    io::stdout().flush().unwrap();
    let mut cihaz_adi = String::new();
    io::stdin().read_line(&mut cihaz_adi).unwrap();
    let cihaz_adi = cihaz_adi.trim();
    
    print!("Bulunduğu yeri girin: ");
    io::stdout().flush().unwrap();
    let mut yer = String::new();
    io::stdin().read_line(&mut yer).unwrap();
    let yer = yer.trim();
    
    print!("Son kalibrasyon yılını girin: ");
    io::stdout().flush().unwrap();
    let mut yil_girdi = String::new();
    io::stdin().read_line(&mut yil_girdi).unwrap();
    let son_kalibrasyon: i32 = yil_girdi.trim().parse().unwrap();
    let sonraki_kalibrasyon = son_kalibrasyon + 2;
    
    println!("Cihaz               : {}", cihaz_adi);
    println!("Yer                 : {}", yer);
    println!("Sonraki kalibrasyon : {}", sonraki_kalibrasyon);
}