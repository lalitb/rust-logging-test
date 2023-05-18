#[macro_use]
extern crate slog;

use rand::Rng;
use std::thread;
use slog::Drain;
use slog::Logger;
use slog_async::Async;
use std::time::Duration;

fn slow_fib(n: u64) -> u64 {
    match n {
        0 | 1 | 2 => 1,
        n => slow_fib(n - 1) + slow_fib(n - 2),
    }
}

fn main() {
    let drain = slog_json::Json::new(std::io::stdout()).add_default_keys()
                                                       .build()
                                                       .filter_level(slog::Level::Debug)
                                                       .fuse();
    let async_drain = Async::new(drain).build().fuse();
    let test_info = format!("v{}", env!("CARGO_PKG_VERSION"));
    let root_log_context= o!("Super Cool Demo" => test_info);
    let root_logger = Logger::root(async_drain, root_log_context);

    info!(root_logger, "Starting...!");

    warn!(root_logger, " User {} logged with {}", "kriss", "kriss@test.com");

    let child_logger = root_logger.new(o!("child" => 1));

    debug!(child_logger, "debug"; "lazy-closure" => slog::FnValue(|_ : &slog::Record| slow_fib(40)));


}
