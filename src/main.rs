/* Lyrebird - (C) 2022 Roman Hargrave <roman@hargrave.info>
 *
 * Simulates indicative behavior in EDR test environments.
 */

extern crate core;

use std::env;
use std::fs::File;
use std::error::Error;

use clap::Parser;

mod platform;

mod log;
use crate::log::Log;

mod commands;
use crate::commands::LyrebirdCli;

#[cfg(test)]
mod test_helpers;

pub type OrErrorBox = Result<(), Box<dyn Error>>;

fn guess_log_file() -> String {
  env::vars()
    .find(|(k, _)| k == "LYREBIRD_LOG")
    .map(|(_, v)| v)
    .unwrap_or_else(platform::get_log_file_path)
}

fn main() -> Result<(), Box<dyn Error>> {
  let mut log = File::options()
    .create(true)
    .write(true)
    .append(true)
    .open(guess_log_file())
    .map(|f| Log::new(Box::new(f)))?;

  LyrebirdCli::parse().dispatch(&mut log)
}
