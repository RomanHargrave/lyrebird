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
    let mut child = Command::new(&self.exe)
      .args(&self.args)
      .spawn()?;

    // Place info about what we did in the log after starting the
    // process and before waiting for the child to exit, to get more
    // accurate timing
    let log_res = log.record_action("Exec", json!({
        "cmd":  &self.exe,
        "args": &self.args,
        "pid":  child.id()
    }));

    // Wait for child to exit, discard result or early return Err().
    // It's not clear under what conditions an error would be
    // returned, possibly some kind of race, but I have yet to produce
    // such conditions.
    let _ = child.wait()?;

    // Once the child has exited, return whatever status the logger
    // gave us, in case it encountered an IO or formatting error.
    log_res
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::test_helpers::*;
  use std::fs;

  // The idea here is to prove that StartProcess ran a command by
  // expecting that command to create a file. This is possibly on
  // windows, but much more circuitous. For windows, we would need a
  // program on the path that can create a file and is not complex.
  // The closest thing I can find in system32 is `tar`, which means we
  // would need an input file as well.
  #[cfg(unix)]
  #[test]
  fn test_create_process_unix() {
    let mut log = get_test_log();
    let expect_file = get_test_filename();

    let command = StartProcess {
      exe: String::from("touch"),
      args: vec![expect_file.clone()]
    };

    let _ = fs::remove_file(&expect_file);
    assert!(check_file_missing(&expect_file),
            "unable to test start_process by creating file: file {} already exists",
            expect_file);

    command.start_process(&mut log)
           .expect("start_process() failed");

    // Test existence by trying to stat() (there's not a good
    // standalone existence test in std::fs yet)
    fs::metadata(&expect_file)
      .expect(&format!("could not get file metadata for test file {}", expect_file));

    // remove the test file
    let _ = fs::remove_file(&expect_file);
  }
}
