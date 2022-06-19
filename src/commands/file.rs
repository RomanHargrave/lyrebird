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

impl FileCommand {
  pub fn dispatch(&self, log: &mut Log) -> OrErrorBox {
    match self {
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
}

#[cfg(test)]
mod test {
  use super::*;
  use std::fs;
  use crate::test_helpers::*;

  #[test]
  fn test_create_file() {
    let mut log = get_test_log();
    let test_file = get_test_filename();

    // don't care if this fails (test file may not exist already)
    let _ = remove_file(&test_file);

    let cmd = FileCommand::Create(FileArgs {
      path: test_file.clone()
    });

    cmd.dispatch(&mut log).expect("create() failed");

    let md = fs::metadata(&test_file).expect("could not get metadata for test file");

    assert!(md.is_file(), "create() did not create regular file at {}", &test_file);

    remove_file(&test_file).expect("could not remove test file");
  }

  #[test]
  fn test_modify_file() {
    use std::time::Duration;

    let mut log = get_test_log();
    let test_file = get_test_filename();

    // ensure file exists
    let _ = remove_file(&test_file);
    create(&mut log, &test_file).expect("create() failed");

    let md_before =
      fs::metadata(&test_file).expect("could not get metadata for test file");

    // wait 1s to ensure measurable difference in mtime, especially on
    // systems with second-resolution file times (if there are still
    // any)
    std::thread::sleep(Duration::from_secs(1));

    let cmd = FileCommand::Modify(FileArgs {
      path: test_file.clone()
    });

    cmd.dispatch(&mut log).expect("could not modify test file");

    let md_after =
      fs::metadata(&test_file).expect("could not get metadata for test file after modify");

    assert!(md_before.modified().unwrap() < md_after.modified().unwrap(), "modified time did not increment");
    assert!(md_before.len() < md_after.len(), "file size did not increase after modification (append)");

    // clean up
    let _ = remove_file(&test_file);
  }

  #[test]
  fn test_remove_file() {
    let mut log = get_test_log();
    let test_file = get_test_filename();

    // ensure file exists
    let _ = remove_file(&test_file);
    create(&mut log, &test_file).expect("create() failed");

    let cmd = FileCommand::Delete(FileArgs {
      path: test_file.clone()
    });

    cmd.dispatch(&mut log).expect("delete() failed");

    // does the file exist?
    assert!(check_file_missing(&test_file),
            "expected file {} to not exist after delete() but it still does", test_file)
  }
}
