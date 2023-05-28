use crate::{correctness::Correctness, wordle::Word};

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy)]
pub struct Guess {
	pub word: Word,
	pub correctness: [Correctness; 5],
}

impl Guess {
	pub fn allows(&self, other: Word) -> bool {
		Correctness::compute(other, self.word) == self.correctness
	}
}
