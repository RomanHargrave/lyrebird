/* Platform Support Abstractions (Windows) - (C) 2022 Roman Hargrave <roman@hargrave.info>
 */

use winapi::{
  DWORD,
  GetUserNameA
}

use std::ptr;

pub const LOG_FILE = "%TMP%/lyrebird.log";

pub fn get_username() -> Option<String> {
  /* In this case, WINAPI is just slightly more tolerable than POSIX,
   * since we can actually poke it for the expected size of the
   * username and allocate just that.
   */
  let mut name_length: DWORD = 0;
  let mut name_buf

  // first, we need to figure out how many bytes we actually need.
  unsafe {
    GetUserNameA(ptr::null_mut(), &mut name_length);
  }
}
