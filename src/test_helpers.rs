/* Lyrebird Test Helpers - (C) 2022 Roman Hargrave <roman@hargrave.info>
 */

use crate::log::Log;

use std::fs;
use std::io;
use random_string;

/// Get a logger that writes to nowhere
#[inline]
pub fn get_test_log() -> Log {
  Log::new(Box::new(std::io::sink()))
}

/// Get a filename somewhere in /tmp with a random name
pub fn get_test_filename() -> String {
  let mut file_name = String::from("lb_test_");
  file_name.push_str(&random_string::generate(16, "abcdefghijklmnopqrstuvwxyz123456789"));

  file_name
}

/// Return true if a file does not exist. Tests file existence by
/// trying to get metadata and expecting io::ErrorKind::NotFound. Any
/// other error or Ok(_) will yield a negative result.
pub fn check_file_missing(file: &str) -> bool {
  match fs::metadata(file) {
    Err(e) => e.kind() == io::ErrorKind::NotFound,
    _ => false
  }
}
