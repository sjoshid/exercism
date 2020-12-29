fn main() {
	let rna = "sujitisamazing";
	let mut s = 0;

	let iterations = rna.len() / 3;
	for iteration in 0..iterations {
		let mut e = s + 3;
		let asdf = &rna[s..e];
		println!("{:?}", asdf);
		s = e;
	}

	/*let mut adlsfj = rna.chars()
		.enumerate()
		.flat_map(|(i, c)| {
			if i != 0 && i % 3 == 0 {
				Some(' ')
			} else {
				None
			}
				.into_iter()
				.chain(std::iter::once(c))
		})
		.collect::<String>();
	let lajsdf: Vec<&str>  = adlsfj.split(" ").collect();*/

}