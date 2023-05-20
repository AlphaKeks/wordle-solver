use crate::{Guess, Guesser, DICTIONARY};
use std::collections::HashMap;

pub struct NaiveGuesser {
	remaining: HashMap<&'static str, usize>,
}

impl Default for NaiveGuesser {
	fn default() -> Self {
		Self {
			remaining: HashMap::from_iter(DICTIONARY.lines().map(|line| {
				let (word, count) = line
					.split_once(' ')
					.expect("line format of `word count`");

				let count = count
					.parse()
					.expect("count must be a number");

				(word, count)
			})),
		}
	}
}

impl Guesser for NaiveGuesser {
	fn guess(&mut self, _history: &[Guess]) -> String {
		todo!()
	}
}
