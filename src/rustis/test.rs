extern mod rustis;

use rustis::Client;

#[test]
fn ping() {
  let mut c = Client::new("127.0.0.1:6379"); 

  let r = c.ping();

  let expected = "PONG";
  assert!(expected == r);
}
