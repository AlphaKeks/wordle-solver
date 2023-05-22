macro_rules! cmask {
	(C) => { $crate::Correctness::Correct };
	(M) => { $crate::Correctness::Misplaced };
	(I) => { $crate::Correctness::Incorrect };
	($($c:tt)+) => [[$(cmask!($c)),+]];
}

pub(crate) use cmask;

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
