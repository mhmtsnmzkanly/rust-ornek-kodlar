fn main()
{
	let kanallar: (
		(&str, &str),
		(&str, &str),
		(&str, &str)
		) = (
		("Bilgi Denizi", "İlginç bilgileri video olarak derleyen kanal"),
		("Goygoycu", "Gündemi mizahi bir dille anlatan kanal"),
		("Adım Adım", "Bir içeriği Adım Adım işleyerek öğreten kanal")
	);
	
	println!("-- Kanal Listesi:");
	println!("Ad: {} - Açıklama: {}", kanallar.0.0,kanallar.0.1);
	println!("Ad: {} - Açıklama: {}", kanallar.1.0,kanallar.1.1);
	println!("Ad: {} - Açıklama: {}", kanallar.2.0,kanallar.2.1);
	
	/*
		Sayılarla yani index'lere göre içeriğe erişilir.
		Yazılımda sayma işlemi doğal sayılardan başlanıldığı için 0'dan başlanır ve uzunluk neyse orada biter.
		Uzunlukta 0'da sayılır.
		kanallar.len() dediğimizde "3" sayısını veriyorsa bu demektir ki "0, 1, 2"'den oluşuyordur kısacası "kanallar.len() - 1" bize son numarayı verir.
		Tupler'e her zaman sayılar ile erişilir, kendi içinde tupler'e sahipse yine 0'dan başlayarak erişilir.
	*/
}