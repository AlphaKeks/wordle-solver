#[cfg(test)]
pub(crate) mod tests;

use crate::wordle::Word;
use itertools::iproduct;
use lazy_static::lazy_static;

lazy_static! {
	pub static ref PATTERNS: Vec<[Correctness; 5]> = {
		iproduct! {
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect],
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect],
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect],
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect],
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect]
		}
		.map(|(a, b, c, d, e)| [a, b, c, d, e])
		.collect()
	};
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Correctness {
	/// Green
	Correct,
	/// Yellow
	Misplaced,
	/// Gray
	Incorrect,
}

impl Correctness {
	/// Computes how correct a `guess` is, given an `answer`.
	/// Each item in the returned array represents the [`Correctness`] of a letter in the `guess`.
	///
	/// We keep track of all characters in the `answer` that is not in the `guess` at the same
	/// position and count how often that is the case. Afterwards, when iterating over the `guess`
	/// again, we will check the count for each character in `misplaced` and if it is greater than
	/// 0, that must mean we encountered it in the `answer` previously. We mark it as `Misplaced`
	/// and decrement the count.
	#[inline]
	pub fn compute(answer: Word, guess: Word) -> [Self; 5] {
		let mut cmask = [Correctness::Incorrect; 5];
		let mut misplaced = [0u8; 26];

		for ((answer, guess), cmask) in std::iter::zip(answer, guess).zip(cmask.iter_mut()) {
			if answer == guess {
				*cmask = Correctness::Correct;
			} else {
				misplaced[(answer - b'a') as usize] += 1;
			}
		}

		for (guess, cmask) in std::iter::zip(guess, cmask.iter_mut()) {
			if *cmask == Correctness::Incorrect && misplaced[(guess - b'a') as usize] > 0 {
				*cmask = Correctness::Misplaced;
				misplaced[(guess - b'a') as usize] -= 1;
			}
		}

		cmask
	}
}
