use std::collections::HashMap;

/* Sabitler */


/* Fonksiyonlar */
fn main(){
    let takım_isimleri = vec![
        String::from("Siyah"),
        String::from("Mavi"),
        String::from("Yeşil"),
        String::from("Sarı"),
        String::from("Kırmızı"),
    ]; // Takım isimleri

    let mut takım_tablosu: HashMap<String, usize> = HashMap::new(); // Takım tablosu
    takımları_oluştur(&mut takım_tablosu, &takım_isimleri);
    
    takım_skoruna_ekle(&mut takım_tablosu, takım_bul(&takım_isimleri, 1), 10);
    takım_skoruna_ekle(&mut takım_tablosu, takım_bul(&takım_isimleri, 3), 30);
    takım_skoruna_ekle(&mut takım_tablosu, takım_bul(&takım_isimleri, 2), 5);
    takım_skoruna_ekle(&mut takım_tablosu, takım_bul(&takım_isimleri, 4), 23);
    print!("{:?}", takım_tablosu);
}

// Takım isimlerinden takımı bulan fonksiyon.
fn takım_bul(isimler: &Vec<String>, id: usize) -> String
{
    match isimler.get(id) {
        Some(value) => (*value).to_string(), // İstenilen
        None => (*isimler.get(0).unwrap()).to_string(), // Varsayılan
    }
}

// Programın başında takım isimlerinden takım tablosu oluşturan fonksiyon.
fn takımları_oluştur(takımlar: &mut HashMap<String, usize>, takım_isimleri: &Vec<String>) {
    for takım_ismi in takım_isimleri {
        takımlar.insert(takım_ismi.to_string(), 0);
    }
}

// Takım tablosuna skor ekleyen fonksiyon.
fn takım_skoruna_ekle(takımlar: &mut HashMap<String, usize>, takım: String, yeni_skor: usize) {
    let takım_bilgisi = takımlar.entry(takım).or_insert(0);
    *takım_bilgisi += yeni_skor;
}