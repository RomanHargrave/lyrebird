/* Lyrebird Test Helpers - (C) 2022 Roman Hargrave <roman@hargrave.info>
 */

use crate::log::Log;

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
