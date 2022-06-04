use std::convert::TryInto;

#[derive(Debug)]
struct IPv4 {
    a: u8,
    b: u8,
    c: u8,
    d: u8
}

impl IPv4 {
    fn new() -> IPv4{
        IPv4 {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
        }
    } // 0.0.0.0 değerine sahip yeni bir IPv4 oluşturur
    
    fn from(i_a: usize, i_b: usize, i_c: usize, i_d: usize) -> IPv4{
        IPv4 {
            a: i_a.try_into().unwrap(),
            b: i_b.try_into().unwrap(),
            c: i_c.try_into().unwrap(),
            d: i_d.try_into().unwrap(),
        }
    } // usize olarak girilen değerlerle bir IPv4 oluşturur

    fn from_u8(i_a: u8, i_b: u8, i_c: u8, i_d: u8) -> IPv4{
        IPv4 {
            a: i_a,
            b: i_b,
            c: i_c,
            d: i_d,
        }
    } // direk IPv4 oluşturur

    fn to_string(&self) -> String {
        format!("{0}.{1}.{2}.{3}", self.a, self.b, self.c, self.d)
    } // Hazır olanı ekrana basar
}

fn main(){
    let sıfırlı_ipv4: IPv4 = IPv4::new();
    let öbür_ipv4: IPv4 = IPv4::from(253, 122, 12, 46);
    let keyfekeder_ipv4: IPv4 = IPv4::from_u8(127, 0, 0, 1);
    let dertli_ipv4: IPv4 = IPv4::from_u8(17, 52, 34, 76);
    
    println!("sıfırlı_ipv4 = {:?}", sıfırlı_ipv4);
    println!("öbür_ipv4 = {:?}", öbür_ipv4);
    println!("keyfekeder_ipv4 = {:?}", keyfekeder_ipv4);
    println!("dertli_ipv4.to_string() -> {0}", dertli_ipv4.to_string());
    println!("_");
    println!("sıfırlı_ipv4.a = {0}", sıfırlı_ipv4.a);
    println!("öbür_ipv4.c = {0}", öbür_ipv4.c);
    println!("keyfekeder_ipv4.d = {0}", keyfekeder_ipv4.d);
    println!("dertli_ipv4.b = {0}", dertli_ipv4.b);
}