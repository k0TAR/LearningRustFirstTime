pub fn run() {
	//Print to console
	println!("Hellow from print.rs file.");
	println!("Number: {}", 1);
	println!("{0} = {0}", 2);
	println!("{name} is here.", name = "AAA");
	println!("Binary: {0:b} Hex: {1:x} Octal: {2:o}.", 10, 11, 12);
	//Placeholder for debug trait
	println!("{:?}", (12, true, "Hello."));
}
