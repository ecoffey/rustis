pub enum Response {
  Status(~str),
  Bulk(~[u8])
}

impl Eq for Response {
  fn eq(&self, other: &Response) -> bool {
    match (self, other) {
      (&Status(ref s1), &Status(ref s2)) => s1 == s2,
      (&Bulk(ref b1), &Bulk(ref b2)) => b1 == b2,
      _ => false
    }
  }
}

impl ToStr for Response {
  fn to_str(&self) -> ~str {
    match self {
      &Status(ref status_str) => status_str.clone(),
      &Bulk(ref bulk_bytes) => bulk_bytes.to_str()
    }
  }
}
