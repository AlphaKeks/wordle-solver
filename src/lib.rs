mod correctness;
pub use correctness::Correctness;

mod wordle;
pub use wordle::{Dictionary, Word, ANSWERS, LEGAL_WORDS};

mod guesser;
pub use guesser::{Guess, Guesser};

/// Prints a [`Word`] as a [`str`].
macro_rules! print_word {
	($word:expr) => {
		std::str::from_utf8(&$word).unwrap_or("invalid word")
	};
}

pub(crate) use print_word;
