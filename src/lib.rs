use std::{borrow::Cow, collections::HashSet};
use tracing::debug;

mod correctness;
pub use correctness::Correctness;

mod guesser;
pub use guesser::{Guess, Guesser};

pub mod algorithms;

const DICTIONARY: &str = include_str!("../data/words/legal-words-with-counts.txt");

pub type Word = &'static str;

pub struct Wordle {
	dictionary: HashSet<Word>,
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

	pub fn play(
		&self,
		answer: Word,
		mut guesser: impl Guesser,
		max_attempts: usize,
	) -> Option<usize> {
		let mut history = Vec::new();

		// The actual Wordle game allows 6 guesses.
		// We allow more for stats purposes.
		for i in 1..=max_attempts {
			let guess = guesser.guess(&history);
			debug!("Guessed {guess}");

			// make sure the guess is legal
			assert!(self.dictionary.contains(guess.as_str()), "illegal guess `{guess}`");

			if guess == answer {
				return Some(i);
			}

			let correctness = Correctness::compute(answer, &guess);

			history.push(Guess { word: Cow::Owned(guess), correctness });
		}

		None
	}
}
