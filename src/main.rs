use clap::{Parser, ValueEnum};
use wordle_solver::{algorithms, Guesser, Wordle};

const GAMES: &str = include_str!("../data/answers.txt");
const MAX_ATTEMPTS: usize = usize::MAX;

fn main() {
	let args = Args::parse();
	match args.implementation {
		Implementation::Naive => play(algorithms::NaiveGuesser::default()),
	};
}

fn play(mut guesser: impl Guesser) {
	let wordle = Wordle::new();

	for answer in GAMES.lines() {
		if let Some(n_attempts) = wordle.play::<MAX_ATTEMPTS>(answer, &mut guesser) {
			println!("Guessed {answer} in {n_attempts} attempts.");
		} else {
			println!("Did not guess {answer} in <={MAX_ATTEMPTS} attempts.");
		}
	}
}

#[derive(Parser)]
struct Args {
	/// The implementation to use.
	#[arg(long = "impl")]
	#[clap(default_value = "naive")]
	implementation: Implementation,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Implementation {
	Naive,
}
