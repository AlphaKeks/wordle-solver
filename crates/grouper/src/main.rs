fn main() {
	let input = std::env::args()
		.nth(1)
		.expect("Missing input file.");

	std::fs::read_to_string(input)
		.expect("Failed to read input file.")
		.split("\n\n")
		.map(|group| {
			group
				.lines()
				.fold(("", 0usize), |(_, count_acc), line| {
					let (word, count) = line
						.split_once(' ')
						.expect("Invalid line format.");

					let count = count
						.parse::<usize>()
						.expect("`count` is not a number.");

					(word, count_acc + count)
				})
		})
		.for_each(|(word, count)| println!("{word} {count}"));
}
