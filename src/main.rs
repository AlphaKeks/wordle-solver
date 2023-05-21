use clap::{Parser, ValueEnum};
use std::time::Instant;
use tracing::{error, info, Level};
use wordle_solver::{algorithms, Guesser, Wordle};

mod logger;

const GAMES: &str = include_str!("../data/words/answers.txt");

fn main() {
	let args = Args::parse();

	logger::init(args);

	let (started, avg_score) = match args.implementation {
		Implementation::Naive => play::<algorithms::NaiveGuesser>(args),
		Implementation::LessAllocs => play::<algorithms::LessAllocsGuesser>(args),
		Implementation::VecDict => play::<algorithms::VecDictGuesser>(args),
		Implementation::OnceCell => play::<algorithms::OnceInitGuesser>(args),
		Implementation::Precalc => play::<algorithms::PrecalcGuesser>(args),
		Implementation::Weight => play::<algorithms::WeightGuesser>(args),
		Implementation::Prune => play::<algorithms::PruneGuesser>(args),
	};

	let took = started.elapsed();

	match args.max_games {
		1 if args.parsable => info!("{avg_score:.2} avg done after {took:.2?}."),
		1 => info!("Played 1 game in {took:.2?}. Average score: {avg_score:.2}"),
		_ if args.parsable => info!("{avg_score:.2} avg done after {took:.2?}."),
		n => info!("Played {n} games in {took:.2?}. Average score: {avg_score:.2}"),
	};
}

fn play<G: Guesser + Default>(
	Args { max_games, max_attempts, parsable, .. }: Args,
) -> (Instant, f64) {
	let wordle = Wordle::new();

	if !parsable {
		match max_games {
			1 => info!("Playing a game!"),
			n => info!("Playing {n} games!"),
		};
	}

	let start = Instant::now();
	let mut score = 0;
	let mut games = 0;

	for answer in GAMES.lines().take(max_games) {
		let start = Instant::now();
		if let Some(n_attempts) = wordle.play(answer, G::default(), max_attempts) {
			score += n_attempts;
			games += 1;
			let took = start.elapsed();
			match parsable {
				true => info!("{answer} in {n_attempts} ({took:.2?})"),
				false => info!("Guessed \"{answer}\" in {n_attempts} attempts. (took {took:.2?})"),
			};
		} else {
			let took = start.elapsed();
			match parsable {
				true => error!("no answer in {max_attempts} ({took:.2?})"),
				false => error!(
					"Did not guess \"{answer}\" in <={max_attempts} attempts. (took {took:.2?})"
				),
			};
		}
	}

	let score = score as f64 / games as f64;

	(start, score)
}

#[derive(Clone, Copy, Parser)]
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

	/// Print the elapsed time since the beginning of the program
	#[arg(short = 'e', long = "elapsed")]
	#[clap(default_value = "true")]
	total_elapsed: bool,

	/// Print a format that is much easier to parse
	#[arg(short, long)]
	#[clap(default_value = "false")]
	parsable: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Implementation {
	Naive,
	LessAllocs,
	VecDict,
	OnceCell,
	Precalc,
	Weight,
	Prune,
}
