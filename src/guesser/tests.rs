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
		ensure!(b"abcde" + [C C C C C] allows b"abcde");
		ensure!(b"abcde" + [C C C C C] disallows b"abcdf");
		ensure!(b"fghij" + [I I I I I] allows b"abcde");
		ensure!(b"bcdea" + [M M M M M] allows b"abcde");
		ensure!(b"accaa" + [C M I I I] disallows b"aaabb");
		ensure!(b"caacc" + [I C M I I] disallows b"baaaa");
		ensure!(b"bcdea" + [I I I I I] disallows b"abcde");
		ensure!(b"brink" + [I M M I I] disallows b"tares");
		ensure!(b"aaccc" + [I C M I I] allows b"baaaa");
	}
}
