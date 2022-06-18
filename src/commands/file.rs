/* File Commands - (C) 2022 Roman Hargrave <roman@hargrave.info>
 */

use clap::{Args, Subcommand};
use serde_json::json;
use std::fs::{remove_file, canonicalize, File};

use std::io::Write;

use crate::OrErrorBox;
use crate::log::Log;

const LB_FILE_CONTENT: &str = "Hello from Lyrebird!\n";

/// File action logging helper. Places {"type": "file"} entries in the
/// log with appropriate data
fn record_file_action(log: &mut Log, action: &str, path: &str) -> OrErrorBox {
  log.record_action("File", json!({
    "action": action,
    "file": path
  }))
}

/// Create a new file with some data at `path` and record doing so to
/// `log`.
fn create(log: &mut Log, path: &str) -> OrErrorBox {
  let f = File::options()
    .create_new(true)
    .write(true)
    .open(path);

  f?.write_all(LB_FILE_CONTENT.as_bytes())
    .and_then(|()| canonicalize(path).map(|pb| pb.to_string_lossy().to_string()))
    .map_err(|e| e.into())
    .and_then(|path| record_file_action(log, "create", &path))
}

/// Place some data and a newline at the end of the file at `path` and
/// record having done so in the `log`.
fn modify(log: &mut Log, path: &str) -> OrErrorBox {
  let f = File::options()
    .append(true)
    .create(false)
    .open(path);

  f?.write_all(LB_FILE_CONTENT.as_bytes())
    .map_err(|e| e.into())
    .and_then(|()| record_file_action(log, "modify", path))
}

/// Delete the file at `path` and record doing so to `log`.
fn delete(log: &mut Log, path: &str) -> OrErrorBox {
  remove_file(path)
    .map_err(|e| e.into())
    .and_then(|()| record_file_action(log, "delete", path))
}

// Command Line Interfaces

#[derive(Args)]
pub struct FileArgs {
  /// Path to the file
  path: String,
}

impl FileArgs {
  pub fn abspath(&self) -> std::io::Result<String> {
    canonicalize(&self.path)
      .map(|pb| pb.to_string_lossy().to_string())
  }
}

#[derive(Subcommand)]
pub enum FileCommand {
  /// Create a file
  Create(FileArgs),
  /// Delete a file
  Delete(FileArgs),
  /// Modify a file
  Modify(FileArgs),
}

pub fn handle_command(log: &mut Log, cmd: &FileCommand) -> OrErrorBox {
  match cmd {
    FileCommand::Create(args) => create(log, &args.path),
    FileCommand::Modify(args) =>
      args.abspath()
          .map_err(|e| e.into())
          .and_then(|path| modify(log, &path)),
    FileCommand::Delete(args) =>
      args.abspath()
          .map_err(|e| e.into())
          .and_then(|path| delete(log, &path)),
  }
}
