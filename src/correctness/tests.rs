macro_rules! cmask {
	(C) => { $crate::Correctness::Correct };
	(M) => { $crate::Correctness::Misplaced };
	(I) => { $crate::Correctness::Incorrect };
	($($c:tt)+) => [[$(cmask!($c)),+]];
}

pub(crate) use cmask;

mod game {
	use crate::Dictionary;

	macro_rules! guesser {
		(|$self:ident| $impl:block) => {{
			#[allow(dead_code)]
			#[derive(Debug, Default)]
			struct G {
				history: Vec<()>,
			}

			impl $crate::Guesser for G {
				const MAX_ATTEMPTS: usize = 6;

				fn guess(&mut $self) -> $crate::Word {
					$impl
				}
			}

			G::default()
		}};
	}

	#[test]
	fn genius() {
		let w = Dictionary::new();
		let g = guesser!(|self| { *b"right" });
		let result = w.play(g, *b"right");
		assert_eq!(result, Some(1));
	}

	#[test]
	fn magnificent() {
		let w = Dictionary::new();
		let g = guesser!(|self| {
			let guess = if self.history.len() == 1 { *b"right" } else { *b"wrong" };
			self.history.push(());
			guess
		});
		let result = w.play(g, *b"right");
		assert_eq!(result, Some(2));
	}

	#[test]
	fn impressive() {
		let w = Dictionary::new();
		let g = guesser!(|self| {
			let guess = if self.history.len() == 2 { *b"right" } else { *b"wrong" };
			self.history.push(());
			guess
		});
		let result = w.play(g, *b"right");
		assert_eq!(result, Some(3));
	}

	#[test]
	fn splendid() {
		let w = Dictionary::new();
		let g = guesser!(|self| {
			let guess = if self.history.len() == 3 { *b"right" } else { *b"wrong" };
			self.history.push(());
			guess
		});
		let result = w.play(g, *b"right");
		assert_eq!(result, Some(4));
	}

	#[test]
	fn great() {
		let w = Dictionary::new();
		let g = guesser!(|self| {
			let guess = if self.history.len() == 4 { *b"right" } else { *b"wrong" };
			self.history.push(());
			guess
		});
		let result = w.play(g, *b"right");
		assert_eq!(result, Some(5));
	}

	#[test]
	fn phew() {
		let w = Dictionary::new();
		let g = guesser!(|self| {
			let guess = if self.history.len() == 5 { *b"right" } else { *b"wrong" };
			self.history.push(());
			guess
		});
		let result = w.play(g, *b"right");
		assert_eq!(result, Some(6));
	}

	#[test]
	fn failed() {
		let w = Dictionary::new();
		let g = guesser!(|self| {
			let guess = *b"wrong";
			self.history.push(());
			guess
		});
		let result = w.play(g, *b"right");
		assert_eq!(result, None);
	}
}

mod compute {
	use crate::Correctness;

	#[test]
	fn all_correct() {
		assert_eq!(Correctness::compute(b"abcde", b"abcde",), cmask![C C C C C]);
	}

	#[test]
	fn all_misplaced() {
		assert_eq!(Correctness::compute(b"bcdea", b"abcde",), cmask![M M M M M]);
	}

	#[test]
	fn all_incorrect() {
		assert_eq!(Correctness::compute(b"fghij", b"abcde",), cmask![I I I I I]);
	}

	#[test]
	fn repeat_correct() {
		assert_eq!(Correctness::compute(b"aaccc", b"aabbb",), cmask![C C I I I]);
	}

	#[test]
	fn repeat_misplaced() {
		assert_eq!(Correctness::compute(b"ccaac", b"aabbb",), cmask![I I M M I]);
	}

	#[test]
	fn repeat_some_correct() {
		assert_eq!(Correctness::compute(b"caacc", b"aabbb",), cmask![I C M I I]);
	}

	#[test]
	fn random() {
		assert_eq!(Correctness::compute(b"aaabb", b"azzaz",), cmask![C M I I I]);
		assert_eq!(Correctness::compute(b"aaddd", b"baccc",), cmask![I C I I I]);
		assert_eq!(Correctness::compute(b"aacde", b"abcde",), cmask![C I C C C]);
	}
}
