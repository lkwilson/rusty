#[cfg(test)]
mod tests {
  fn func_with_const_if<T, const ENABLED: bool>(val: &T)
  where T: std::fmt::Debug {
    if ENABLED {
      println!("val: {:?}", val);
    }
  }

  #[test]
  fn call_the_func() {
    let val: i32 = 5;
    func_with_const_if::<_, true>(&val);
  }
}
