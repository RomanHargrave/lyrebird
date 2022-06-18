/* Lyrebird Commands - (C) 2022 Roman Hargrave <roman@hargrave.info>
 */

use clap::{Parser, Subcommand};

use crate::OrErrorBox;
use crate::log::Log;

pub mod file;

#[derive(Subcommand)]
pub enum Commands {
  /// File operations
  #[clap(subcommand)]
  File(file::FileCommand)
}

#[derive(Parser)]
pub struct LyrebirdCli {
  #[clap(subcommand)]
  command: Commands,
}

impl LyrebirdCli {
  pub fn dispatch(&self, log: &mut Log) -> OrErrorBox {
    match &self.command {
      Commands::File(file_cmd) => file::handle_command(log, file_cmd)
    }
  }
}
