//! Substrate Node Template CLI library.
#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod silly_rpc;
mod service;
mod benchmarking;
mod cli;
mod command;
mod rpc;


fn main() -> sc_cli::Result<()> {
	command::run()
}
