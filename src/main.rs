mod correctness;
mod guess;
mod guesser;
mod wordle;

use clap::Parser;
use std::time::Instant;
use tracing::{error, info, Level};
use tracing_subscriber::fmt::time::Uptime;
use wordle::Wordle;

use crate::wordle::bytes_to_string;

#[derive(Parser)]
struct Args {
	/// Print logs at the given level
	#[arg(long = "logs")]
	log_level: Option<Level>,

	/// Print the total elapsed time
	#[arg(long = "elapsed")]
	#[clap(default_value = "false")]
	show_elapsed: bool,

	/// The maximum amount of guesses a guesser is allowed to make before failing
	#[arg(long = "attempts")]
	#[clap(default_value = "6")]
	max_attempts: usize,

	/// The amount of games to play
	#[arg(long = "games")]
	#[clap(default_value = "1")]
	games: usize,

	/// Play in interactive mode
	#[arg(long)]
	#[clap(default_value = "false")]
	interactive: bool,
}

fn main() {
	let Args {
		log_level,
		show_elapsed,
		max_attempts,
		games,
		interactive,
	} = Args::parse();

	if let Some(log_level) = log_level {
		setup_tracing(log_level, show_elapsed);
	} else if interactive {
		setup_tracing(Level::INFO, show_elapsed);
	}

	match games {
		0 => return info!(":tf:"),
		1 => info!("Playing 1 game!"),
		n => info!("Playing {n} games!"),
	};

	info!("Welcome to Wordle!");

	let (avg_score, total_fails) = match interactive {
		true => play_interative(games),
		false => play(max_attempts, games),
	};

	info!("Done.");
	info!("Stats:");
	info!("  * Average score: {avg_score:.2}");
	info!("  * Failed games: {total_fails}");
}

fn setup_tracing(log_level: Level, show_elapsed: bool) {
	let subscriber = tracing_subscriber::fmt()
		.compact()
		.with_max_level(log_level);

	if show_elapsed {
		subscriber
			.with_timer(Uptime::default())
			.init();
	} else {
		subscriber.without_time().init();
	}
}

fn play(max_attempts: usize, games: usize) -> (f64, usize) {
	let mut wordle = Wordle::new(max_attempts);
	let mut total_attempts = 0;
	let mut total_games = 0;
	let mut total_fails = 0;

	for answer in wordle.iter().take(games) {
		let start = Instant::now();

		if let Some(attempts) = wordle.play(answer) {
			total_attempts += attempts;

			let answer = bytes_to_string!(answer);
			let elapsed = start.elapsed();
			info!("Guessed \"{answer}\" in {attempts} attempts. ({elapsed:?})");
		} else {
			total_fails += 1;

			let answer = bytes_to_string!(answer);
			let elapsed = start.elapsed();
			error!("Did not guess \"{answer}\" in <= {max_attempts} attempts. ({elapsed:?})");
		}

		total_games += 1;
	}

	let avg_score = total_attempts as f64 / total_games as f64;

	(avg_score, total_fails)
}

fn play_interative(games: usize) -> (f64, usize) {
	let mut wordle = Wordle::new(6);
	let mut total_attempts = 0;
	let mut total_games = 0;
	let mut total_fails = 0;

	for _ in 0..games {
		if let Some(attempts) = wordle.play_interactive() {
			total_attempts += attempts;
		} else {
			total_fails += 1;
		}

		total_games += 1;
	}

	let avg_score = total_attempts as f64 / total_games as f64;

	(avg_score, total_fails)
}
