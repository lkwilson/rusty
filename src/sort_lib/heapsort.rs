use crate::heap_lib::heap;

pub fn heap_sort<F, T>(vec: &mut Vec<T>, compare: &F) where
    F: Fn(&T, &T)->bool {
  heap::heapify(vec, compare, vec.len());
  let size = vec.len();
  for index in (1..size).rev() {
    vec.swap(0, index);
    heap::heapify_from(vec, compare, 0, index);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  pub fn build_vector() -> Vec<i32> {
    vec![0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5]
  }

  fn is_sorted<T, F>(vec: &Vec<T>, compare: &F) -> bool
  where F: Fn(&T, &T)->bool {
    for index in 1..vec.len() {
      let prev_value = &vec[index-1];
      let curr_value = &vec[index];
      if !compare(prev_value, curr_value) {
        return false;
      }
    }
    true
  }

  #[test]
  fn max_heap_sort() {
    let mut heap = build_vector();

    heap_sort(&mut heap, &std::cmp::PartialOrd::ge);
    assert!(is_sorted(&mut heap, &std::cmp::PartialOrd::le));
  }

  #[test]
  fn min_heap_sort() {
    let mut heap = build_vector();

    heap_sort(&mut heap, &std::cmp::PartialOrd::le);
    assert!(is_sorted(&mut heap, &std::cmp::PartialOrd::ge));
  }
}
