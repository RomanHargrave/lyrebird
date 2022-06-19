/* Network send command - (C) 2022 Roman Hargrave <roman@hargrave.info>
 */

use std::io::Write;
use clap::{Args, Subcommand};
use serde_json::json;
use std::time::Duration;
use std::net::{
  TcpStream,
  SocketAddr
};

use crate::OrErrorBox;
use crate::log::Log;

const DEFAULT_MESSAGE: &str = "Ping from Lyrebird!";

fn record_net_action(log: &mut Log, proto: &str, bytes: usize, laddr: &SocketAddr, peer: &SocketAddr) -> OrErrorBox {
  log.record_action("NetSend", json!({
    "proto":    proto,
    "src_addr": laddr.ip().to_string(),
    "src_port": laddr.port(),
    "dst_addr": peer.ip().to_string(),
    "dst_port": peer.port(),
    "bytes":    bytes
  }))
}

#[derive(Args)]
pub struct TcpMessage {
  /// Destination address
  dest: String,

  /// What to send to the destination
  #[clap(default_value_t = String::from(DEFAULT_MESSAGE))]
  data: String,

  /// Connect timeout in seconds
  #[clap(short = 't', default_value_t = 10)]
  timeout: u64,
}

impl TcpMessage {
  pub fn send(&self, log: &mut Log) -> OrErrorBox {
    let mut sock =
      TcpStream::connect_timeout(&self.dest.parse()?,
                                 Duration::from_secs(self.timeout))?;

    let bytes_written = sock.write(self.data.as_bytes())?;

    record_net_action(log, "TCP", bytes_written, &sock.local_addr()?, &sock.peer_addr()?)
  }
}

#[derive(Subcommand)]
pub enum NetCommands {
  /// Send data to an address over TCP
  Tcp(TcpMessage)
}

impl NetCommands {
  pub fn dispatch(&self, log: &mut Log) -> OrErrorBox {
    match self {
      NetCommands::Tcp(message) => message.send(log)
    }
  }
}
