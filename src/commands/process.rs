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

pub fn start_process(log: &mut Log, params: &StartProcessArgs) -> OrErrorBox {
  Command::new(&params.exe)
    .args(&params.args)
    .spawn()
    .map_err(|e| e.into())
    .map(|child| json!({
      "cmd":  &params.exe,
      "args": &params.args,
      "pid":  child.id()
    }))
    .and_then(|rec| log.record_action("StartProcess", &rec))
}
