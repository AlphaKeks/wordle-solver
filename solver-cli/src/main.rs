use clap::{Parser, ValueEnum};
use std::time::Instant;
use tracing::{error, info, Level};
use tracing_subscriber::fmt::time::Uptime;
use wordle::{Guesser, Wordle};

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

	/// The algorithm making guesses
	#[arg(long)]
	#[clap(default_value = "naive")]
	algorithm: Algorithm,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Algorithm {
}

fn main() {
	let Args {
		log_level,
		show_elapsed,
		max_attempts,
		algorithm,
	} = Args::parse();

	if let Some(log_level) = log_level {
		setup_tracing(log_level, show_elapsed);
	}

	match max_attempts {
		0 => return info!(":tf:"),
		1 => info!("Playing 1 game!"),
		n => info!("Playing {n} games!"),
	};

	info!("Welcome to Wordle!");
	info!("Algorithm: {algorithm:?}");

	let (avg_score, total_fails) = match algorithm {
	};

	info!("Done.");
	info!("Stats:");
	info!("  * Average score: {avg_score}");
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

fn play<G: Guesser>(max_attempts: usize) -> (f64, usize) {
	let wordle = <G::Wordle as Wordle<G>>::new();
	let mut total_attempts = 0;
	let mut total_games = 0;
	let mut total_fails = 0;

	while let Some(answer) = wordle.next_answer() {
		let start = Instant::now();

		if let Some(attempts) = wordle.play(G::new(), max_attempts) {
			total_attempts += attempts;

			let elapsed = start.elapsed();
			info!("Guessed \"{answer}\" in {attempts} attempts. ({elapsed:?})");
		} else {
			total_fails += 1;

			let elapsed = start.elapsed();
			error!("Did not guess \"{answer}\" in <= {max_attempts} attempts. ({elapsed:?})");
		}

		total_games += 1;
	}

	let avg_score = total_attempts as f64 / total_games as f64;

	(avg_score, total_fails)
}

