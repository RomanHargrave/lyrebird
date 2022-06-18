/* Lyrebird - (C) 2022 Roman Hargrave <roman@hargrave.info>
 *
 * Simulates indicative behavior in EDR test environments.
 */

extern crate core;

use std::env;
use std::io::stdout;
use std::error::Error;

use clap::Parser;

mod platform;

mod log;
use crate::log::Log;

mod commands;
use crate::commands::LyrebirdCli;

pub type OrErrorBox = Result<(), Box<dyn Error>>;

fn guess_log_file() -> String {
  env::vars()
    .find(|(k, _)| k == "LYREBIRD_LOG")
    .map(|(_, v)| v)
    .unwrap_or_else(|| String::from(platform::LOG_FILE))
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut log = Log::new(Box::new(stdout()));

  LyrebirdCli::parse().dispatch(&mut log)
}
