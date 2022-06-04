// Vec<i32> değerlerini tek tek ekrana yazdıran fonksiyon
fn vektörün_değerleri(vek: &Vec<i32>) {
    for öğe in vek {
        println!("{}", öğe);
    }
} // Vektörün bellekten silinmemesi için referans ile eriştik.


fn vektörün_değerleri_kendiyle_çarp(vek: &mut Vec<i32>) {
    for öğe in vek {
        *öğe += 1;
    }
}

fn main(){
    // i32 içeren Vektörü oluşturduk
    let mut sayılarım: Vec<i32> = Vec::new();
    println!("sayılarım = {:?}", sayılarım);

    // Sırasıyla Vektöre "1,3,6" sayılarını ekledik.
    sayılarım.push(1);
    sayılarım.push(3);
    sayılarım.push(6);
    println!("sayılarım = {:?}", sayılarım);

    // Vektör içerisinde 2. sırada olan "3" sayısına referans atadık.
    // Mutable olan Vektör, referanslandığı için Immutable olarak güncellendi.
    let sayılarım_üç: &i32 = &sayılarım[1];
    println!("sayılarım_üç = {:?}", sayılarım_üç);
    
    // "3" sayısına giden referansı düşürdük/sildik/kaldırdık.
    // Artık Immutable olan Vektör eski haline dönerek Mutable oldu.
    drop(sayılarım_üç);
    
    // "3" sayısını "7" olarak güncelledik.
    sayılarım[1] = 7;
    println!("sayılarım = {:?}", sayılarım);

    // Bütün değerleri tek tek ekrana yazdıralım.
    vektörün_değerleri(&sayılarım);
    println!("sayılarım = {:?}", sayılarım);

    vektörün_değerleri_kendiyle_çarp(&mut sayılarım);
    println!("sayılarım = {:?}", sayılarım);
}