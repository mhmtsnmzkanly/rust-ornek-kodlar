use std::convert::TryInto;

fn main(){
    let kendiyle_çarp = |sayı: i32| -> isize { (sayı * sayı).try_into().unwrap() };
    let ikiyle_çarp = |sayı: i32| -> isize { (sayı * 2).try_into().unwrap() };

    println!("Kendiyle çarpılınca: {0}*{0}={1}", 45, kendiyle_çarp(45));
    println!("İkiyle çarpılınca: {0}*2={1}", 45, ikiyle_çarp(45));
    
    println!("Kendiyle çarpılınca: {0}*{0}={1}", 180, kendiyle_çarp(180));
    println!("İkiyle çarpılınca: {0}*2={1}", 180, ikiyle_çarp(180));
}