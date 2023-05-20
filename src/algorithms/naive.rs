use crate::{Guess, Guesser};

pub struct NaiveGuesser;

impl Guesser for NaiveGuesser {
	fn guess(&mut self, _history: &[Guess]) -> String {
		todo!()
	}
}
