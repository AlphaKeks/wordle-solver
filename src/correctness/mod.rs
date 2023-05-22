use crate::Word;
use itertools::iproduct;

/// The correctness of a guess in Wordle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Correctness {
	/// Green
	Correct,
	/// Yellow
	Misplaced,
	/// Gray
	Incorrect,
}

pub type CorrectnessPattern = [Correctness; 5];

impl Correctness {
	/// Generates the cartesian product for [`Correctness`].
	#[rustfmt::skip]
	pub fn patterns() -> impl Iterator<Item = CorrectnessPattern> {
		iproduct! {
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect],
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect],
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect],
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect],
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect]
		}
		.map(|(a, b, c, d, e)| [a, b, c, d, e])
	}

	#[inline(always)]
	pub fn is_correct(&self) -> bool {
		matches!(self, Correctness::Correct)
	}

	#[inline(always)]
	pub fn is_misplaced(&self) -> bool {
		matches!(self, Correctness::Misplaced)
	}

	#[inline(always)]
	pub fn is_incorrect(&self) -> bool {
		matches!(self, Correctness::Incorrect)
	}

	/// Calculates the correctness of a guess, given the actual answer.
	#[inline]
	pub fn compute(guess: Word, answer: Word) -> CorrectnessPattern {
		let mut cmask = [Correctness::Incorrect; 5];

		// Mark correct characters.
		for (i, (answer_char, guess_char)) in
			std::iter::zip(answer.bytes(), guess.bytes()).enumerate()
		{
			if answer_char == guess_char {
				cmask[i] = Correctness::Correct;
			}
		}

		let mut already_marked = [false; 5];

		// Keep track of characters that are considered "correct" so we can ignore them later on.
		for (i, correctness) in cmask.iter().enumerate() {
			if correctness.is_correct() {
				already_marked[i] = true;
			}
		}

		// Mark misplaced characters.
		for (i, guess_char) in guess.bytes().enumerate() {
			// Ignore correct characters.
			if cmask[i].is_correct() {
				continue;
			}

			// If any of the characters appear somewhere else in the word and have _not_ yet been
			// marked, that means they are misplaced.
			if std::iter::zip(answer.bytes(), already_marked.iter_mut()).any(|(check, marked)| {
				if check == guess_char && !*marked {
					*marked = true;
					return true;
				}

				false
			}) {
				cmask[i] = Correctness::Misplaced;
			}
		}

		cmask
	}
}

use std::cmp::Ordering;

impl PartialOrd for Correctness {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(match self {
			Correctness::Correct => match other {
				Correctness::Correct => Ordering::Equal,
				Correctness::Misplaced => Ordering::Greater,
				Correctness::Incorrect => Ordering::Greater,
			},
			Correctness::Misplaced => match other {
				Correctness::Correct => Ordering::Less,
				Correctness::Misplaced => Ordering::Equal,
				Correctness::Incorrect => Ordering::Greater,
			},
			Correctness::Incorrect => match other {
				Correctness::Correct => Ordering::Less,
				Correctness::Misplaced => Ordering::Less,
				Correctness::Incorrect => Ordering::Equal,
			},
		})
	}
}

impl Ord for Correctness {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other)
			.expect("The `PartialOrd` implementation should always yield `Some`.")
	}
}

#[cfg(test)]
pub(crate) mod tests;
