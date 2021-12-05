fn func_with_const_if<const ENABLED: bool, T>(val: &T)
where T: std::fmt::Debug {
  if ENABLED {
    println!("val: {:?}", val);
  }
}

pub fn main() -> u8 {
  let val: i32 = 5;
  func_with_const_if::<true>(&val);

  0
}
