
/// sort vec using quick sort.
///
/// compare(left, right) returns true if left comes before right.
/// pre: start_idx < end_idx => vec contains at least two elements
fn quicksort_sub<F, T>(vec: &mut Vec<T>, compare: &F, start_idx: usize, end_idx: usize) where
    F: Fn(&T, &T)->bool {
  let mut partition_idx = start_idx;
  for idx in start_idx..end_idx {
    if compare(&vec[idx], &vec[end_idx]) {
      vec.swap(partition_idx, idx);
      partition_idx += 1;
    }
  }
  vec.swap(partition_idx, end_idx);
  if partition_idx + 1 < end_idx {
    quicksort_sub(vec, compare, partition_idx+1, end_idx);
  }
  if start_idx + 1 < partition_idx {
    quicksort_sub(vec, compare, start_idx, partition_idx-1);
  }
}

/// sort vec using quick sort. compare(left, right) returns true if left comes
/// before right.
pub fn quicksort<F, T>(vec: &mut Vec<T>, compare: &F) where
    F: Fn(&T, &T)->bool {
  if vec.len() > 1 {
    quicksort_sub(vec, compare, 0, vec.len()-1)
  }
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
  fn test_short() {
    let mut data: Vec<i32> = vec![];
    quicksort(&mut data, &std::cmp::PartialOrd::lt);
    assert_eq!(data, vec![]);
  }

  #[test]
  fn test_one() {
    let mut data: Vec<i32> = vec![1];
    quicksort(&mut data, &std::cmp::PartialOrd::lt);
    assert_eq!(data, vec![1]);
  }

  #[test]
  fn test_two() {
    let mut data: Vec<i32> = vec![2, 1];
    quicksort(&mut data, &std::cmp::PartialOrd::lt);
    assert_eq!(data, vec![1, 2]);
  }

  #[test]
  fn test_three() {
    let mut data: Vec<i32> = vec![2, 1, 3];
    quicksort(&mut data, &std::cmp::PartialOrd::lt);
    assert_eq!(data, vec![1, 2, 3]);
  }

  #[test]
  fn test_quicksort() {
    let mut data = build_vector();
    quicksort(&mut data, &std::cmp::PartialOrd::lt);
    assert_eq!(data, build_vector_sorted());
  }
}
