/* Process Command - (C) 2022 Roman Hargrave <roman@hargrave.info>
 */

use clap::Args;
use std::process::Command;
use serde_json::json;

use crate::OrErrorBox;
use crate::log::Log;

#[derive(Args)]
pub struct StartProcessArgs {
  /// Name of the program to start
  exe: String,
  /// Arguments to be passed
  args: Vec<String>
}

/// Start a process based on command line parameters and record having
/// done so in the log.
pub fn start_process(log: &mut Log, params: &StartProcessArgs) -> OrErrorBox {
  // Create a new command builder with the given executable name and
  // arguments.
  Command::new(&params.exe)
    .args(&params.args)
    // start the command
    .spawn()
    .map_err(|e| e.into())
    // If Command succeeded, extract what we need for the log from the
    // Child structure
    .map(|child| json!({
      "cmd":  &params.exe,
      "args": &params.args,
      "pid":  child.id()
    }))
    // And then place the extracted data in the log
    .and_then(|rec| log.record_action("StartProcess", &rec))
}
