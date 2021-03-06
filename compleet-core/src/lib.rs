mod autocmds;
mod client;
mod commands;
mod completion_source;
mod config;
mod error;
mod hlgroups;
mod mappings;
mod messages;
mod setup;

pub use client::Client;
use client::State;
pub use completion_source::CompletionSource;
use config::Config;
pub use error::{Error, Result};
