use clap::{Parser, ValueEnum};
use std::time::Instant;
use tracing::{error, info, Level};
use tracing_subscriber::fmt::time::Uptime;
use wordle_solver::{Guesser, DICTIONARY};

mod guessers;

static GAMES: &str = include_str!("../data/words/wordle-answers.txt");

#[derive(Debug, Clone, Copy, Parser)]
struct Args {
	/// The amount of games to play
	#[arg(long = "games")]
	max_games: Option<usize>,

	/// The maximum amount of guesses until the guesser fails
	#[arg(long = "attempts", long = "tries")]
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

	match max_games.unwrap_or(1).max(1) {
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
	let max_games = max_games.unwrap_or(1).max(1);

	for answer in GAMES.lines().take(max_games) {
		let start = Instant::now();

		if let Some(n_attempts) =
			dictionary.play(G::default(), answer.as_bytes().try_into().unwrap(), max_attempts)
		{
			total_attempts += n_attempts;
			games_played += 1;

			let took = start.elapsed();
			info!("Guessed \"{answer}\" in {n_attempts} attempts. ({took:?})");
		} else {
			games_played += 1;
			games_failed += 1;

			let took = start.elapsed();
			error!("Did not guess \"{answer}\" in <={max_attempts} attempts. ({took:?})");
		}
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
