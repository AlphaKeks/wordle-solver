use super::{Word, Wordle};

pub struct WordleIter {
	answers: &'static [Word],
	idx: usize,
}

impl Iterator for WordleIter {
	type Item = Word;

	fn next(&mut self) -> Option<Self::Item> {
		if self.idx >= self.answers.len() {
			return None;
		}

		let answer = self.answers[self.idx];
		self.idx += 1;
		Some(answer)
	}
}

impl IntoIterator for &Wordle {
	type Item = Word;
	type IntoIter = WordleIter;

	fn into_iter(self) -> Self::IntoIter {
		WordleIter { answers: self.answers, idx: 0 }
	}
}

impl IntoIterator for Wordle {
	type Item = Word;
	type IntoIter = WordleIter;

	fn into_iter(self) -> Self::IntoIter {
		WordleIter { answers: self.answers, idx: 0 }
	}
}
