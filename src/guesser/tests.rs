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
		let g = guesser!(|_history| { "right" });
		let result = w.play(g, "right", MAX_ATTEMPTS);
		assert_eq!(result, Some(1));
	}

	#[test]
	fn magnificent() {
		let w = &DICTIONARY;
		let g = guesser!(|history| {
			if history.len() == 1 {
				"right"
			} else {
				"wrong"
			}
		});
		let result = w.play(g, "right", MAX_ATTEMPTS);
		assert_eq!(result, Some(2));
	}

	#[test]
	fn impressive() {
		let w = &DICTIONARY;
		let g = guesser!(|history| {
			if history.len() == 2 {
				"right"
			} else {
				"wrong"
			}
		});
		let result = w.play(g, "right", MAX_ATTEMPTS);
		assert_eq!(result, Some(3));
	}

	#[test]
	fn splendid() {
		let w = &DICTIONARY;
		let g = guesser!(|history| {
			if history.len() == 3 {
				"right"
			} else {
				"wrong"
			}
		});
		let result = w.play(g, "right", MAX_ATTEMPTS);
		assert_eq!(result, Some(4));
	}

	#[test]
	fn great() {
		let w = &DICTIONARY;
		let g = guesser!(|history| {
			if history.len() == 4 {
				"right"
			} else {
				"wrong"
			}
		});
		let result = w.play(g, "right", MAX_ATTEMPTS);
		assert_eq!(result, Some(5));
	}

	#[test]
	fn phew() {
		let w = &DICTIONARY;
		let g = guesser!(|history| {
			if history.len() == 5 {
				"right"
			} else {
				"wrong"
			}
		});
		let result = w.play(g, "right", MAX_ATTEMPTS);
		assert_eq!(result, Some(6));
	}

	#[test]
	fn failed() {
		let w = &DICTIONARY;
		let g = guesser!(|_history| { "wrong" });
		let result = w.play(g, "right", MAX_ATTEMPTS);
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
		ensure!("abcde" + [C C C C C] allows "abcde");
		ensure!("abcdf" + [C C C C C] disallows "abcde");
		ensure!("abcde" + [I I I I I] allows "fghij");
		ensure!("abcde" + [M M M M M] allows "bcdea");
		ensure!("aaabb" + [C M I I I] disallows "accaa");
		ensure!("baaaa" + [I C M I I] disallows "caacc");
		ensure!("abcde" + [I I I I I] disallows "bcdea");
		ensure!("tares" + [I M M I I] disallows "brink");
		ensure!("baaaa" + [I C M I I] allows "aaccc");
	}
}
