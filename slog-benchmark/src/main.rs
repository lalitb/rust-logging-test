#[macro_use]
extern crate slog;
extern crate chrono;
extern crate slog_term;
extern crate slog_async;

use microbench::{self, Options};

use crate::slog::Drain;
use std::fs::File;



/* 
slog - is_enabled(level) (5.0s) ...           1.218 ns/iter (0.998 R²)
slog - noop drain  (5.0s) ...              2.412 ns/iter (0.996 R²)
*/

fn main() {
    let options: Options = Options::default();

    let drain = slog::Discard;

    // update filter level at bootstrap
    drain.filter_level(slog::Level::Debug).fuse();

    let root_logger: slog::Logger = slog::Logger::root(drain, o!());

   // assert!(root_logger.is_debug_enabled() == true);
   println!(" Logger level: {}", root_logger.is_enabled(slog::Level::Error));

    //runtime filter level
    drain.filter_level(slog::Level::Error).fuse();
    microbench::bench(&options, "slog noop - is_enabled(level)", || root_logger.is_enabled(slog::Level::Debug));
    microbench::bench(&options, "slog noop info ", || info!(root_logger, "Application started"));

    // use /dev/null with filtering
    let mut f = File::open("/dev/null").expect("error opening");

    let decorator = slog_term::PlainSyncDecorator::new(f);
    let drain = slog_term::FullFormat::new(decorator).build().filter_level(slog::Level::Info).fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let root_logger: slog::Logger = slog::Logger::root(drain, o!());
    assert!(root_logger.is_enabled(slog::Level::Debug) == true);
    microbench::bench(&options, "slog /dev/null ", || root_logger.is_enabled(slog::Level::Debug));
    microbench::bench(&options, "slog /dev/null - is_enabled(level)", || root_logger.is_enabled(slog::Level::Debug));
    microbench::bench(&options, "slog /dev/null - is_enabled(level)", || root_logger.is_enabled(slog::Level::Debug));

}