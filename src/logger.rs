use crate::Args;
use tracing::Level;

pub(crate) fn init(Args { log_level, total_elapsed, parsable, .. }: Args) {
	if parsable {
		let subscriber = tracing_subscriber::fmt()
			.compact()
			.with_max_level(Level::INFO)
			.with_level(false)
			.with_target(false)
			.with_line_number(false)
			.with_file(false);

		if total_elapsed {
			subscriber
				.with_timer(tracing_subscriber::fmt::time::uptime())
				.init();
		} else {
			subscriber.without_time().init();
		}
	} else {
		let subscriber = tracing_subscriber::fmt()
			.compact()
			.with_max_level(log_level);

		if total_elapsed {
			subscriber
				.with_timer(tracing_subscriber::fmt::time::uptime())
				.init();
		} else {
			subscriber.without_time().init();
		}
	}
}
