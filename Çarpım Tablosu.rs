use std::convert::TryInto; // try_into().unwrap()

fn bir_artır(girilen_sayı: isize) -> isize {
    girilen_sayı + 1
}
/*
fn bir_artır_ref(girilen_sayı: &mut isize) {
   *girilen_sayı += 1; // Hata verirse "*" kaldırılıp tekrar denenilsin
}
*/

fn çarpım_tablosu(kaçtan: usize, kaça: usize) {
    let mut sayı1 = 0;
    let mut sayı2 = 0;
	
	println!("\n\n---- ---- ---- {0}x{1} Çarpım Tablosu ---- ---- ----\n", kaçtan, kaça);
    loop {
        sayı1 = bir_artır(sayı1);
	//  bir_artır_ref(&mut sayı1);
        loop {
            sayı2 = bir_artır(sayı2);

            println!("- {0} x {1} = {2}", sayı1, sayı2, sayı1 * sayı2);

            if sayı2 == kaça.try_into().unwrap() {
                println!("{0}'nin çarpımları tamamlandı!\n", sayı1);
                sayı2 = 0;
                break;
            }
        }
        
        if sayı1 == kaçtan.try_into().unwrap() {
            println!("Tablo tamamlandı!");
            break;
        }
    }
}

fn main() {
    çarpım_tablosu(25, 25);
}