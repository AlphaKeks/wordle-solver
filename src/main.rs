use clap::{Parser, ValueEnum};
use std::time::Instant;
use tracing::{error, info, Level};
use tracing_subscriber::fmt::time::Uptime;
use wordle_solver::{Guesser, ANSWERS, DICTIONARY};

mod guessers;

#[derive(Debug, Clone, Copy, Parser)]
struct Args {
	/// The amount of games to play
	#[arg(long = "games")]
	#[clap(default_value = "10")]
	max_games: usize,

	/// The maximum amount of guesses until the guesser fails
	#[arg(long = "attempts", alias = "tries")]
	#[clap(default_value = "6")]
	max_attempts: usize,

	/// `RUST_LOG` level
	#[arg(long = "logs")]
	#[clap(default_value = "info")]
	log_level: Level,

	/// Show the total elapsed time in the logs
	#[arg(long = "elapsed")]
	#[clap(default_value = "false")]
	show_total_elapsed: bool,

	/// The guesser to use
	#[arg(long = "guesser")]
	#[clap(default_value = "schnose")]
	guesser_impl: GuesserImpl,
}

fn main() {
	let args = Args::parse();
	setup_tracing(args);
	let Args { max_games, guesser_impl, .. } = args;

	info!("--- Welcome to Wordle! ---");

	match max_games {
		1 => info!("  Playing 1 game!"),
		n => info!("  Playing {n} games!"),
	};

	info!("   Guesser: {guesser_impl:?}\n");

	let (avg_score, failed_games) = match guesser_impl {
		GuesserImpl::Schnose => play::<guessers::Schnose>(args),
	};

	info!("  Done playing!");
	info!("  Stats:");
	info!("    ∙ Average score: {avg_score:.2}");
	info!("    ∙ Failed games: {failed_games}");
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum GuesserImpl {
	Schnose,
}

fn play<G: Guesser + Default>(Args { max_games, max_attempts, .. }: Args) -> (f64, usize) {
	let dictionary = &DICTIONARY;
	let mut total_attempts = 0;
	let mut games_played = 0;
	let mut games_failed = 0;

	for (answer_str, answer_bytes) in ANSWERS
		.lines()
		.take(max_games)
		.filter_map(|answer| Some((answer, answer.as_bytes().try_into().ok()?)))
	{
		let start = Instant::now();

		if let Some(n_attempts) = dictionary.play(&mut G::default(), answer_bytes, max_attempts) {
			total_attempts += n_attempts;

			let took = start.elapsed();
			info!("Guessed \"{answer_str}\" in {n_attempts} attempts. ({took:?})");
		} else {
			games_failed += 1;

			let took = start.elapsed();
			error!("Did not guess \"{answer_str}\" in <={max_attempts} attempts. ({took:?})");
		}

		games_played += 1;
	}

	let avg_score = total_attempts as f64 / games_played as f64;

	(avg_score, games_failed)
}

fn setup_tracing(Args { log_level, show_total_elapsed, .. }: Args) {
	let subscriber = tracing_subscriber::fmt()
		.compact()
		.with_file(false)
		.with_line_number(false)
		.with_target(false)
		.with_max_level(log_level);

	if show_total_elapsed {
		subscriber
			.with_timer(Uptime::default())
			.init();
	} else {
		subscriber.without_time().init();
	}
}
