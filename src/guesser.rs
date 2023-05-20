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

		Correctness::compute(cmp, &self.word) == self.correctness
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

		#[test]
		fn caused_panic_lol() {
			check!("tares" + [I M M I I] disallows "brink");
		}
	}
}
