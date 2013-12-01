pub enum Response {
  Status(~str)
}

impl Eq for Response {
  fn eq(&self, other: &Response) -> bool {
    match (self, other) {
      (&Status(ref s1), &Status(ref s2)) => s1 == s2
    }
  }
}

impl ToStr for Response {
  fn to_str(&self) -> ~str {
    match self {
      &Status(ref status_str) => status_str.clone()
    }
  }
}
