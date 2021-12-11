/*
These free functions are for heap management

note: index is 0 based
*/

fn get_parent_index(node_index: usize) -> usize {
  (node_index + 1) / 2 - 1
}

fn get_left_child_index(node_index: usize) -> usize {
  (node_index + 1) * 2 - 1
}

/// This will heapify a vector from index node_index with heap property tester,
/// compare. compare(parent_value, child_value) will return true if the heap
/// property for those nodes is satisfied. note that this means compare(value,
/// value) is always true. This function assumes child nodes are heaps, but the
/// node at node_index may not be. size is usually vec.len(), but doesn't have
/// to be if you want to use the back elements, for say heap sort.
pub(super) fn heapify_from<T, F>(vec: &mut Vec<T>, compare: &F, node_index: usize, size: usize) where
    F: Fn(&T, &T)->bool {
  let li = get_left_child_index(node_index);
  let ri = li + 1;

  let mut swap_index = None;
  let nv = &vec[node_index];
  if ri < size { // both children
    let lv = &vec[li];
    let rv = &vec[ri];
    if compare(rv, lv) { // rv higher
      if !compare(nv, rv) { // not heap
        swap_index = Some(ri);
      }
    } else if !compare(nv, lv) { // lv higher and not heap
      swap_index = Some(li);
    }
  } else if li < size && !compare(nv, &vec[li]) { // only left child, not heap
    swap_index = Some(li);
  }
  match swap_index {
    Some(swap_index) => {
      vec.swap(swap_index, node_index);
      heapify_from(vec, compare, swap_index, size);
    },
    None => {}
  }
}

/// Push a value into a heap
pub fn insert_heap<F, T>(val: T, vec: &mut Vec<T>, compare: &F)
where F:Fn(&T, &T)->bool {
  let mut new_value_idx = vec.len();
  vec.push(val);
  while new_value_idx != 0 {
    let parent_idx = get_parent_index(new_value_idx);
    if !compare(&vec[parent_idx], &vec[new_value_idx]) {
      vec.swap(parent_idx, new_value_idx);
      new_value_idx = parent_idx;
    } else {
      return
    }
  }
}

/// Pop the root node out of a heap. Assumes size is non zero. 
/// Swap the 
pub fn extract_heap<F, T>(vec: &mut Vec<T>, compare: &F) -> T
where F:Fn(&T, &T)->bool {
  let size = vec.len();
  let mut parent_idx = 0;

  // TOOD: we keep swapping the root element, we can just assign log(n) times
  // and return instead of swap log(n) times
  loop {
    let left_child_idx = get_left_child_index(parent_idx);
    // TODO we know the height of the tree, no need to check for size at all
    if left_child_idx + 1 < size {
      if compare(&vec[left_child_idx], &vec[left_child_idx + 1]) {
        vec.swap(parent_idx, left_child_idx);
        parent_idx = left_child_idx;
      } else {
        let right_child_idx = left_child_idx + 1;
        vec.swap(parent_idx, right_child_idx);
        parent_idx = right_child_idx;
      }
    } else {
      vec.swap(parent_idx, left_child_idx);
      // only left child. swap pop and return
    }
  }
}

/// give me a vector, and I'll heapify it.
pub fn heapify<F, T>(vec: &mut Vec<T>, compare: &F, size: usize)
where F:Fn(&T, &T)->bool {
  if size > 1 {
    let index = size - 1;
    let index = get_parent_index(index);
    for index in (0..(index+1)).rev() {
      heapify_from(vec, compare, index, size);
    }
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

  fn is_heap<F, T>(vec: &Vec<T>, compare: &F) -> bool
  where F: Fn(&T, &T)->bool {
    for (index, node) in vec.iter().enumerate() {
      if index == 0 {
        continue;
      }
      let parent_index = get_parent_index(index);
      if !compare(&vec[parent_index], node) {
        return false;
      }
    }
    true
  }

  #[test]
  fn extract_and_insert_heap() {
    let data = build_vector();
    let mut heap = Vec::new();
    let compare = &std::cmp::PartialOrd::ge;
    for el in data {
      let len = heap.len();
      insert_heap(el, &mut heap, compare, len);
    }
    let mut output = Vec::new();
    while !heap.is_empty() {
      let len = heap.len();
      let val = extract_heap(&mut heap, compare, len);
      println!("Popped: {}", val);
      output.push(val);
    }
    let sorted: Vec<i32> = build_vector_sorted();

    assert_eq!(sorted.len(), output.len());
    assert_eq!(sorted, output);
  }

  #[test]
  fn parent_child_index_relative() {
    for i in 1..1000 {
      let li = get_left_child_index(i);
      let ri = li + 1;
      assert_eq!(i, get_parent_index(li));
      assert_eq!(i, get_parent_index(ri));
    }
  }

  #[test]
  fn child_index_hard_coded_values() {
    assert_eq!(get_left_child_index(7), 15);
    assert_eq!(get_left_child_index(0), 1);
    assert_eq!(get_left_child_index(1), 3);
    assert_eq!(get_left_child_index(2), 5);
    assert_eq!(get_left_child_index(10), 21);
  }

  #[test]
  fn max_heap() {
    let mut heap = build_vector();
    let size = heap.len();

    heapify(&mut heap, &std::cmp::PartialOrd::ge, size);
    assert!(is_heap(&heap, &std::cmp::PartialOrd::ge));
    assert!(!is_heap(&heap, &std::cmp::PartialOrd::le));
  }

  #[test]
  fn min_mean() {
    let mut heap = build_vector();
    let size = heap.len();

    heapify(&mut heap, &std::cmp::PartialOrd::le, size);
    assert!(is_heap(&heap, &std::cmp::PartialOrd::le));
    assert!(!is_heap(&heap, &std::cmp::PartialOrd::ge));
  }
}
