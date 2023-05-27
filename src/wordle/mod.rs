use crate::{Correctness, Guess, Guesser};
use itertools::Itertools;
use lazy_static::lazy_static;
use std::{collections::HashSet, fmt::Display};

pub type Word = &'static [u8; 5];

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
	pub static ref DICTIONARY: Wordle = {
		Wordle {
			dictionary: include_str!("../../data/words/legal-words.txt")
				.lines()
				.map(|line| {
					let (word, count) = line
						.split_once(' ')
						.expect("Every line in the dictionary should be `word count`.");

					let word = word
						.as_bytes()
						.try_into()
						.expect("All words should be 5 ASCII characters.");

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

/// A struct for playing Wordle.
///
/// It contains a list of _legal_ words together with ther total count according to Google Books.
#[derive(Debug, Clone, PartialEq)]
pub struct Wordle {
	pub dictionary: Vec<DictionaryEntry>,
}

impl Wordle {
	/// Plays a game of Wordle against the given [`Guesser`].
	/// If the [`Guesser`] fails to guess the `answer` in `max_attempts` or less, this function
	/// will return [`None`].
	/// Otherwise it will return [`Some`] with the amount of required guesses.
	pub fn play<G: Guesser>(
		&self,
		guesser: &mut G,
		answer: Word,
		max_attempts: usize,
	) -> Option<usize> {
		let mut guess_history = Vec::new();

		for round in 1..=max_attempts {
			let guess = guesser.guess(&guess_history);

			// Ensure the guess is actually legal.
			// let guess_str = std::str::from_utf8(guess).unwrap();
			// assert!(LEGAL_WORDS.contains(guess_str), "illegal guess \"{guess_str}\"");

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

impl Display for Wordle {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.dictionary)
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DictionaryEntry {
	pub word: Word,
	pub count: usize,
}

impl Display for DictionaryEntry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		f.write_str(std::str::from_utf8(self.word).unwrap())
	}
}
