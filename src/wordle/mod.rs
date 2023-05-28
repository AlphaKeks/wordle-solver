mod iter;
pub use iter::WordleIter;

mod macros;
pub(crate) use macros::bytes_to_string;

use crate::{correctness::Correctness, guess::Guess, guesser::Guesser};
use hashbrown::HashSet;
use itertools::Itertools;
use lazy_static::lazy_static;

pub type Word = &'static [u8; 5];

lazy_static! {
	pub static ref DICTIONARY: Vec<(Word, usize)> = {
		include_str!("../../data/dictionary.txt")
			.lines()
			.map(|line| {
				let (word, count) = line
					.split_once(' ')
					.expect("Every line in the dictionary should be `word count`.");

				let word = word
					.as_bytes()
					.try_into()
					.expect("Every word in the dictionary should be 5 ASCII characters.");

				let count = count
					.parse()
					.expect("Every count should fit into `usize::MAX`.");

				(word, count)
			})
			.sorted_unstable_by_key(|&(_, count)| std::cmp::Reverse(count))
			.collect()
	};
	pub static ref ANSWERS: Vec<Word> = {
		include_str!("../../data/answers.txt")
			.lines()
			.map(|line| {
				line.as_bytes()
					.try_into()
					.expect("Every word in the dictionary should be 5 ASCII characters.")
			})
			.collect()
	};
}

#[derive(Debug, Clone)]
pub struct Wordle {
	dictionary: HashSet<Word>,
	answers: &'static [Word],
	guesser: Guesser,
}

impl Wordle {
	pub fn new(max_attempts: usize) -> Self {
		Self {
			dictionary: DICTIONARY
				.iter()
				.map(|(word, _)| *word)
				.collect(),
			answers: &ANSWERS,
			guesser: Guesser::new(max_attempts),
		}
	}

	pub fn play(&mut self, answer: Word) -> Option<usize> {
		self.guesser.reset();

		for round in 1..=self.guesser.max_attempts {
			let guess = self.guesser.guess();

			assert!(
				self.dictionary.contains(guess),
				"Illegal guess \"{}\".",
				bytes_to_string!(guess)
			);

			if guess == answer {
				return Some(round);
			}

			self.guesser.history.push(Guess {
				word: guess,
				correctness: Correctness::compute(answer, guess),
			});
		}

		None
	}

	pub fn iter(&self) -> WordleIter {
		self.into_iter()
	}
}
