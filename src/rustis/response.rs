#[deriving(Eq, TotalEq)]
pub enum Response {
  Status(~str)
}

