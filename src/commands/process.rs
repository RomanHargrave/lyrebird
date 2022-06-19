/* Process Command - (C) 2022 Roman Hargrave <roman@hargrave.info>
 */

use clap::Args;
use std::process::Command;
use serde_json::json;

use crate::OrErrorBox;
use crate::log::Log;

#[derive(Args)]
pub struct StartProcess {
  /// Name of the program to start
  exe: String,
  /// Arguments to be passed
  args: Vec<String>
}

impl StartProcess {
  /// Start a process based on command line parameters and record having
  /// done so in the log.
  pub fn start_process(&self, log: &mut Log) -> OrErrorBox {
    // Create a new command builder with the given executable name and
    // arguments.
    Command::new(&self.exe)
      .args(&self.args)
    // start the command
      .spawn()
      .map_err(|e| e.into())
    // If Command succeeded, extract what we need for the log from the
    // Child structure
      .map(|child| json!({
        "cmd":  &self.exe,
        "args": &self.args,
        "pid":  child.id()
      }))
    // And then place the extracted data in the log
      .and_then(|rec| log.record_action("Exec", &rec))
  }
}
