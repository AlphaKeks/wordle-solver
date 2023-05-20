use std::time::Instant;

use clap::{Parser, ValueEnum};
use tracing::{error, info, Level};
use wordle_solver::{algorithms, Guesser, Wordle};

const GAMES: &str = include_str!("../data/answers.txt");
const MAX_ATTEMPTS: usize = usize::MAX;

fn main() {
	let args = Args::parse();

	tracing_subscriber::fmt()
		.compact()
		.with_max_level(args.log_level)
		.without_time()
		.init();

	let took = match args.implementation {
		Implementation::Naive => play::<algorithms::NaiveGuesser>(args.max_games),
	}
	.elapsed();

	match args.max_games {
		1 => info!("Played 1 game in {took:.2?}."),
		n => info!("Played {n} games in {took:.2?}."),
	};
}

fn play<G: Guesser + Default>(max_games: usize) -> Instant {
	let wordle = Wordle::new();

	match max_games {
		1 => info!("Playing a game!"),
		n => info!("Playing {n} games!"),
	};

	let start = Instant::now();

	for answer in GAMES.lines().take(max_games) {
		let start = Instant::now();
		if let Some(n_attempts) = wordle.play::<MAX_ATTEMPTS>(answer, G::default()) {
			let took = start.elapsed();
			info!("Guessed \"{answer}\" in {n_attempts} attempts. (took {took:.2?})");
		} else {
			let took = start.elapsed();
			error!("Did not guess \"{answer}\" in <={MAX_ATTEMPTS} attempts. (took {took:.2?})");
		}
	}

	start
}

#[derive(Parser)]
struct Args {
	/// The implementation to use.
	#[arg(long = "impl")]
	#[clap(default_value = "naive")]
	implementation: Implementation,

	/// `RUST_LOG` level
	#[arg(long = "logs")]
	#[clap(default_value = "DEBUG")]
	log_level: Level,

	/// The amount of games to play
	#[arg(long = "games")]
	#[clap(default_value = "1")]
	max_games: usize,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Implementation {
	Naive,
}
