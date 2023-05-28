use crate::{
	correctness::{Correctness, PATTERNS},
	guess::Guess,
	wordle::{Word, DICTIONARY},
};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Guesser {
	pub dictionary: Cow<'static, Vec<(Word, usize)>>,
	pub patterns: Cow<'static, Vec<[Correctness; 5]>>,
	pub history: Vec<Guess>,
	pub max_attempts: usize,
}

impl Guesser {
	/// Constructs a new [`Guesser`].
	pub fn new(max_attempts: usize) -> Self {
		Self {
			dictionary: Cow::Borrowed(&DICTIONARY),
			patterns: Cow::Borrowed(&PATTERNS),
			history: Vec::with_capacity(max_attempts),
			max_attempts,
		}
	}

	pub fn reset(&mut self) {
		self.dictionary = Cow::Borrowed(&DICTIONARY);
		self.patterns = Cow::Borrowed(&PATTERNS);
		self.history = Vec::new();
	}

	/// Makes a guess.
	pub fn guess(&mut self) -> Word {
		// Eliminate any entries in the dictionary we won't need anymore
		self.prune();

		if self.history.is_empty() {
			return b"tares";
		}

		let mut best_candidate = {
			let (first_word, _) = self
				.dictionary
				.first()
				.copied()
				.expect("Dictionary should never be empty.");

			(first_word, self.compute_score(first_word))
		};

		let remaining_count = self.measure_dict();
		let cutoff = (self.dictionary.len() / 3).max(20);
		let dictionary = self.dictionary.iter().take(cutoff);

		for &(word, count) in dictionary {
			let mut total_score = 0.0;

			let check_pattern = |pattern: &[Correctness; 5]| {
				let mut matching_words = 0;

				for &(candidate, count) in self.dictionary.iter() {
					let guess = Guess { word, correctness: *pattern };

					if guess.allows(candidate) {
						matching_words += count;
					}
				}

				if matching_words == 0 {
					return false;
				}

				let p = matching_words as f64 / remaining_count as f64;
				total_score += p * p.log2();
				true
			};

			match self.patterns {
				Cow::Owned(ref mut patterns) => {
					patterns.retain(check_pattern);
				}
				Cow::Borrowed(patterns) => {
					self.patterns = Cow::Owned(
						patterns
							.iter()
							.copied()
							.filter(check_pattern)
							.collect(),
					);
				}
			};

			let p_word = count as f64 / remaining_count as f64;
			let score = p_word * -total_score;

			if score > best_candidate.1 {
				best_candidate = (word, score);
			}
		}

		best_candidate.0
	}

	/// Returns the amount of words left in the dictionary.
	fn measure_dict(&self) -> usize {
		self.dictionary
			.iter()
			.map(|(_, count)| *count)
			.sum()
	}

	/// Removes entries from the dictionary by eliminating words that cannot possibly be the answer
	/// in order to cut down the search space.
	fn prune(&mut self) {
		let Some(last) = self.history.last() else {
			return;
		};

		match self.dictionary {
			Cow::Owned(ref mut dictionary) => {
				dictionary.retain(|(word, _)| last.allows(word));
			}
			Cow::Borrowed(dictionary) => {
				self.dictionary = Cow::Owned(
					dictionary
						.iter()
						.filter(|(word, _)| last.allows(word))
						.copied()
						.collect(),
				);
			}
		};
	}

	/// Computes the entropy of a given `word` with respect to the remaining patterns and entries
	/// in the dictionary.
	///
	/// [See Wikipedia](https://en.wikipedia.org/wiki/Information_theory#Entropy_of_an_information_source)
	fn compute_score(&self, word: Word) -> f64 {
		0.0 - self
			.patterns
			.iter()
			.fold(0.0, |total_score, pattern| {
				let mut matching_words = 0;

				for &(candidate, count) in self.dictionary.iter() {
					let guess = Guess { word, correctness: *pattern };

					if guess.allows(candidate) {
						matching_words += count;
					}
				}

				if matching_words == 0 {
					return total_score;
				}

				let p = matching_words as f64 / self.measure_dict() as f64;
				total_score + p * p.log2()
			})
	}
}
