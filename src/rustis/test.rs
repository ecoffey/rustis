extern mod rustis;

use rustis::client::Client;
use rustis::response::Status;

#[test]
fn ping() {
  let mut c = Client::new("127.0.0.1:6379"); 

  let r = c.ping();

  let expected = Status(~"PONG");
  assert!(expected.eq(r));
}
