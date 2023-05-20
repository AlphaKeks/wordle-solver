use std::collections::HashSet;

mod correctness;
pub use correctness::Correctness;

mod guesser;
pub use guesser::{Guess, Guesser};

pub mod algorithms;

const DICTIONARY: &str = include_str!("../data/legal-words-with-counts.txt");

pub struct Wordle {
	dictionary: HashSet<&'static str>,
}

#[allow(clippy::new_without_default)]
impl Wordle {
	pub fn new() -> Self {
		Self {
			dictionary: HashSet::from_iter(DICTIONARY.lines().map(|line| {
				line.split_once(' ')
					.expect("line format of `word count`")
					.0
			})),
		}
	}

	pub fn play<const ROUNDS: usize>(
		&self,
		answer: &'static str,
		mut guesser: impl Guesser,
	) -> Option<usize> {
		let mut history = Vec::new();

		// The actual Wordle game allows 6 guesses.
		// We allow more for stats purposes.
		for i in 1..=ROUNDS {
			let guess = guesser.guess(&history);

			// make sure the guess is legal
			assert!(self.dictionary.contains(guess.as_str()), "illegal guess `{guess}`");

			if guess == answer {
				return Some(i);
			}

			let correctness = Correctness::compute(answer, &guess);

			history.push(Guess { word: guess, correctness });
		}

		None
	}
}
