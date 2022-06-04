fn main()
{
	let name = String::from("Mehmet");
	let message = String::from("Hello");
	let output = format!("{0} {1}!", message, name);
	println!("{}", output);
}