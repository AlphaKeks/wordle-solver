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

impl Correctness {
	/// Generates the cartesian product for [`Correctness`].
	#[rustfmt::skip]
	pub fn patterns() -> impl Iterator<Item = [Correctness; 5]> {
		iproduct! {
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect],
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect],
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect],
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect],
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect]
		}
		.map(|(a, b, c, d, e)| [a, b, c, d, e])
	}

	/// Calculates the correctness of a guess, given the actual answer.
	#[inline]
	pub fn compute(guess: Word, answer: Word) -> [Correctness; 5] {
		let mut cmask = [Correctness::Incorrect; 5];
		let mut checked = [false; 5];

		// Mark correct characters.
		for (i, (answer_char, guess_char)) in std::iter::zip(answer, guess).enumerate() {
			if answer_char == guess_char {
				cmask[i] = Correctness::Correct;
				checked[i] = true;
			}
		}

		// Mark misplaced characters.
		for (i, guess_char) in guess.iter().enumerate() {
			// Ignore correct characters.
			if cmask[i] == Correctness::Correct {
				continue;
			}

			// If the current `guess_char` appears anywhere else in the word, mark it as
			// `Misplaced` and set the other occurrence as "checked".
			for (j, check) in answer.iter().enumerate() {
				if check == guess_char && !checked[j] {
					cmask[i] = Correctness::Misplaced;
					checked[j] = true;
					break;
				}
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
