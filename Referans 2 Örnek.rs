fn stringin_boyutu(metin: &String) -> usize {
    metin.len()
}

fn değiştir_ve_yenisini_döndür(metin: String, hedef: &str, kaynak: &str) -> String {
    metin.replace(hedef, kaynak)
}

fn değiştir_ve_orjinalini_döndür(metin: &mut String) {
    metin.push_str("\nTekrardan hoş geldin.");
}

fn main(){
    let merhaba_mesajı = String::from("Merhaba abc!");
    println!("Orjinal: \"{0}\", Boyutu: {1}", merhaba_mesajı, stringin_boyutu(&merhaba_mesajı));
    
    let merhaba_mesajı = değiştir_ve_yenisini_döndür(merhaba_mesajı, "abc", "Mehmet");
    println!("Değiştirilen: \"{0}\", Boyutu: {1}", merhaba_mesajı, stringin_boyutu(&merhaba_mesajı));

    let mut merhaba_mesajı = merhaba_mesajı;
    println!("Değiştrebilir Kopya: \"{0}\"", merhaba_mesajı);
    değiştir_ve_orjinalini_döndür(&mut merhaba_mesajı);
    println!("Değiştrebilir Kopya Değiştirildi:\n\"{0}\", Boyutu: {1}", merhaba_mesajı, stringin_boyutu(&merhaba_mesajı));
}