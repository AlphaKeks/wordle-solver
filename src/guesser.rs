use std::borrow::Cow;

use crate::Correctness;

pub trait Guesser {
	fn guess(&mut self, history: &[Guess]) -> String;
}

impl<T: Guesser> Guesser for &mut T {
	fn guess(&mut self, history: &[Guess]) -> String {
		<T as Guesser>::guess(self, history)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Guess<'w> {
	pub(crate) word: Cow<'w, str>,
	pub(crate) correctness: [Correctness; 5],
}

impl Guess<'_> {
	pub fn matches(&self, cmp: &str) -> bool {
		// sanity checks
		debug_assert_eq!(self.word.len(), 5);
		debug_assert_eq!(cmp.len(), 5);

		let mut marked = [false; 5];

		// mark characters as `Correct`
		let prev_guess = self.word.bytes();
		let prev_correctness = self.correctness.iter();
		let cmp_guess = cmp.bytes();

		for (idx, ((prev_char, &prev_correctness), cmp_char)) in prev_guess
			.zip(prev_correctness)
			.zip(cmp_guess)
			.enumerate()
		{
			// current character of the old word is marked as `Correct`
			if prev_correctness.is_correct() {
				// new word has different character -> the two don't match
				// TODO: what about `Misplaced` characters?
				if prev_char != cmp_char {
					return false;
				} else {
					marked[idx] = true;
				}
			}
		}

		let prev_correctness = self.correctness.iter();
		let cmp_guess = cmp.bytes();

		for (idx, (cmp_char, &prev_correctness)) in
			std::iter::zip(cmp_guess, prev_correctness).enumerate()
		{
			if prev_correctness.is_correct() {
				continue;
			}

			let mut plausible = true;

			let found_misplaced = self
				.word
				.bytes()
				.zip(&self.correctness)
				.enumerate()
				.any(|(j, (char, correctness))| {
					if char != cmp_char {
						return false;
					}

					if marked[j] {
						return false;
					}

					// We are looking for a character in `cmp`, and have found that character in our
					// previous guess. The `correctness` of that previous character will tell us whether
					// this new character _might_ be worth considering.
					match correctness {
						Correctness::Correct => {
							unreachable!("All correct characters should have resulted in returning, or been marked.");
						}
						Correctness::Misplaced if idx == j => {
							// `cmp_char` was `Misplaced` in the same position last time, which means that
							// `cmp` _cannot_ possibly be the answer.
							plausible = false;
							false
						}
						Correctness::Misplaced => {
							marked[j] = true;
							true
						}
						Correctness::Incorrect => {
							plausible = false;
							false
						}
					}
				});

			if !found_misplaced && !plausible {
				return false;
			}
		}

		true
	}
}

#[cfg(test)]
mod tests {
	mod matcher {
		use crate::{correctness::cmask, Guess};
		use std::borrow::Cow;

		macro_rules! check {
			($word:literal + [$($mask:tt)+] allows $guess:literal) => {
				assert!(Guess {
						word: Cow::Borrowed($word),
						correctness: $crate::correctness::cmask![$($mask )+],
					}
					.matches($guess)
				);
			};

			($word:literal + [$($mask:tt)+] disallows $guess:literal) => {
				assert!(!Guess {
						word: Cow::Borrowed($word),
						correctness: $crate::correctness::cmask![$($mask )+],
					}
					.matches($guess)
				);
			};
		}

		#[test]
		fn basic() {
			check!("abcde" + [C C C C C] allows "abcde");
			check!("abcdf" + [C C C C C] disallows "abcde");
			check!("abcde" + [I I I I I] allows "fghij");
			check!("abcde" + [M M M M M] allows "bcdea");
			check!("aaabb" + [C M I I I] disallows "accaa");
			check!("baaaa" + [I C M I I] disallows "caacc");
			check!("baaaa" + [I C M I I] allows "aaccc");
			check!("abcde" + [I I I I I] disallows "bcdea");
		}
	}
}
