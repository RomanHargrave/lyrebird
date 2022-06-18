/* Platform Support Abstractions (*NIX-like) - (C) 2022 Roman Hargrave <roman@hargrave.info>
 *
 * These were tested on Linux, but will likely work on macOS (I don't
 * have one to test with), as they are all POSIXy and macOS purports
 * to still be compliant.
 */

use libc::{
  passwd,
  getuid,
  getpwuid_r,
  ERANGE
};

use std::ptr;
use std::ffi::CStr;

const PWBUF_MAX_SIZE: usize = 8192;
const PWBUF_GROW_FAC: usize = 2048;

pub const LOG_FILE: &str = "/tmp/lyrebird.log";

/// create a `passwd` (`/etc/passwd` entry) structure typical of the Linux layout
#[cfg(target_os = "linux")]
fn init_passwd() -> passwd {
  passwd {
    pw_name: ptr::null_mut(),
    pw_passwd: ptr::null_mut(),
    pw_uid: 0,
    pw_gid: 0,
    pw_gecos: ptr::null_mut(),
    pw_dir: ptr::null_mut(),
    pw_shell: ptr::null_mut()
  }
}

/// create a `passwd` structure typical of BSD/macOS
#[cfg(target_os = "macos")] // should work for any BSD
fn init_passwd() -> passwd {
  passwd {
    pw_name: ptr::null_mut(),
    pw_passwd: ptr::null_mut(),
    pw_uid: 0,
    pw_gid: 0,
    pw_change: 0,
    pw_class: ptr::null_mut(),
    pw_gecos: ptr::null_mut(),
    pw_dir: ptr::null_mut(),
    pw_shell: ptr::null_mut(),
    pw_expire: 0,
  }
}
// getpwuid_r() accepts a structure, and some pre-allocated memory.
// it returns a status code that is only meaningful if it does not
// set its output parameter (`result`).
//
// more specifically, we will need to provide getpwuid_r() both with
// a pre-allocated structure, as well as pre-allocated memory that
// it may use to store data pointed to by the pointers in that
// structure.
//
// if it sets the pointer pointed to by `result` to NULL and returns
// 0, we assume that there is no corresponding pwent for the user id
// in question.
//
// this is still the most robust option in comparison to getlogin(),
// which relies on utmp having good data, or to checking LOGNAME or
// USER, which could be altered (it seems desirable to base this off
// of process owner UID, honestly).
//
// as an aside, there is not a general way to ask the system what
// the optimal buffer size is. on linux, GETPWNAM(3) suggests
// querying sysconf() for _SC_GETPW_R_SIZE_MAX; however, this
// appears to not be implemented in the BSDs, which means it is
// likely not implemented on macOS. thus, we shall generally assume
// that the buffer size will not exceed 2Ki, a typical value for the
// maximum length of an entry in /etc/passwd. of course, some
// systems may use different name services (see nsswitch.conf) - to
// accomodate this, we will try to grow the buffer a few times
// before giving up. specifically, we will call it quits after
// 8Ki, incrementing by 2Ki each try.
//
/// Get the real username of the process owner. Uses `getpwuid_r()` to relate
/// the process UID to a name. Returns None when the username is not available,
/// or `getpwuid_r()` wants an unreasonably large buffer (see `PWBUF_MAX_SIZE`).
///
/// Panics when `getpwuid_r()` returns an abnormal error code
pub fn get_username() -> Option<String> {
  let mut pwent: passwd = init_passwd();

  let mut pwent_p: *mut passwd = ptr::null_mut();
  let mut pwbuf: Vec<i8> = vec![0; PWBUF_GROW_FAC];

  let result = loop {
    let status = unsafe {
      getpwuid_r(
        getuid(),
        &mut pwent,
        pwbuf.as_mut_ptr(),
        pwbuf.capacity(),
        &mut pwent_p
      )
    };

    // if our pwent result pointer was set to null, we either need to
    // grow the buffer, assume that no pwent was found, or that
    // something more serious has happened
    if pwent_p.is_null() {
      match status {
        // no user was found
        0 => break None,
        // reserve more space in buffer if requested
        ERANGE if pwbuf.capacity() < PWBUF_MAX_SIZE =>
          // resize() is specifically used here instead of reserve to
          // ensure that over-reservation is performed, as the
          // last-ditch exit condition for the loop is the vector
          // reaching PWBUF_MAX_SIZE.
          pwbuf.resize(pwbuf.capacity() + PWBUF_GROW_FAC, 0),
        // the buffer has reached the maximum allowable size and
        // getpwuid_r() still wants more. just give up at this point.
        ERANGE => break None,
        // some other error was returned
        e => panic!("getpwuid_r() returned unexpected value {}", e),
      }
    } else {
      // pwent has been populated.
      if pwent.pw_name.is_null() {
        break None;
      } else {
        break Some(pwent.pw_name);
      }
    }    
  };

  result
    .map(|strp| unsafe { CStr::from_ptr(strp) })
    // if an encoding error happens, just die for now.
    .map(|cstr| cstr.to_str().unwrap())
    .map(String::from)
}
