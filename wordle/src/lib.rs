use std::fmt::Display;

/// A single guess emitted by a [`Guesser`]
#[derive(Debug, Clone, PartialEq)]
pub struct Guess<Word, Correctness> {
	/// The guessed word
	pub word: Word,
	/// How correct this guess was compared to a previous one
	pub correctness: Correctness,
}

impl<W: Copy, C: Copy> Copy for Guess<W, C> {}

pub trait Guesser
where
	Self: Sized,
{
	type Wordle: self::Wordle<Self>;

	type Word: PartialEq + Display;
	type Correctness;

	fn new() -> Self;

	/// Makes a single guess given a history of previous guesses
	fn guess(&mut self, guess_history: &[Guess<Self::Word, Self::Correctness>]) -> Self::Word;
}

pub trait Wordle<Guesser: self::Guesser> {
	fn new() -> Self;

	/// Produces an answer for the [`Guesser`] to guess
	fn next_answer(&self) -> Option<Guesser::Word>;

	/// Plays a single game of wordle, given a [`Guesser`] and a limit of attempts
	fn play(&self, guesser: Guesser, max_attempts: usize) -> Option<usize>;
}
