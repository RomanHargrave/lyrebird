/* Lyrebird Activity Log - (C) 2022 Roman Hargrave <roman@hargrave.info>
 *
 * Collects events and inserts them into an output stream as JSON
 * objects. Each JSON object includes the PID, time, user, and command
 * line.
 */

use std::{process, env};
use std::io::Write;

use serde_json::json;

use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::platform::get_username;

pub struct Log<W: Write> {
  writer: W,
  username: Option<String>,
  cmdline: Vec<String>
}

impl<W: Write> Log<W> {
  pub fn new(writer: W) -> Log<W> {
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
  
  pub fn record_action(&mut self, record_type: &str, data: Option<serde_json::Value>) {
    let ts = OffsetDateTime::now_utc().format(&Rfc3339).unwrap();

    let record = json!({
      "type": record_type,
      "time": ts,
      "pid": process::id(),
      "user": &self.username,
      "cmd": &self.cmdline,
      "data": data,
    });

    serde_json::to_writer(&mut self.writer, &record).unwrap();
    self.writer.write_all("\n".as_bytes()).unwrap();
  }
}


