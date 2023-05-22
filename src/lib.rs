mod correctness;
pub use correctness::{Correctness, CorrectnessPattern};

mod wordle;
pub use wordle::{Dictionary, Word, ANSWERS, DICTIONARY, LEGAL_WORDS};

mod guesser;
pub use guesser::{Guess, Guesser};
