macro_rules! bytes_to_string {
	($word:expr) => {
		String::from_iter($word.map(|c| c as char))
	};
}

pub(crate) use bytes_to_string;
