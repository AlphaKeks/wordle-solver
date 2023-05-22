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
		assert_eq!(Correctness::compute(b"abcde", b"abcde"), cmask![C C C C C]);
	}

	#[test]
	fn all_misplaced() {
		assert_eq!(Correctness::compute(b"bcdea", b"abcde"), cmask![M M M M M]);
	}

	#[test]
	fn all_incorrect() {
		assert_eq!(Correctness::compute(b"fghij", b"abcde"), cmask![I I I I I]);
	}

	#[test]
	fn repeat_correct() {
		assert_eq!(Correctness::compute(b"aaccc", b"aabbb"), cmask![C C I I I]);
	}

	#[test]
	fn repeat_misplaced() {
		assert_eq!(Correctness::compute(b"ccaac", b"aabbb"), cmask![I I M M I]);
	}

	#[test]
	fn repeat_some_correct() {
		assert_eq!(Correctness::compute(b"caacc", b"aabbb"), cmask![I C M I I]);
	}

	#[test]
	fn random() {
		assert_eq!(Correctness::compute(b"aaabb", b"azzaz"), cmask![C M I I I]);
		assert_eq!(Correctness::compute(b"aaddd", b"baccc"), cmask![I C I I I]);
		assert_eq!(Correctness::compute(b"aacde", b"abcde"), cmask![C I C C C]);
	}
}
