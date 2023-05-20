use wordle_solver::{algorithms, Wordle};

const GAMES: &str = include_str!("../data/answers.txt");
const MAX_ATTEMPTS: usize = usize::MAX;

fn main() {
	let wordle = Wordle::new();

	let mut guesser = algorithms::NaiveGuesser;

	for answer in GAMES.lines() {
		if let Some(n_attempts) = wordle.play::<MAX_ATTEMPTS>(answer, &mut guesser) {
			println!("Guessed {answer} in {n_attempts} attempts.");
		} else {
			println!("Did not guess {answer} in <={MAX_ATTEMPTS} attempts.");
		}
	}
}
