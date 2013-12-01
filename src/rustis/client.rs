use std::io::net::tcp::TcpStream;
use std::io::net::ip::SocketAddr;
use std::io::buffered::BufferedStream;
use std::str::replace;

use response::{Response,Status};

pub struct Client {
  host: SocketAddr,
  priv stream: BufferedStream<TcpStream>
}

impl Client {
  pub fn new(host: &str) -> Client {
    match from_str(host) {
      Some(h) => {
        match TcpStream::connect(h) {
          Some(s) => Client { host: h, stream: BufferedStream::new(s) },
          None => fail!("Could not connect")
        }
      },
      None => fail!("Could not parse socket")
    }
  }

  pub fn ping(&mut self) -> ~Response {
    self.send_command(~[~"PING"]);
    
    self.parse_response()
  }

  fn parse_response(&mut self) -> ~Response {
    match self.stream.read_char() {
      Some(c) => {
        match c {
          '+' => self.parse_status(),
          _ => fail!("Unknown response type")
        }
      },
      _ => fail!("No response")
    }
  }

  fn parse_status(&mut self) -> ~Response {
    ~Status(replace(self.stream.read_line().unwrap(), "\r\n", ""))
  }

  fn send_command(&mut self, cmd_pieces: ~[~str]) {
    let mut cmd_str = ~"*";  
    cmd_str.push_str(cmd_pieces.len().to_str());
    cmd_str.push_str("\r\n");
    
    for cmd in cmd_pieces.iter() {
      cmd_str.push_str("$");
      cmd_str.push_str(cmd.len().to_str());
      cmd_str.push_str("\r\n");
      cmd_str.push_str(*cmd);
      cmd_str.push_str("\r\n");
    }

    self.stream.write(cmd_str.as_bytes());
    self.stream.flush();
  }
}
