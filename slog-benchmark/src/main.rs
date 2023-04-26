#[macro_use]
extern crate slog;
extern crate chrono;
use microbench::{self, Options};

/* slog - noop drain  (5.0s) ...              2.452 ns/iter (0.990 R²)
*/

fn main() {
    let options: Options = Options::default();

    let drain = slog::Discard;

    let root_logger = slog::Logger::root(drain, o!());

    microbench::bench(&options, "slog - noop drain ", || info!(root_logger, "Application started"));
}