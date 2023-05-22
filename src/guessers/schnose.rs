use lazy_static::lazy_static;
use std::borrow::Cow;
use wordle_solver::{
	Correctness, CorrectnessPattern, Dictionary, Guess, Guesser, Word, DICTIONARY,
};

lazy_static! {
	pub static ref PATTERNS: Vec<CorrectnessPattern> = Correctness::patterns().collect();
}

#[derive(Debug)]
pub struct Schnose {
	dictionary: Cow<'static, Dictionary>,
	patterns: Cow<'static, Vec<CorrectnessPattern>>,
}

impl Default for Schnose {
	fn default() -> Self {
		Self {
			dictionary: Cow::Borrowed(&DICTIONARY),
			patterns: Cow::Borrowed(&PATTERNS),
		}
	}
}

impl Schnose {
	#[inline]
	fn prune_dictionary(&mut self, guess_history: &[Guess]) {
		if let Some(last) = guess_history.last() {
			if let Cow::Owned(_) = self.dictionary {
				self.dictionary
					.to_mut()
					.entries
					.retain(|entry| last.allows(entry.word));
			} else {
				self.dictionary = Cow::Owned(Dictionary {
					entries: self
						.dictionary
						.entries
						.iter()
						.filter(|entry| last.allows(entry.word))
						.copied()
						.collect(),
				})
			}
		}
	}

	#[inline]
	fn measure_dict(&self) -> usize {
		self.dictionary
			.entries
			.iter()
			.map(|entry| entry.count)
			.sum()
	}

	fn compute_score(&self, word: Word) -> f64 {
		0.0 - self
			.patterns
			.iter()
			.fold(0.0, |total_score, pattern| {
				let mut in_pattern_total = 0;

				for candidate in self.dictionary.entries.iter() {
					let guess = Guess { word, correctness: *pattern };

					if guess.allows(candidate.word) {
						in_pattern_total += candidate.count;
					}
				}

				if in_pattern_total == 0 {
					return total_score;
				}

				let p = in_pattern_total as f64 / self.measure_dict() as f64;
				total_score + p * p.log2()
			})
	}
}

#[derive(Debug, Clone, Copy)]
struct Candidate {
	word: Word,
	score: f64,
}

impl Guesser for Schnose {
	fn guess(&mut self, guess_history: &[Guess]) -> Word {
		self.prune_dictionary(guess_history);

		if guess_history.is_empty() {
			self.patterns = Cow::Borrowed(&PATTERNS);
			return "tares";
		} else {
			assert!(!self.patterns.is_empty());
		}

		let mut best_candidate = {
			let entry = self
				.dictionary
				.entries
				.first()
				.expect("Dictionary should never be empty.");

			Candidate {
				word: entry.word,
				score: self.compute_score(entry.word),
			}
		};

		let remaining = self.measure_dict();
		let cutoff = (self.dictionary.entries.len() / 3).max(20);
		let dictionary = self
			.dictionary
			.entries
			.iter()
			.take(cutoff);

		for entry in dictionary {
			let mut total_score = 0.0;

			let check_pattern = |pattern: &CorrectnessPattern| {
				let mut in_pattern_total = 0;

				for candidate in &self.dictionary.entries {
					let guess = Guess { word: entry.word, correctness: *pattern };

					if guess.allows(candidate.word) {
						in_pattern_total += candidate.count;
					}
				}

				if in_pattern_total == 0 {
					return false;
				}

				let p = in_pattern_total as f64 / remaining as f64;
				total_score += p * p.log2();

				true
			};

			if let Cow::Owned(_) = self.patterns {
				self.patterns
					.to_mut()
					.retain(check_pattern);
			} else {
				self.patterns = Cow::Owned(
					self.patterns
						.iter()
						.copied()
						.filter(check_pattern)
						.collect(),
				);
			}

			let p_word = entry.count as f64 / remaining as f64;
			let score = p_word * -total_score;

			if score > best_candidate.score {
				best_candidate = Candidate { word: entry.word, score };
			}
		}

		best_candidate.word
	}
}
