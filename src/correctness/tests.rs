use super::Correctness;

macro_rules! cmask {
	(C) => { $crate::correctness::Correctness::Correct };
	(M) => { $crate::correctness::Correctness::Misplaced };
	(I) => { $crate::correctness::Correctness::Incorrect };
	($($c:tt)+) => [[$(cmask!($c)),+]];
}

pub(crate) use cmask;

#[test]
fn all_correct() {
	assert_eq!(Correctness::compute(b"abcde", b"abcde"), cmask![C C C C C]);
}

#[test]
fn all_misplaced() {
	assert_eq!(Correctness::compute(b"abcde", b"bcdea"), cmask![M M M M M]);
}

#[test]
fn all_incorrect() {
	assert_eq!(Correctness::compute(b"abcde", b"fghij"), cmask![I I I I I]);
}

#[test]
fn repeat_correct() {
	assert_eq!(Correctness::compute(b"aabbb", b"aaccc"), cmask![C C I I I]);
}

#[test]
fn repeat_misplaced() {
	assert_eq!(Correctness::compute(b"aabbb", b"ccaac"), cmask![I I M M I]);
}

#[test]
fn repeat_some_correct() {
	assert_eq!(Correctness::compute(b"aabbb", b"caacc"), cmask![I C M I I]);
}

#[test]
fn random() {
	assert_eq!(Correctness::compute(b"azzaz", b"aaabb"), cmask![C M I I I]);
	assert_eq!(Correctness::compute(b"baccc", b"aaddd"), cmask![I C I I I]);
	assert_eq!(Correctness::compute(b"abcde", b"aacde"), cmask![C I C C C]);
}
