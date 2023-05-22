macro_rules! cmask {
	(C) => { $crate::Correctness::Correct };
	(M) => { $crate::Correctness::Misplaced };
	(I) => { $crate::Correctness::Incorrect };
	($($c:tt)+) => [[$(cmask!($c)),+]];
}

pub(crate) use cmask;

mod game {
	use crate::DICTIONARY;

	macro_rules! guesser {
		(|$self:ident| $impl:block) => {{
			#[allow(dead_code)]
			#[derive(Debug, Default)]
			struct G {
				history: Vec<()>,
			}

			impl $crate::Guesser for G {
				fn guess(&mut $self) -> $crate::Word {
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
		let g = guesser!(|self| { "right" });
		let result = w.play(g, "right", MAX_ATTEMPTS);
		assert_eq!(result, Some(1));
	}

	#[test]
	fn magnificent() {
		let w = &DICTIONARY;
		let g = guesser!(|self| {
			let guess = if self.history.len() == 1 { "right" } else { "wrong" };
			self.history.push(());
			guess
		});
		let result = w.play(g, "right", MAX_ATTEMPTS);
		assert_eq!(result, Some(2));
	}

	#[test]
	fn impressive() {
		let w = &DICTIONARY;
		let g = guesser!(|self| {
			let guess = if self.history.len() == 2 { "right" } else { "wrong" };
			self.history.push(());
			guess
		});
		let result = w.play(g, "right", MAX_ATTEMPTS);
		assert_eq!(result, Some(3));
	}

	#[test]
	fn splendid() {
		let w = &DICTIONARY;
		let g = guesser!(|self| {
			let guess = if self.history.len() == 3 { "right" } else { "wrong" };
			self.history.push(());
			guess
		});
		let result = w.play(g, "right", MAX_ATTEMPTS);
		assert_eq!(result, Some(4));
	}

	#[test]
	fn great() {
		let w = &DICTIONARY;
		let g = guesser!(|self| {
			let guess = if self.history.len() == 4 { "right" } else { "wrong" };
			self.history.push(());
			guess
		});
		let result = w.play(g, "right", MAX_ATTEMPTS);
		assert_eq!(result, Some(5));
	}

	#[test]
	fn phew() {
		let w = &DICTIONARY;
		let g = guesser!(|self| {
			let guess = if self.history.len() == 5 { "right" } else { "wrong" };
			self.history.push(());
			guess
		});
		let result = w.play(g, "right", MAX_ATTEMPTS);
		assert_eq!(result, Some(6));
	}

	#[test]
	fn failed() {
		let w = &DICTIONARY;
		let g = guesser!(|self| {
			let guess = "wrong";
			self.history.push(());
			guess
		});
		let result = w.play(g, "right", MAX_ATTEMPTS);
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
		assert_eq!(Correctness::compute("bcdea", "abcde"), cmask![M M M M M]);
	}

	#[test]
	fn all_incorrect() {
		assert_eq!(Correctness::compute("fghij", "abcde"), cmask![I I I I I]);
	}

	#[test]
	fn repeat_correct() {
		assert_eq!(Correctness::compute("aaccc", "aabbb"), cmask![C C I I I]);
	}

	#[test]
	fn repeat_misplaced() {
		assert_eq!(Correctness::compute("ccaac", "aabbb"), cmask![I I M M I]);
	}

	#[test]
	fn repeat_some_correct() {
		assert_eq!(Correctness::compute("caacc", "aabbb"), cmask![I C M I I]);
	}

	#[test]
	fn random() {
		assert_eq!(Correctness::compute("aaabb", "azzaz"), cmask![C M I I I]);
		assert_eq!(Correctness::compute("aaddd", "baccc"), cmask![I C I I I]);
		assert_eq!(Correctness::compute("aacde", "abcde"), cmask![C I C C C]);
	}
}
