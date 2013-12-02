extern mod rustis;

use rustis::client::Client;
use rustis::response::Status;
use rustis::response::Bulk;

#[test]
fn ping() {
  let mut c = Client::new("127.0.0.1:6379"); 

  let r = c.ping();

  let expected = Status(~"PONG");

  assert!(r.eq(&expected));
}

#[test]
fn set_is_ok() {
  let mut c = Client::new("127.0.0.1:6379");

  let r = c.set("hello", "world");

  let expected = Status(~"OK");

  assert!(r.eq(&expected));
}

#[test]
fn get() {
  let mut c = Client::new("127.0.0.1:6379");

  c.set("hello", "world");

  let r = c.get("hello");

  let expected_str = ~"world";
  let expected = Bulk(expected_str.into_bytes());

  assert!(r.eq(&expected));
}
