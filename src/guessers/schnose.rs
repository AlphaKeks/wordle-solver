use lazy_static::lazy_static;
use std::borrow::Cow;
use wordle_solver::{Correctness, CorrectnessPattern, Dictionary, Guesser, Word, DICTIONARY};

lazy_static! {
	pub static ref PATTERNS: Vec<CorrectnessPattern> = Correctness::patterns().collect();
}

#[derive(Debug)]
pub struct Schnose {
	dictionary: Cow<'static, Dictionary>,
	guess_history: Vec<Word>,
	patterns: Cow<'static, Vec<CorrectnessPattern>>,
}

impl Default for Schnose {
	fn default() -> Self {
		Self {
			dictionary: Cow::Borrowed(&DICTIONARY),
			guess_history: Vec::new(),
			patterns: Cow::Borrowed(&PATTERNS),
		}
	}
}

impl Guesser for Schnose {
	fn guess(&mut self) -> wordle_solver::Word {
		todo!()
	}
}
