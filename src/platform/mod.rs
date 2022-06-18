/* Platform Support Abstractions - (C) 2022 Roman Hargrave <roman@hargrave.info>
 *
 * Fills in (perceived) gaps in the standard library where
 * platform-specific implementations are required.
 */

use cfg_if::cfg_if;

cfg_if! {
  if #[cfg(unix)] {
    pub mod unix;
    pub use unix::*;
  } else if #[cfg(windows)] {
    pub mod winapi;
    pub use winapi::*;
  }
}
