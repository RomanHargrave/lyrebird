/* Lyrebird Commands - (C) 2022 Roman Hargrave <roman@hargrave.info>
 */

use clap::{Parser, Subcommand};

use crate::OrErrorBox;
use crate::log::Log;

pub mod file;
pub mod process;
pub mod network;
  
#[derive(Parser)]
pub struct LyrebirdCli {
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
  /// File operations
  #[clap(subcommand)]
  File(file::FileCommand),
  
  /// Start a process
  Exec(process::StartProcess),
  
  /// Send data over the network
  #[clap(subcommand)]
  NetSend(network::NetCommands),
}

impl LyrebirdCli {
  pub fn dispatch(&self, log: &mut Log) -> OrErrorBox {
    match &self.command {
      Commands::File(file_cmd) => file_cmd.dispatch(log),
      Commands::Exec(exec)     => exec.start_process(log),
      Commands::NetSend(net)   => net.dispatch(log)
    }
  }
}
