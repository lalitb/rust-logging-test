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
