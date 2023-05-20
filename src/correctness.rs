use std::cmp::Ordering;

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
	pub fn compute(answer: &str, guess: &str) -> [Self; 5] {
		// sanity checks
		debug_assert_eq!(answer.len(), 5);
		debug_assert_eq!(guess.len(), 5);

		let mut correctness_mask = [Correctness::Incorrect; 5];

		// mark correct characters
		for (idx, (answer_char, guess_char)) in
			std::iter::zip(answer.chars(), guess.chars()).enumerate()
		{
			if answer_char == guess_char {
				correctness_mask[idx] = Correctness::Correct;
			}
		}

		// keep track of the characters we already marked as `Correct`
		let mut marked = [false; 5];
		for (idx, &correctness) in correctness_mask.iter().enumerate() {
			if correctness == Correctness::Correct {
				marked[idx] = true;
			}
		}

		// mark misplaced characters
		for (idx, guess_char) in guess.chars().enumerate() {
			// skip already marked characters
			if correctness_mask[idx] == Correctness::Correct {
				continue;
			}

			// check if there are other characters in the `answer` that are the same character we are
			// currently checking (`guess_char`) => mark as `Misplaced`
			if answer
				.chars()
				.enumerate()
				.any(|(dup_idx, dup_c)| {
					if dup_c == guess_char && !marked[dup_idx] {
						marked[dup_idx] = true;
						return true;
					}
					false
				}) {
				correctness_mask[idx] = Correctness::Misplaced;
			}
		}

		correctness_mask
	}
}

impl PartialOrd for Correctness {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self {
			Correctness::Correct => match other {
				Correctness::Correct => Some(Ordering::Equal),
				Correctness::Misplaced => Some(Ordering::Greater),
				Correctness::Incorrect => Some(Ordering::Greater),
			},
			Correctness::Misplaced => match other {
				Correctness::Correct => Some(Ordering::Less),
				Correctness::Misplaced => Some(Ordering::Equal),
				Correctness::Incorrect => Some(Ordering::Greater),
			},
			Correctness::Incorrect => match other {
				Correctness::Correct => Some(Ordering::Less),
				Correctness::Misplaced => Some(Ordering::Less),
				Correctness::Incorrect => Some(Ordering::Equal),
			},
		}
	}
}

impl Ord for Correctness {
	fn cmp(&self, other: &Self) -> Ordering {
		self.partial_cmp(other).unwrap()
	}
}

#[cfg(test)]
mod tests {
	/// # Example
	///
	/// ```
	/// cmask![C C M I C] // turns into `[Correct, Correct, Misplaced, Incorrect, Correct]`
	/// ```
	macro_rules! cmask {
		(C) => { $crate::Correctness::Correct };
		(M) => { $crate::Correctness::Misplaced };
		(I) => { $crate::Correctness::Incorrect };
		($($c:tt)+) => [[$(cmask!($c)),+]];
	}

	mod compute {
		use crate::Correctness;

		#[test]
		fn all_correct() {
			assert_eq!(Correctness::compute("abcde", "abcde"), cmask![C C C C C]);
		}

		#[test]
		fn all_misplaced() {
			assert_eq!(Correctness::compute("abcde", "bcdea"), cmask![M M M M M]);
		}

		#[test]
		fn all_incorrect() {
			assert_eq!(Correctness::compute("abcde", "fghij"), cmask![I I I I I]);
		}

		#[test]
		fn repeat_correct() {
			assert_eq!(Correctness::compute("aabbb", "aaccc"), cmask![C C I I I]);
		}

		#[test]
		fn repeat_misplaced() {
			assert_eq!(Correctness::compute("aabbb", "ccaac"), cmask![I I M M I]);
		}

		#[test]
		fn repeat_some_correct() {
			assert_eq!(Correctness::compute("aabbb", "caacc"), cmask![I C M I I]);
		}

		#[test]
		fn random() {
			assert_eq!(Correctness::compute("azzaz", "aaabb"), cmask![C M I I I]);
			assert_eq!(Correctness::compute("baccc", "aaddd"), cmask![I C I I I]);
			assert_eq!(Correctness::compute("abcde", "aacde"), cmask![C I C C C]);
		}
	}
}
