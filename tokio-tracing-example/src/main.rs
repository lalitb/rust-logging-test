use tracing::{event, info, Level};
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

mod custom_layer;
use custom_layer::CustomLayer;
struct User {
    name: String,
    email: String,
}

fn main() {
    let filter = EnvFilter::from_default_env()
        .add_directive("target1=error".parse().unwrap())
        .add_directive(Level::INFO.into());

    let layer = CustomLayer {}.with_filter(filter);
    tracing_subscriber::registry().with(layer).init();

    info!("test123"); //default target as module path/name

    info!(target: "target1", "something has happened");

    event!(target: "target1", Level::ERROR, "something has happened!");

    // structured field using struct
    let user = User {
        name: "ferris".into(),
        email: "ferris@rust-lang.org".into(),
    };
    info!(target: "target2", event_id = 2, event_name = "event_name", user.name, user.email);

    //stuct field using key-value part of arguments
    info!(target: "target2", event_id = 2, event_name = "event_name", user.name, user.email);

    //message template using escape
    info!(
        event_id = 2,
        event_name = "event_name",
        username = "test",
        email = "test123",
        "Successful login for {{username}} and {{email}}"
    );
}
