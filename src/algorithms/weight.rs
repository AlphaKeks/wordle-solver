use crate::{Correctness, Guess, Guesser, Word, DICTIONARY};
use once_cell::sync::OnceCell;
use std::borrow::Cow;
use tracing::{debug, trace};

static DICT: OnceCell<Vec<(Word, usize)>> = OnceCell::new();

pub struct WeightGuesser {
	dict: Cow<'static, Vec<(Word, usize)>>,
	remaining_count: usize,
}

impl WeightGuesser {
	fn update_remaining_count(&mut self) {
		self.remaining_count = self
			.dict
			.iter()
			.map(|(_, count)| count)
			.sum();
	}

	fn compute_score(&self, word: &str) -> f64 {
		0.0 - Correctness::permutations().fold(0.0, |sum, pattern| {
			let mut in_pattern_total = 0.0;

			for &(candidate, count) in self.dict.iter() {
				let guess = Guess {
					word: Cow::Borrowed(word),
					correctness: pattern,
				};

				if guess.matches(candidate) {
					in_pattern_total += count as f64;
				}
			}

			if in_pattern_total == 0.0 {
				return sum;
			}

			let p_of_this_pattern = in_pattern_total / (self.remaining_count as f64);
			sum + (p_of_this_pattern * p_of_this_pattern.log2())
		})
	}
}

impl Default for WeightGuesser {
	fn default() -> Self {
		let mut dict = DICTIONARY
			.lines()
			.map(|line| {
				let (word, count) = line
					.split_once(' ')
					.expect("line format of `word count`");

				let count = count
					.parse()
					.expect("count must be a number");

				(word, count)
			})
			.collect::<Vec<(&str, usize)>>();

		dict.sort_by_key(|w| w.0);

		let dict = DICT.get_or_init(|| dict);

		let mut guesser = Self {
			dict: Cow::Borrowed(dict),
			remaining_count: 0,
		};
		guesser.update_remaining_count();
		guesser
	}
}

#[derive(Debug, Clone, Copy)]
struct Candidate {
	word: Word,
	score: f64,
}

impl Guesser for WeightGuesser {
	fn guess(&mut self, history: &[Guess]) -> String {
		if let Some(last) = history.last() {
			if let Cow::Owned(dict) = &mut self.dict {
				dict.retain(|(word, _)| last.matches(word));
			} else {
				self.dict = Cow::Owned(
					self.dict
						.iter()
						.filter(|(word, _)| last.matches(word))
						.copied()
						.collect(),
				);
			}

			self.update_remaining_count();
		}

		if history.is_empty() {
			return String::from("tares");
		}

		let mut remaining = self.dict.iter();

		// setup initial candidate
		let mut best = {
			let (word, _) = remaining
				.next()
				.expect("dictionary is never empty");

			Candidate { word, score: self.compute_score(word) }
		};

		debug!("starting with {best:?}");

		for &(word, count) in remaining {
			trace!("progress");
			let p_word = count as f64 / self.remaining_count as f64;
			let score = p_word * self.compute_score(word);
			if score > best.score {
				debug!(%score, %best.score, "\"{}\" is better than \"{}\"", word, best.word);
				best = Candidate { word, score };
			}
		}

		best.word.to_owned()
	}
}
