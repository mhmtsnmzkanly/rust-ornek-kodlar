// Kullanılan kütüphane:
// reqwest = { version = "0.11", features = ["blocking", "json"] }
use std::io;
use std::collections::HashMap;

const SEHIR_PLAKA_VE_ADLARI: &str = "https://gist.githubusercontent.com/serong/9b25594a7b9d85d3c7f7/raw/9904724fdf669ad68c07ab79af84d3a881ff8859/iller.json";

#[derive(Debug)]
struct Sehir {
    plaka: u8,
    ad: String,
}

impl Sehir {
    fn aktar(p: u8, a: String) -> Sehir {
        Sehir { plaka: p, ad: a }
    }

    fn bilgi(&self) -> String{
        format!("Şehrin Adı {0}, Plakası {1}.", self.ad, self.plaka)
    }
}

fn cikis(kod: i32) {
    println!("\nProgram \"{k}\" kodu ile sonlandırılıyor...", k = kod);
    std::process::exit(kod);
}

// reqwest kütüphanesi kullanıldı.
fn sehir_plaka_ve_adlarini_al() -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
    let liste = reqwest::blocking::get(SEHIR_PLAKA_VE_ADLARI)?.json::<HashMap<String, String>>()?;
    Ok(liste)
}

fn hata_sehir_plaka_ve_adlarini_al(hata: Box<dyn std::error::Error>) -> String {
    String::from(format!("\nDetaylar:\n{0:?}", hata))
}

fn sehir_listesi_olustur(liste: &mut Vec<Sehir>, liste_hashmap: HashMap<String, String>) {
    for sehir in liste_hashmap {
        let _ = &liste.push(Sehir::aktar(sehir.0.parse::<u8>().unwrap(), sehir.1));
    }
}

// Bubble short kullanıldı
fn sehir_listesini_sirala(liste: &mut Vec<Sehir>) {
    let uzunluk = liste.len();
    let mut yer_degistir = true;

    while yer_degistir {
        yer_degistir = false;
        for u in 1..uzunluk {
            if liste[u - 1].plaka > liste[u].plaka {
                liste.swap(u - 1, u);
                yer_degistir = true;
            }
        }
    }
}

fn sehir_plakasi_al(plaka: &mut usize, son_il: usize) {
    loop {
        println!("Plaka: ");
        
        let mut giris_string: String = String::new();
        io::stdin().read_line(&mut giris_string).expect("Hata! Giriş alınamadı.");

        let giris_isize = giris_string.trim().parse::<isize>().unwrap();

        if giris_isize == 0 {
            cikis(0);
        }
        
        if giris_isize < 0 || giris_string.trim().parse::<usize>().unwrap() > son_il {
            println!("Lütfen 1 ila {son_il} arasında (kendileri de dahil olmak üzere) bir sayı girin.", son_il = son_il);
            continue;
        }
                
        *plaka = giris_string.trim().parse().unwrap();
        break;
    }
}

fn main() {
    println!("Program çalışmaya başladı.\n\nŞehir (plaka ve ad) Listesi güncelleniyor...");

    let sehir_liste_hashmap = match sehir_plaka_ve_adlarini_al() {
        Ok(liste) => {
            println!("Şehir (plaka ve ad) Listesi güncellendi.");
            liste
        }
        Err(hata) => {
            println!("Şehir (plaka ve ad) Listesi güncellenirken hata ile karşılaşıldı.{0}", hata_sehir_plaka_ve_adlarini_al(hata));
            cikis(1);
            HashMap::new()
        }
    };

    let mut sehir_listesi: Vec<Sehir> = Vec::new();
    
    sehir_listesi_olustur(&mut sehir_listesi, sehir_liste_hashmap);
    
    sehir_listesini_sirala(&mut sehir_listesi);

    println!("\nArtık hazırız,\nLütfen 1 ila {sehir_sayisi} arasında (kendileri de dahil olmak üzere) bir sayı girin.\nProgramdan çıkmak için \"0\" giriniz.", sehir_sayisi = sehir_listesi.len());
    
    let mut istenilen_plaka: usize = 0;
    sehir_plakasi_al(&mut istenilen_plaka, sehir_listesi.len());
    println!("{0}", sehir_listesi[istenilen_plaka-1].bilgi());
    cikis(0);
}
