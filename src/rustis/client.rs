use std::io::net::tcp::TcpStream;
use std::io::net::ip::SocketAddr;
use std::io::buffered::BufferedStream;
use std::str::replace;
use std::uint::parse_bytes;

use response::{Response,Status,Bulk};

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

  pub fn ping(&mut self) -> Response {
    self.send_command(~["PING"]);
    
    self.parse_response()
  }

  pub fn set(&mut self, key: &str, value: &str) -> Response {
    self.send_command(~[&"SET", key, value]);

    self.parse_response()
  }

  pub fn get(&mut self, key: &str) -> Response {
    self.send_command(~[&"GET", key]);

    self.parse_response()
  }

  fn parse_response(&mut self) -> Response {
    match self.stream.read_char() {
      Some(c) => {
        match c {
          '+' => self.parse_status(),
          '$' => self.parse_bulk(),
          _ => fail!("Unknown response type")
        }
      },
      _ => fail!("No response")
    }
  }

  fn parse_status(&mut self) -> Response {
    Status(chomp(self.stream.read_line().unwrap()))
  }

  fn parse_bulk(&mut self) -> Response {
    let len = self.parse_int();
    let res = Bulk(self.stream.read_bytes(len));

    self.stream.read_line();
    
    res
  }

  fn parse_int(&mut self) -> uint {
    let chomped_line = chomp(self.stream.read_line().unwrap());
    let chomped_bytes = chomped_line.as_bytes();
    parse_bytes(chomped_bytes, 10).unwrap()
  }

  fn send_command(&mut self, cmd_pieces: ~[&str]) {
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

fn chomp(s: ~str) -> ~str {
  replace(s, "\r\n", "")
}
