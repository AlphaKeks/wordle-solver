use crate::Correctness;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Guess {
	pub(crate) word: String,
	pub(crate) correctness: [Correctness; 5],
}

pub trait Guesser {
	fn guess(&mut self, history: &[Guess]) -> String;
}

impl<T: Guesser> Guesser for &mut T {
	fn guess(&mut self, history: &[Guess]) -> String {
		<T as Guesser>::guess(self, history)
	}
}
