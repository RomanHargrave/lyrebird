/* Platform Support Abstractions (Windows) - (C) 2022 Roman Hargrave <roman@hargrave.info>
 */

use std::ptr;
use std::string::String;

use windows::core::{PWSTR, PCWSTR};
use windows::Win32::Foundation::{
  GetLastError,
  WIN32_ERROR
};
use windows::Win32::System::{
  WindowsProgramming::GetUserNameW,
  Environment::ExpandEnvironmentStringsW
};

// TODO: for %TMP% support on windows, it will be necessary
// to refactor this into get_log_path() -> Option<String> and
// use (unsafe) ExpandEnvironmentStringsW to compute the path.
const LOG_FILE: &str = "%TMP%\\lyrebird.log";

fn last_error() -> WIN32_ERROR {
  unsafe { GetLastError() }
}

fn expand_env(template: &str) -> String {
  // we will need to convert our template to a sequence of UTF-16
  // chars, which will in our case shall be a Vec<u16>. since rust
  // uses smart strings, we're also going to need to append a null
  // terminator.
  let mut template_wcstr: Vec<u16> =
    template.encode_utf16().collect();

  template_wcstr.push(0);

  let mut dst_wcstr: Vec<u16> = vec![];

  // Probe for the required output vector size. Amusingly, windows-rs
  // implements a wrapper here that will accept a vec and pass its
  // length to the underlying function for us, unlike GetUserNameW...
  let need_len =
    unsafe { ExpandEnvironmentStringsW(PCWSTR(template_wcstr.as_ptr()), &mut dst_wcstr) };

  // if need_len = 0, some error occurred
  assert!(need_len > 0, "ExpandEnvironmentStringsW() returned 0, last_error = {:?}", last_error());

  dst_wcstr.resize(need_len.try_into().unwrap(), 0);

  // Now, try to expand env vars in the string
  let wrote_len =
    unsafe { ExpandEnvironmentStringsW(PCWSTR(template_wcstr.as_ptr()), &mut dst_wcstr) };

  // wrote_len should equal need_len if this succeeded
  assert_eq!(wrote_len, need_len,
             "ExpandEnvironmentStringsW() requested capacity {} but only wrote {} chars, last error = {:?}",
             need_len, wrote_len, last_error());

  // Discard the trailing null character
  let _ = dst_wcstr.pop();

  // Given what this fn is used for, it is OK to panic if we have
  // chars we can't represent.
  String::from_utf16(&dst_wcstr).unwrap()
}

/// Get the path to the log file
pub fn get_log_file_path() -> String {
  expand_env(LOG_FILE)
}

/// Get the name of the executing user
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

  assert!(name_length > 0, "GetUserNameW set pcbBuffer = 0");

  // now, we should have a username length (in 16-bit chars)
  name_buf.resize(name_length.try_into().unwrap(), 0);

  // GetUserNameW should now return TRUE
  unsafe { GetUserNameW(PWSTR(name_buf.as_mut_ptr()), &mut name_length) }
    .expect("GetUserNameW failed");

  // Discard trailing null char
  let _ = name_buf.pop();

  Some(String::from_utf16_lossy(&name_buf))
}
