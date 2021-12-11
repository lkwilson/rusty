/// sort vec using quick sort. compare(left, right) returns true if left comes
/// before right.
pub fn quicksort<F, T>(vec: &mut Vec<T>, compare: &F) where
    F: Fn(&T, &T)->bool {
}

#[cfg(test)]
mod tests {
  use super::*;
  pub fn build_vector() -> Vec<i32> {
    vec![0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5]
  }

  pub fn build_vector_sorted() -> Vec<i32> {
    vec![0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5]
  }

  #[test]
  fn test_quicksort() {
    let data = build_vector();
    quicksort(&mut data, &std::cmp::PartialOrd::le);
    assert_eq!(data, build_vector_sorted());
  }
}
