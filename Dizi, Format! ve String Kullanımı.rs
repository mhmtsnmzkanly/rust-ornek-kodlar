fn main() {
    let dizi = [3, 6, 9, 12, 15, 18, 21, 24, 27, 30];
    let mut dizi_konum = 0;
    let mut yazdırılacak_veri = String::new();

    while dizi_konum < dizi.len() {
        yazdırılacak_veri.push_str(&format!("dizi[{konum}] = {değer}\n", konum = dizi_konum, değer = dizi[dizi_konum]));
        dizi_konum += 1;
    }
    
	yazdırılacak_veri.insert_str(0, &format!("dizi = {0:?}.len(): {1}\n", dizi, dizi.len()));
    yazdırılacak_veri.push_str(&format!("Tüm değerler gösterildi!"));
    println!("{}", yazdırılacak_veri);
}

// push_str(string: &str) -> String'in sonuna "string" ekler.
// insert_str(idx: usize, string: &str) -> String'in "idx" kısmına "string" ekler.
// Format!() -> Format makrosu girilen &str ve String değerleri derleyerek String olarak geri döndürür.