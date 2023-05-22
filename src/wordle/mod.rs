use crate::{Correctness, Guess, Guesser};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::{collections::HashSet, fmt::Display};

pub type Word = &'static str;

/// Answers for the official Wordle game (March 5th 2022)
pub static ANSWERS: &str = include_str!("../../data/words/wordle-answers.txt");

lazy_static! {
	/// All words allowed by Wordle + their occurrence count according Google Books
	/// (latest year per book)
	pub static ref LEGAL_WORDS: HashSet<&'static str> = {
		include_str!("../../data/words/legal-words.txt")
			.lines()
			.map(|line| {
				line.split_once(' ')
					.expect("Every line in the dictionary should be `word count`.")
					.0
			})
			.collect()
	};

	/// Global instance of a [`Dictionary`] so that the guesser doesn't have to make a new one for
	/// each guess.
	pub static ref DICTIONARY: Dictionary = {
		Dictionary {
			entries: include_str!("../../data/words/legal-words.txt")
				.lines()
				.map(|line| {
					let (word, count) = line
						.split_once(' ')
						.expect("Every line in the dictionary should be `word count`.");

					let count = count
						.parse()
						.expect("Every count should fit in `usize::MAX`.");

					DictionaryEntry { word, count }
				})
				.sorted_unstable_by_key(|entry| std::cmp::Reverse(entry.count))
				.collect_vec(),
		}
	};
}

/// The dictionary used for playing Wordle.
///
/// It contains a list of _legal_ words together with ther total count according to Google Books
/// and relative frequency.
#[derive(Debug, Clone, PartialEq)]
pub struct Dictionary {
	pub entries: Vec<DictionaryEntry>,
}

impl Dictionary {
	/// Plays a game of Wordle against the given [`Guesser`] and.
	/// If the [`Guesser`] fails to guess the `answer` in `max_attempts`, this function will return
	/// [`None`].
	/// Otherwise it will return [`Some`] with the amount of required guesses.
	pub fn play(
		&self,
		mut guesser: impl Guesser,
		answer: Word,
		max_attempts: usize,
	) -> Option<usize> {
		let mut guess_history = Vec::new();

		for round in 1..=max_attempts {
			// Make the guess
			let guess = guesser.guess(&guess_history);

			// Ensure the guess is actually legal.
			assert!(LEGAL_WORDS.contains(guess), "illegal guess \"{guess}\"");

			// We guessed correctly!
			if guess == answer {
				return Some(round);
			}

			guess_history.push(Guess {
				word: guess,
				correctness: Correctness::compute(guess, answer),
			});
		}

		None
	}
}

impl Display for Dictionary {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.entries)
	}
}

/// A single entry in the [`Dictionary`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DictionaryEntry {
	pub word: Word,
	pub count: usize,
}

impl Display for DictionaryEntry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(self.word)
	}
}
