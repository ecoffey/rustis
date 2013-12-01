extern mod rustis;

use rustis::client::Client;
use rustis::response::Status;

#[test]
fn ping() {
  let mut c = Client::new("127.0.0.1:6379"); 

  let r = c.ping();

  println(r.to_str());

  let expected = ~Status(~"PONG");

  assert!(r.eq(&expected));
}
