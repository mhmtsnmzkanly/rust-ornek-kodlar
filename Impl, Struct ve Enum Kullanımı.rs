#[derive(Debug)]
enum Cinsiyet {
    Erkek,
    Kadın,
}

#[derive(Debug)]
struct Vatandaş {
    tc_no: String,
    adı: String,
    soy_adı: String,
    cinsiyeti: Cinsiyet,
}

impl Vatandaş {
    fn yeni(tc: &str, ad: &str, soyad: &str, cins: Cinsiyet) -> Vatandaş {
        Vatandaş {
            tc_no: tc.to_string(),
            adı: ad.to_string(),
            soy_adı: soyad.to_string(),
            cinsiyeti: cins
        }
    }

    fn tc_no_güncelle(&self, yeni_tc_no: &str) {
        self.tc_no = yeni_tc_no.to_string();
    }
}

fn main(){
    let vtn_a = Vatandaş::yeni("34346521", "Mehmet", "Sönmez", Cinsiyet::Erkek);
    println!("Vatandaş A: {:?}", vtn_a);
}