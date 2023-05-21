use crate::{print_word, Guesser};
use itertools::Itertools;
use once_cell::sync::OnceCell;
use std::{borrow::Cow, collections::HashSet, fmt::Display};
use tracing::{debug, trace};

/// All words allowed by Wordle + their occurrence count according Google Books
/// (latest year per book)
pub static LEGAL_WORDS: &str = include_str!("../../data/words/legal-words.txt");

/// Answers for the official Wordle game (March 5th 2022)
pub static ANSWERS: &str = include_str!("../../data/words/wordle-answers.txt");

pub(crate) static DICTIONARY_WORDS: OnceCell<Vec<DictionaryEntry>> = OnceCell::new();

pub type Word = [u8; 5];

/// The dictionary used for playing Wordle.
///
/// It contains a list of _legal_ words together with ther total count according to Google Books
/// and relative frequency.
#[derive(Debug, Clone, PartialEq)]
pub struct Dictionary<'dict> {
	pub words: Cow<'dict, Vec<DictionaryEntry>>,
	pub legal_words: HashSet<Word>,
}

#[allow(clippy::new_without_default)]
impl Dictionary<'_> {
	/// Initializes a new [`Dictionary`], re-using a cache of dictionary words until it is required
	/// to be changed (i.e. by pruning).
	pub fn new() -> Self {
		let legal_words = LEGAL_WORDS
			.lines()
			.map(|line| -> (Word, usize) {
				// Example line:
				//
				// hello 21260257
				let (word, count) = line
					.split_once(' ')
					.expect("Every line in the dictionary should be `word count`.");

				// Every word consists of 5 ASCII characters, so we turn it into bytes so we don't have
				// to carry around strings.
				let word = word
					.as_bytes()
					.try_into()
					.expect("Every word should be 5 ASCII characters.");

				let count = count
					.parse()
					.expect("Every count should fit in `usize::MAX`.");

				(word, count)
			})
			.collect_vec();

		let total_word_count: usize = legal_words
			.iter()
			.map(|&(_, count)| count)
			.sum();

		let words = Cow::Borrowed(DICTIONARY_WORDS.get_or_init(|| {
			legal_words
				.into_iter()
				.map(|(word, count)| {
					// TODO: Apply sigmoid to improve the average quality of guesses.
					let frequency = count as f64 / total_word_count as f64;

					DictionaryEntry { word, count, frequency }
				})
				.sorted_by_key(|entry| std::cmp::Reverse(entry.count))
				.collect_vec()
		}));

		let legal_words = HashSet::from_iter(words.iter().map(|entry| entry.word));

		Self { words, legal_words }
	}
}

impl Dictionary<'_> {
	/// Plays a game of Wordle against the given [`Guesser`] and.
	/// If the [`Guesser`] fails to guess the `answer` in [`Guesser::MAX_ATTEMPTS`], this function
	/// will return [`None`].
	/// Otherwise it will return [`Some`] with the amount of required guesses.
	pub fn play<G: Guesser>(&self, mut guesser: G, answer: Word) -> Option<usize> {
		for round in 1..=G::MAX_ATTEMPTS {
			trace!("[Round {} / {}] Making a guess...", round, G::MAX_ATTEMPTS);

			// Make the guess
			let guess = guesser.guess();
			debug!("Guessed `{}`.", print_word!(guess));

			// Ensure the guess is actually legal.
			// [`debug_assert`] to prevent this from hurting performance in release mode.
			debug_assert!(self.legal_words.contains(&guess));

			// We guessed correctly!
			if guess == answer {
				return Some(round);
			}
		}

		None
	}
}

impl Display for Dictionary<'_> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:?}", self.words)
	}
}

/// A single entry in the [`Dictionary`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DictionaryEntry {
	pub word: Word,
	pub count: usize,
	pub frequency: f64,
}

impl Display for DictionaryEntry {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let word = std::str::from_utf8(&self.word).expect("Every word is 5 ASCII characters.");
		write!(f, "{word}")
	}
}
