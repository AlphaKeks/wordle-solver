mod naive;
pub use naive::NaiveGuesser;

mod less_allocs;
pub use less_allocs::LessAllocsGuesser;

mod vec_dict;
pub use vec_dict::VecDict;
