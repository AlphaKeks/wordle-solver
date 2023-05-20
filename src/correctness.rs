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
			std::iter::zip(answer.bytes(), guess.bytes()).enumerate()
		{
			if answer_char == guess_char {
				correctness_mask[idx] = Correctness::Correct;
			}
		}

		// keep track of the characters we already marked as `Correct`
		let mut marked = [false; 5];
		for (idx, &correctness) in correctness_mask.iter().enumerate() {
			if correctness.is_correct() {
				marked[idx] = true;
			}
		}

		// mark misplaced characters
		for (idx, guess_char) in guess.bytes().enumerate() {
			// skip already marked characters
			if correctness_mask[idx].is_correct() {
				continue;
			}

			// check if there are other characters in the `answer` that are the same character we are
			// currently checking (`guess_char`) => mark as `Misplaced`
			if answer
				.bytes()
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

	pub fn is_correct(&self) -> bool {
		matches!(self, Correctness::Correct)
	}

	pub fn is_misplaced(&self) -> bool {
		matches!(self, Correctness::Misplaced)
	}

	pub fn is_incorrect(&self) -> bool {
		matches!(self, Correctness::Incorrect)
	}

	#[rustfmt::skip]
	pub fn permutations() -> impl Iterator<Item = [Self; 5]> {
		itertools::iproduct!(
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect],
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect],
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect],
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect],
			[Correctness::Correct, Correctness::Misplaced, Correctness::Incorrect]
		)
		.map(|(a, b, c, d, e)| [a, b, c, d, e])
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

/// # Example
///
/// ```
/// cmask![C C M I C] // turns into `[Correct, Correct, Misplaced, Incorrect, Correct]`
/// ```
#[cfg(test)]
macro_rules! cmask {
	(C) => { $crate::Correctness::Correct };
	(M) => { $crate::Correctness::Misplaced };
	(I) => { $crate::Correctness::Incorrect };
	($($c:tt)+) => [[$(cmask!($c)),+]];
}

#[cfg(test)]
pub(crate) use cmask;

#[cfg(test)]
mod tests {
	mod game {
		use crate::Wordle;

		macro_rules! guesser {
			(|$history:ident| $impl:block) => {{
				struct G;

				impl $crate::Guesser for G {
					fn guess(&mut self, $history: &[$crate::Guess]) -> String {
						$impl
					}
				}

				G
			}};
		}

		const MAX_ROUNDS: usize = 6;

		#[test]
		fn genius() {
			let w = Wordle::new();
			let g = guesser!(|_history| { String::from("moved") });
			let result = w.play("moved", g, MAX_ROUNDS);
			assert_eq!(result, Some(1));
		}

		#[test]
		fn magnificent() {
			let w = Wordle::new();
			let g = guesser!(|history| {
				if history.len() == 1 {
					return String::from("right");
				}
				String::from("wrong")
			});
			let result = w.play("right", g, MAX_ROUNDS);
			assert_eq!(result, Some(2));
		}

		#[test]
		fn impressive() {
			let w = Wordle::new();
			let g = guesser!(|history| {
				if history.len() == 2 {
					return String::from("right");
				}
				String::from("wrong")
			});
			let result = w.play("right", g, MAX_ROUNDS);
			assert_eq!(result, Some(3));
		}

		#[test]
		fn splendid() {
			let w = Wordle::new();
			let g = guesser!(|history| {
				if history.len() == 3 {
					return String::from("right");
				}
				String::from("wrong")
			});
			let result = w.play("right", g, MAX_ROUNDS);
			assert_eq!(result, Some(4));
		}

		#[test]
		fn great() {
			let w = Wordle::new();
			let g = guesser!(|history| {
				if history.len() == 4 {
					return String::from("right");
				}
				String::from("wrong")
			});
			let result = w.play("right", g, MAX_ROUNDS);
			assert_eq!(result, Some(5));
		}

		#[test]
		fn phew() {
			let w = Wordle::new();
			let g = guesser!(|history| {
				if history.len() == 5 {
					return String::from("right");
				}
				String::from("wrong")
			});
			let result = w.play("right", g, MAX_ROUNDS);
			assert_eq!(result, Some(6));
		}

		#[test]
		fn nope() {
			let w = Wordle::new();
			let g = guesser!(|_history| { String::from("wrong") });
			let result = w.play("right", g, MAX_ROUNDS);
			assert_eq!(result, None);
		}
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
