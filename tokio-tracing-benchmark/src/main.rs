
use tracing::{info};
use tracing_subscriber::prelude::*; 
use tracing::Id;
use microbench::{self, Options};


mod custom_layer;
use custom_layer::CustomLayer;
use custom_layer::NoopCustomLayer;

/*
Result :
no logging subscriber: info with structure data (5.0s) ...           0.274 ns/iter (0.989 R²)
no logging subscriber: info with unstructured data (5.0s) ...           0.483 ns/iter (0.997 R²)
noop logging subscriber: info with structure data (5.0s) ...          12.172 ns/iter (0.997 R²)
noop logging subscriber: info with unstructured data (5.0s) ...           9.727 ns/iter (0.990 R²)

 */
fn main() {

    let options: Options = Options::default();

    microbench::bench(&options, "no logging subscriber: info with structure data", || info!(a_bool = true, answer = 42, message = "first example"));
    microbench::bench(&options, "no logging subscriber: info with unstructured data", || info!("test123"));

    let layer = CustomLayer{};
    let noop_layer = NoopCustomLayer{};
    tracing_subscriber::registry().with(noop_layer)
      .init();

    microbench::bench(&options, "noop logging subscriber: info with structure data", || info!(a_bool = true, answer = 42, message = "first example"));
    microbench::bench(&options, "noop logging subscriber: info with unstructured data", || info!("test123"));
}
