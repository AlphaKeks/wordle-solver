use wordle_solver::Guesser;

#[derive(Debug, Default)]
pub struct Schnose;

impl Guesser for Schnose {
	fn guess(&mut self) -> wordle_solver::Word {
		todo!()
	}
}
