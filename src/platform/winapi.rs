/* Platform Support Abstractions (Windows) - (C) 2022 Roman Hargrave <roman@hargrave.info>
 */

use std::ptr;
use std::string::String;

use windows::core::PWSTR;
use windows::Win32::System::WindowsProgramming::GetUserNameW;

pub const LOG_FILE: &str = "lyrebird.log";

pub fn get_username() -> Option<String> {
  /* In this case, WINAPI is just slightly more tolerable than POSIX,
   * since we can actually poke it for the expected size of the
   * username and allocate just that.
   */
  let mut name_length: u32 = 0;
  let mut name_buf: Vec<u16> = vec![];

  // first, we need to figure out how many bytes we actually need.
  // according to windows docs, this should populate name length with 
  // some value > 0 and return FALSE
  unsafe { GetUserNameW(PWSTR(ptr::null_mut()), &mut name_length) };

  assert!(name_length > 0, "GetUserNameW set pcbBuffer <= 0");

  // now, we should have a username length (in 16-bit codepoints)
  name_buf.resize(name_length.try_into().unwrap(), 0);

  // GetUserNameW should now return TRUE
  unsafe { GetUserNameW(PWSTR(name_buf.as_mut_ptr()), &mut name_length) }
    .expect("GetUserNameW failed");

  // Remove trailing null character
  let mut name = String::from_utf16_lossy(&name_buf);
  name.truncate(name.len() - 1);

  Some(name)
}
