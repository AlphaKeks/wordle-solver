use crate::{correctness::CorrectnessPattern, Correctness, Word};

/// A single guess emitted by a [`Guesser`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Guess {
	pub word: Word,
	pub correctness: CorrectnessPattern,
}

impl Guess {
	/// Computes whether `other` should still be considered for future guesses, given the current
	/// guess.
	#[inline]
	pub fn allows(&self, other: Word) -> bool {
		Correctness::compute(self.word, other) == self.correctness
	}
}

pub trait Guesser {
	/// A single guess by the [`Guesser`] that produces a [`Word`].
	fn guess(&mut self, guess_history: &[Guess]) -> Word;
}

impl<G: Guesser> Guesser for &mut G {
	fn guess(&mut self, guess_history: &[Guess]) -> Word {
		<G as Guesser>::guess(self, guess_history)
	}
}

#[cfg(test)]
pub(crate) mod tests;
