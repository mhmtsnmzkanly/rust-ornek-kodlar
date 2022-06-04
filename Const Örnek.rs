const AD: &str = "Mehmet"; // &str kullanılmasının nedeni derleyici'nin const en iyi nereye saklayacağını bulabilmesi için(miş).
const SOYAD: &str = "Sönmez"; // &str kullanılmasının nedeni derleyici'nin const en iyi nereye saklayacağını bulabilmesi için(miş).
const BUILD: u16 = 1000; // Derleme numarası

fn main()
{
	println!("Programcı: {} {}", AD, SOYAD);
	println!("Program Derleme Sürümü: {}", BUILD);
}