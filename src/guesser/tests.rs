mod game {
	use crate::DICTIONARY;

	macro_rules! guesser {
		(|$history:ident| $impl:block) => {{
			#[allow(dead_code)]
			#[derive(Debug, Default)]
			struct G {
				history: Vec<()>,
			}

			impl $crate::Guesser for G {
				fn guess(&mut self, $history: &[$crate::Guess]) -> $crate::Word {
					$impl
				}
			}

			G::default()
		}};
	}

	const MAX_ATTEMPTS: usize = 6;

	#[test]
	fn genius() {
		let w = &DICTIONARY;
		let g = guesser!(|_history| { b"right" });
		let result = w.play(g, b"right", MAX_ATTEMPTS);
		assert_eq!(result, Some(1));
	}

	#[test]
	fn magnificent() {
		let w = &DICTIONARY;
		let g = guesser!(|history| {
			if history.len() == 1 {
				b"right"
			} else {
				b"wrong"
			}
		});
		let result = w.play(g, b"right", MAX_ATTEMPTS);
		assert_eq!(result, Some(2));
	}

	#[test]
	fn impressive() {
		let w = &DICTIONARY;
		let g = guesser!(|history| {
			if history.len() == 2 {
				b"right"
			} else {
				b"wrong"
			}
		});
		let result = w.play(g, b"right", MAX_ATTEMPTS);
		assert_eq!(result, Some(3));
	}

	#[test]
	fn splendid() {
		let w = &DICTIONARY;
		let g = guesser!(|history| {
			if history.len() == 3 {
				b"right"
			} else {
				b"wrong"
			}
		});
		let result = w.play(g, b"right", MAX_ATTEMPTS);
		assert_eq!(result, Some(4));
	}

	#[test]
	fn great() {
		let w = &DICTIONARY;
		let g = guesser!(|history| {
			if history.len() == 4 {
				b"right"
			} else {
				b"wrong"
			}
		});
		let result = w.play(g, b"right", MAX_ATTEMPTS);
		assert_eq!(result, Some(5));
	}

	#[test]
	fn phew() {
		let w = &DICTIONARY;
		let g = guesser!(|history| {
			if history.len() == 5 {
				b"right"
			} else {
				b"wrong"
			}
		});
		let result = w.play(g, b"right", MAX_ATTEMPTS);
		assert_eq!(result, Some(6));
	}

	#[test]
	fn failed() {
		let w = &DICTIONARY;
		let g = guesser!(|_history| { b"wrong" });
		let result = w.play(g, b"right", MAX_ATTEMPTS);
		assert_eq!(result, None);
	}
}

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
		ensure!(b"abcdf" + [C C C C C] disallows b"abcde");
		ensure!(b"abcde" + [I I I I I] allows b"fghij");
		ensure!(b"abcde" + [M M M M M] allows b"bcdea");
		ensure!(b"aaabb" + [C M I I I] disallows b"accaa");
		ensure!(b"baaaa" + [I C M I I] disallows b"caacc");
		ensure!(b"abcde" + [I I I I I] disallows b"bcdea");
		ensure!(b"tares" + [I M M I I] disallows b"brink");
		ensure!(b"baaaa" + [I C M I I] allows b"aaccc");
	}
}
