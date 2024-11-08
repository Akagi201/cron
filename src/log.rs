#![allow(unused)]

pub use tracing::{debug, error, info, warn};
use tracing_subscriber::EnvFilter;
pub fn init_log() {
  let env_filter = EnvFilter::new("info");
  tracing_subscriber::fmt::Subscriber::builder()
    .with_env_filter(env_filter)
    // .with_thread_ids(true)
    // .with_thread_names(true)
    // .with_file(true)
    // .with_line_number(true)
    .with_ansi(true)
    .compact()
    // .json()
    // .flatten_event(true)
    .init();
}
