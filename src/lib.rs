mod correctness;
pub use correctness::Correctness;

mod wordle;
pub use wordle::{Word, Wordle, ANSWERS, DICTIONARY, LEGAL_WORDS};

mod guesser;
pub use guesser::{Guess, Guesser};
