use crate::{Correctness, Guess, Guesser, DICTIONARY};
use std::{borrow::Cow, collections::HashMap};
use tracing::{debug, trace};

pub struct LessAllocsGuesser {
	remaining: HashMap<&'static str, usize>,
	remaining_count: usize,
}

impl LessAllocsGuesser {
	fn update_remaining_count(&mut self) {
		self.remaining_count = self
			.remaining
			.iter()
			.map(|(_, &count)| count)
			.sum();
	}

	fn compute_score(&self, word: &str) -> f64 {
		0.0 - Correctness::permutations().fold(0.0, |sum, pattern| {
			let mut in_pattern_total = 0.0;

			for (candidate, &count) in &self.remaining {
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

impl Default for LessAllocsGuesser {
	fn default() -> Self {
		let remaining = HashMap::from_iter(DICTIONARY.lines().map(|line| {
			let (word, count) = line
				.split_once(' ')
				.expect("line format of `word count`");

			let count = count
				.parse()
				.expect("count must be a number");

			(word, count)
		}));

		let mut guesser = Self { remaining, remaining_count: 0 };
		guesser.update_remaining_count();
		guesser
	}
}

#[derive(Debug, Clone, Copy)]
struct Candidate {
	word: &'static str,
	score: f64,
}

impl Guesser for LessAllocsGuesser {
	fn guess(&mut self, history: &[Guess]) -> String {
		if let Some(last) = history.last() {
			self.remaining
				.retain(|&word, _| last.matches(word));
			self.update_remaining_count();
		}

		if history.is_empty() {
			return String::from("tares");
		}

		let mut remaining = self.remaining.iter();

		// setup initial candidate
		let mut best = {
			let (&word, _) = remaining
				.next()
				.expect("dictionary is never empty");

			Candidate { word, score: self.compute_score(word) }
		};

		debug!("starting with {best:?}");

		for (&word, _) in remaining {
			trace!("progress");
			let score = self.compute_score(word);
			if score > best.score {
				debug!(%score, %best.score, "\"{}\" is better than \"{}\"", word, best.word);
				best = Candidate { word, score };
			}
		}

		best.word.to_owned()
	}
}
