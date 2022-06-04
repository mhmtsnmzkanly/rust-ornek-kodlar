/*
    Clap kütüphanesi kullanılarak yazılan bu program,
    Basit bir CLI (Command Line Interface / Komut Satırı Arayüzü) sunuyor.
    
    Program çalıştırılırken iki değer girilmesi gerekiyor.
    Bu iki değer Kalıp(Pattern / Aranan ifade) ve Yol(Path / Aranacak Dosya)'dan oluşuyor.
    Değerlerden biri boş bırakılırsa program otomatik kendini sonlandırıyor.
    
    Program iki değeri aldıktan sonra önce Yol'dan dosyayı arıyor,
    Bulamazsa "0 | Belirtilen dosya bulunamadı/okunamadı." diyerek hata veriyor.
    Bulursa sonra ki adıma geçiliyor ve istenilen kalıba bakılıyor.
    
    Eğer kalıp "*" ise dosyada ki tüm içerik dönüyor.
    Değilse kalıplar tek tek aranıyor.
    Kalıp bulunamazsa "0 | İstediğiniz kalıbı bulamadık." döndürülüyor.
    
    Bulunursa tüm kalıplar satır satır geri döndürülüyor.
*/
use clap::Parser; // clap = { version = "3.1.9", features = ["derive"] }

fn main() {
    let args = Cli::parse();
    println!("{0}", args.dosyada_ara());
}


/// İstenilen dosyada, istenilen kalıbı arayın.
#[derive(Parser)]
struct Cli {
    /// Aradığınız kalıp.
    pattern: String,
    /// Aranacak dosya.
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

impl Cli {
    // İstenilen dosyayı okur ve String olarak döndürür.
    // Not: std::fs::try_exists kullanıma hazır olunca yeniden yazılacak.
    pub fn dosyayı_oku(&self) -> String {
        match std::fs::read_to_string(&self.path) {
            Ok(içerik) => içerik,
            Err(hata) => panic!("0 | Belirtilen dosya bulunamadı/okunamadı.\n{0:?}", hata),
        }
    }

    // İstenilen kalıbı, Dosyanın içerisinde arar ve döndürür.
    pub fn dosyada_ara(&self) -> String {
        let içerik: String = self.dosyayı_oku();

        let kalıplar: String = Cli::satırları_oluştur(&self.pattern.clone(), içerik.clone());

        match kalıplar.is_empty() {
            true => format!("0 | İstediğiniz kalıbı bulamadık."),
            false => kalıplar
        }
    }

    // Girilen değerlere göre yeni bir satır oluşturur.
    pub fn yeni_satır(satır_no: usize, satır: &str, satırlar: String) -> String {
        format!("{0}{1} | {2}\n", satırlar, Cli::satır_no_oluştur(satır_no), satır)
    }

    // Dosya içeriğini satır satır işler.
    pub fn satırları_oluştur(kalıp: &str, içerik: String) -> String {
        let mut satır_no: usize = 0;
        let mut satırlar: String = String::new();

        for satır in içerik.lines() {
            satır_no += 1;
            
            // İstenilen kalıp bulunamadıysa ve tüm satırlar istenmiyorsa devam et
            if !satır.contains(kalıp) && kalıp != "*" { continue; }
            
            satırlar = Cli::yeni_satır(satır_no, satır, satırlar);
        }

        satırlar
    }

    // Satır numaralarını daha güzel göstermek için
    // Not: Makro'ya dönüştürülebilir.
    pub fn satır_no_oluştur(satır_no: usize) -> String {
        match satır_no {
            0..=9 => format!("00{0}", satır_no),
            10..=99 => format!("0{0}", satır_no),
            _ => format!("{0}", satır_no)
        }
    }
}
