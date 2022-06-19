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

#[cfg(test)]
mod test {
  use super::*;

  use crate::test_helpers::get_test_log;

  use std::net::TcpListener;
  use std::sync::mpsc;
  use std::sync::mpsc::{Sender, Receiver};
  use std::thread;
  use std::io::Read;
  use std::time::Duration;

  // Use something in the loopback range less likely to have bindings
  // (though we can't avoid applications with bindings to all
  // interfaces - as such, this may be strictly reliable only in
  // Docker)
  const TCP_LISTEN_SOCK: &str = "127.0.0.5:9876";

  const MSG_THREAD_READY: &str = "ready";

  /// Number of seconds to wait for response from listen thread
  const RESPONSE_WAIT: u64 = 5;

  /// Test net-send tcp by effectively calling the command and asking
  /// to send data to a TCP socket in another thread, and then check
  /// the data it was asked to send against the data received by the
  /// listener.
  #[test]
  fn test_net_send_tcp() {
    let (to_test, from_listener): (Sender<String>, Receiver<String>) =
      mpsc::channel();

    // Start a listener that will except some data and exit at EOF
    let listen_thread = thread::spawn(move || {
      let listener = TcpListener::bind(TCP_LISTEN_SOCK)
        .expect(&format!("Could not start listener on {}", TCP_LISTEN_SOCK));

      // Notify test that listener is ready to avoid race failure
      to_test.send(String::from(MSG_THREAD_READY))
        .expect("Unable to send ready string to test thread");

      // wait for the connection from lyrebird
      let (ref mut in_stream, _) = listener.accept()
        .expect("Unable to accept connection");

      let mut data = String::new();

      // suck up data from client
      in_stream.read_to_string(&mut data)
               .expect("Unable to read data from peer");

      to_test.send(data)
        .expect("Unable to send peer data to test thread");
    });

    // Wait for listener thread to start accepting
    match from_listener.recv_timeout(Duration::from_secs(RESPONSE_WAIT)) {
      Ok(str) =>
        assert_eq!(str, MSG_THREAD_READY, "Thread responded with unexpected message"),
      Err(e) =>
        panic!("Unable to get ready message from listener thread: {:?}", e)
    };

    let mut log = get_test_log();

    // set up our net-send command
    let tcp_message = TcpMessage {
      dest: String::from(TCP_LISTEN_SOCK),
      data: String::from(DEFAULT_MESSAGE),
      timeout: 10
    };

    tcp_message.send(&mut log)
               .expect("Could not send TCP message");

    // check the message that was sent
    let recv_data = from_listener.recv_timeout(Duration::from_secs(RESPONSE_WAIT))
                                 .expect("Did not get data from listen thread");

    assert_eq!(recv_data, DEFAULT_MESSAGE,
               "Data received by TCP listener did not match data sent by net-send tcp");

    listen_thread.join()
      .expect("Could not join test listen thread to test thread");
  }
}
