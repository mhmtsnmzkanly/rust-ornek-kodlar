enum EvcilHayvan
{
	Kedi,
	Köpek,
	Hamster,
	Fare,
	Yılan,
}

fn ehsi_nedir(kim: String, eh: EvcilHayvan) 
{
	println!("{} Evcil Hayvanı Nedir?", kim);
	print!("{} Evcil Hayvanı ", kim);
	match eh {
		 EvcilHayvan::Kedi => println!("Kedi'dir."),
		 EvcilHayvan::Köpek => println!("Köpek'dir."),
		 EvcilHayvan::Hamster => println!("Hamster'dır."),
		 EvcilHayvan::Fare => println!("Fare'dir."),
		 EvcilHayvan::Yılan => println!("Yılan'dır."),
	}
} // Fonksiyon referans kullanılmadığı için bu kısma geldikten sonra değişken düşer(drop).

fn main()
{
	let aysenin_ehsi: EvcilHayvan = EvcilHayvan::Kedi;
	let mehmetin_ehsi: EvcilHayvan = EvcilHayvan::Yılan;
	let velinin_ehsi: EvcilHayvan = EvcilHayvan::Fare;
	let ahmetin_ehsi: EvcilHayvan = EvcilHayvan::Köpek;
	let fatmanin_ehsi: EvcilHayvan = EvcilHayvan::Hamster;
	
	ehsi_nedir("Ayşe'nin".into(), aysenin_ehsi);
	ehsi_nedir("Mehmet'in".into(), mehmetin_ehsi);
	ehsi_nedir("Veli'nin".into(), velinin_ehsi);
	ehsi_nedir("Ahmet'in".into(), ahmetin_ehsi);
	ehsi_nedir("Fatma'nın".into(), fatmanin_ehsi);
}
