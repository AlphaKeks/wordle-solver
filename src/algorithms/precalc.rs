use crate::{Correctness, Guess, Guesser, Word, DICTIONARY};
use once_cell::sync::OnceCell;
use std::{borrow::Cow, collections::BTreeMap};
use tracing::{debug, trace};

static DICT: OnceCell<Vec<(Word, usize)>> = OnceCell::new();

#[allow(clippy::type_complexity)]
static MATCH: OnceCell<BTreeMap<(Word, Word, [Correctness; 5]), bool>> = OnceCell::new();

pub struct PrecalcGuesser {
	dict: Cow<'static, Vec<(Word, usize)>>,
	remaining_count: usize,
}

impl PrecalcGuesser {
	fn update_remaining_count(&mut self) {
		self.remaining_count = self
			.dict
			.iter()
			.map(|(_, count)| count)
			.sum();
	}

	fn compute_score(&self, word: &str) -> f64 {
		// TODO: Don't consider `Correctness` patterns that had no candidates in the previous
		// iteration.
		0.0 - Correctness::permutations().fold(0.0, |sum, pattern| {
			let mut in_pattern_total = 0.0;

			for &(candidate, count) in self.dict.iter() {
				let matches = MATCH.get_or_init(|| self.compute_matches(candidate));

				let key = if word < candidate {
					(word, candidate, pattern)
				} else {
					(candidate, word, pattern)
				};

				if matches
					.get(&key)
					.copied()
					.unwrap_or_else(|| {
						Guess {
							word: Cow::Borrowed(word),
							correctness: pattern,
						}
						.matches(candidate)
					}) {
					in_pattern_total += count as f64;
				}
			}

			if in_pattern_total == 0.0 {
				return sum;
			}

			// TODO: Weight this by p_word.
			let p_of_this_pattern = in_pattern_total / (self.remaining_count as f64);
			sum + (p_of_this_pattern * p_of_this_pattern.log2())
		})
	}

	fn compute_matches(&self, candidate: &str) -> BTreeMap<(Word, Word, [Correctness; 5]), bool> {
		let words = &DICT.get().unwrap()[..512];
		let mut matches = BTreeMap::new();

		for &(dict_word1, _) in words {
			for &(dict_word2, _) in words {
				if dict_word2 < dict_word1 {
					break;
				}

				for pattern in Correctness::permutations() {
					let guess = Guess {
						word: Cow::Borrowed(dict_word1),
						correctness: pattern,
					};

					matches.insert((dict_word1, dict_word2, pattern), guess.matches(candidate));
				}
			}
		}

		matches
	}
}

impl Default for PrecalcGuesser {
	fn default() -> Self {
		let dict = DICTIONARY
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

		let dict = DICT.get_or_init(|| {
			let mut words = dict;
			words.sort_unstable_by(|a, b| b.1.cmp(&a.1));
			words
		});

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

impl Guesser for PrecalcGuesser {
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

		for (word, _) in remaining {
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
