mod matcher {
	use crate::{correctness::tests::cmask, Guess};

	macro_rules! ensure {
			($word:literal + [$($mask:tt)+] allows $guess:literal) => {
				assert!(Guess {
						word: $word,
						correctness: $crate::correctness::tests::cmask![$($mask )+],
					}
					.allows($guess)
				);
			};

			($word:literal + [$($mask:tt)+] disallows $guess:literal) => {
				assert!(!Guess {
						word: $word,
						correctness: $crate::correctness::tests::cmask![$($mask )+],
					}
					.allows($guess)
				);
			};
		}

	#[test]
	fn basic() {
		ensure!("abcde" + [C C C C C] allows "abcde");
		ensure!("abcde" + [C C C C C] disallows "abcdf");
		ensure!("fghij" + [I I I I I] allows "abcde");
		ensure!("bcdea" + [M M M M M] allows "abcde");
		ensure!("accaa" + [C M I I I] disallows "aaabb");
		ensure!("caacc" + [I C M I I] disallows "baaaa");
		ensure!("bcdea" + [I I I I I] disallows "abcde");
		ensure!("brink" + [I M M I I] disallows "tares");
		ensure!("aaccc" + [I C M I I] allows "baaaa");
	}
}
