use clap::{Parser, ValueEnum};
use std::time::Instant;
use tracing::{error, info, Level};
use wordle_solver::algorithms::{LessAllocsGuesser, NaiveGuesser, OnceInit, VecDict};
use wordle_solver::{Guesser, Wordle};

const GAMES: &str = include_str!("../data/words/answers.txt");

fn main() {
	let Args {
		log_level,
		implementation,
		max_games,
		max_attempts,
	} = Args::parse();

	tracing_subscriber::fmt()
		.compact()
		.with_max_level(log_level)
		.with_timer(tracing_subscriber::fmt::time::uptime())
		.init();

	let took = match implementation {
		Implementation::Naive => play::<NaiveGuesser>(max_games, max_attempts),
		Implementation::LessAllocs => play::<LessAllocsGuesser>(max_games, max_attempts),
		Implementation::VecDict => play::<VecDict>(max_games, max_attempts),
		Implementation::OnceCell => play::<OnceInit>(max_games, max_attempts),
	}
	.elapsed();

	match max_games {
		1 => info!("Played 1 game in {took:.2?}."),
		n => info!("Played {n} games in {took:.2?}."),
	};
}

fn play<G: Guesser + Default>(max_games: usize, max_attempts: usize) -> Instant {
	let wordle = Wordle::new();

	match max_games {
		1 => info!("Playing a game!"),
		n => info!("Playing {n} games!"),
	};

	let start = Instant::now();

	for answer in GAMES.lines().take(max_games) {
		let start = Instant::now();
		if let Some(n_attempts) = wordle.play(answer, G::default(), max_attempts) {
			let took = start.elapsed();
			info!("Guessed \"{answer}\" in {n_attempts} attempts. (took {took:.2?})");
		} else {
			let took = start.elapsed();
			error!("Did not guess \"{answer}\" in <={max_attempts} attempts. (took {took:.2?})");
		}
	}

	start
}

#[derive(Parser)]
#[clap(version)]
struct Args {
	/// `RUST_LOG` level
	#[arg(long = "logs")]
	#[clap(default_value = "DEBUG")]
	log_level: Level,

	/// The implementation to use.
	#[arg(long = "impl")]
	#[clap(default_value = "naive")]
	implementation: Implementation,

	/// The amount of games to play
	#[arg(long = "games")]
	#[clap(default_value = "1")]
	max_games: usize,

	/// The maximum amount of attempts per guess (official Wordle allows 6)
	#[arg(long = "attempts")]
	#[clap(default_value = "18446744073709551615")]
	max_attempts: usize,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Implementation {
	Naive,
	LessAllocs,
	VecDict,
	OnceCell,
}
