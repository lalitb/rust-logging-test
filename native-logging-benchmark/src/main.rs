#[macro_use]
extern crate log;
use std::{error, fmt, result, sync::Arc};
use microbench::{self, Options};
use log::{Level, Metadata, Record};
use std::time::{SystemTime, UNIX_EPOCH};

/* Results -
    No logger attached - log_enabled(level) (5.0s) ...           0.423 ns/iter (0.999 R²)
    No logger attached - log_enabled(target, level) (5.0s) ...           0.420 ns/iter (0.999 R²)
    No logger attached - debug(text (5.0s) ...           0.219 ns/iter (0.997 R²)
    Noop logger attached - log_enabled() (5.0s) ...           1.556 ns/iter (0.998 R²)
    Noop logger attached - log_enabled(target, level) (5.0s) ...           1.321 ns/iter (0.998 R²)
    Noop logger attached - debug(text (5.0s) ...           3.160 ns/iter (0.998 R²)
 */

struct NoopLogger {
    level: Level,
}

impl log::Log for NoopLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }
    fn log(&self, record: &Record) {
    }

    fn flush(&self) {
        
    }
}

impl NoopLogger{
    fn init(level: Level){
        log::set_max_level(level.to_level_filter());
        let logger = NoopLogger{level};
        log::set_boxed_logger(Box::new(logger));
    }
}

fn main () {
    let options: Options = Options::default();

    microbench::bench(&options, "No logger attached - log_enabled(level)", || log_enabled!(Level::Debug));
    microbench::bench(&options, " No logger attached - log_enabled(target, level)", || log_enabled!(target:"Global", Level::Debug));

    microbench::bench(&options, " No logger attached - debug(text", || debug!("expensive debug data: "));

    NoopLogger::init(Level::Debug);
    microbench::bench(&options, "Noop logger attached - log_enabled()", || log_enabled!(Level::Debug));
    microbench::bench(&options, "Noop logger attached - log_enabled(target, level)", || log_enabled!(target:"Global", Level::Debug));

    microbench::bench(&options, "Noop logger attached - debug(text", || debug!("expensive debug data: "));

}











/*
use log::Level::Debug;
use log::{debug, log_enabled};
use microbench::{self, Options};

struct Data{
    x: i32,
    y: i32
}

fn expensive_call() -> Data {
    Data {x:10, y:100}
}

fn main() {
    let options: Options = Options::default();
    microbench::bench(&options, "logs_enabled - Default", || log_enabled!(Debug));
    microbench::bench(&options, "logs_enabled - Component", || log_enabled!(target:"Global", Debug));
    if log_enabled!(Debug) {
        let data = expensive_call();
        debug!("expensive debug data: {} {}", data.x, data.y);
    }
    if log_enabled!(target: "Global", Debug) {
        let data = expensive_call();
    debug!(target: "Global", "expensive debug data: {} {}", data.x, data.y);
    }
}
*/
