use crate::correctness::tests::cmask;

macro_rules! ensure {
	($word:literal + [$($mask:tt)+] allows $guess:literal) => {
		assert!($crate::guess::Guess {
			word: $word,
			correctness: $crate::correctness::tests::cmask![$($mask )+],
			}
			.allows($guess)
		);
	};

	($word:literal + [$($mask:tt)+] disallows $guess:literal) => {
		assert!(!$crate::guess::Guess {
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
	ensure!(b"abcdf" + [C C C C C] disallows b"abcde");
	ensure!(b"abcde" + [I I I I I] allows b"fghij");
	ensure!(b"abcde" + [M M M M M] allows b"bcdea");
	ensure!(b"aaabb" + [C M I I I] disallows b"accaa");
	ensure!(b"baaaa" + [I C M I I] disallows b"caacc");
	ensure!(b"abcde" + [I I I I I] disallows b"bcdea");
	ensure!(b"tares" + [I M M I I] disallows b"brink");
	ensure!(b"baaaa" + [I C M I I] allows b"aaccc");
}
