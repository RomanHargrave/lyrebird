/* Lyrebird Activity Log - (C) 2022 Roman Hargrave <roman@hargrave.info>
 *
 * Collects events and inserts them into an output stream as JSON
 * objects. Each JSON object includes the PID, time, user, and command
 * line.
 */

use std::{process, env};
use std::io::Write;
use std::error::Error;
use std::boxed::Box;

use serde_json::json;
use serde::Serialize;

use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::platform::get_username;

pub struct Log {
  writer: Box<dyn Write>,
  username: Option<String>,
  cmdline: Vec<String>
}

impl Log {
  /// Create a new Logger, `writer` implements `Write` and will be the destination for log records.
  pub fn new(writer: Box<dyn Write>) -> Log {
    Log {
      writer,
      
      // we're going to want to hang on to the process's username since it won't change
      // and it would be nice to avoid excessive calls to things such as getpwuid_r() were
      // the logger to be called more than once in the same process
      username: get_username(),

      // similarly, go ahead and collect process args only once as well
      cmdline: env::args().collect(),
    }
  }

  /// Place an entry in the log file, where "type" is `record_type`
  /// and "data" is `data`. `data` must implement `serde::Serialize`.
  pub fn record_action<D: Serialize>(&mut self, record_type: &str, data: D) -> Result<(), Box<dyn Error>> {
    // Try to format the current time in UTC as Rcf3339 (strict
    // compatible with ISO8601), place it into an object having the
    // general entry structure, try to write that to the log output,
    // and then try to write a newline.
    OffsetDateTime::now_utc()
      .format(&Rfc3339)
      .map_err(|e| e.into())
      // Create the record object
      .map(|ts| json!({
        "type": record_type,
        "time": ts,
        "pid":  process::id(),
        "user": &self.username,
        "cmd":  &self.cmdline,
        "data": data,
      }))
      // Write the record
      .and_then(|entry|
        serde_json::to_writer(&mut self.writer, &entry)
          .map_err(|e| e.into())
      )
      // Write the newline
      .and_then(|()|
        self.writer.write_all("\n".as_bytes())
          .map_err(|e| e.into())
      )
  }
}


