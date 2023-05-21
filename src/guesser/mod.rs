use crate::{correctness::CorrectnessPattern, Correctness, Word};

/// A single guess emitted by a [`Guesser`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Guess<'word> {
	pub(crate) word: &'word Word,
	pub(crate) correctness: CorrectnessPattern,
}

impl Guess<'_> {
	/// Computes whether `other` should still be considered for future guesses, given the current
	/// guess.
	#[inline]
	pub fn allows(&self, other: &Word) -> bool {
		Correctness::compute(other, self.word) == self.correctness
	}
}

pub trait Guesser {
	/// The maximum amount of allowed guesses until the [`Guesser`] "fails".
	const MAX_ATTEMPTS: usize;

	/// A single guess by the [`Guesser`] that produces a [`Word`].
	fn guess(&mut self) -> Word;
}

impl<G: Guesser> Guesser for &mut G {
	const MAX_ATTEMPTS: usize = <G as Guesser>::MAX_ATTEMPTS;

	fn guess(&mut self) -> Word {
		<G as Guesser>::guess(self)
	}
}

#[cfg(test)]
pub(crate) mod tests;
