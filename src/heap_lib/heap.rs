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

/**
 * This will heapify a vector from index node_index with heap property tester,
 * compare. compare(parent_value, child_value) will return true if the heap
 * property for those nodes is satisfied. note that this means compare(value,
 * value) is always true. This function assumes child nodes are heaps, but the
 * node at node_index may not be. size is usually vec.len(), but doesn't have to
 * be if you want to use the back elements, for say heap sort.
 */
fn heapify_from<T, F>(vec: &mut Vec<T>, compare: &F, node_index: usize, size: usize) where
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

pub fn heap_sort<F, T>(vec: &mut Vec<T>, compare: &F) where
    F: Fn(&T, &T)->bool {
  heapify(vec, compare, vec.len());
  let size = vec.len();
  for index in (1..size).rev() {
    vec.swap(0, index);
    heapify_from(vec, compare, 0, index);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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

  fn build_heap() -> Vec<i32> {
    vec![0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5]
  }

  #[test]
  fn max_heap() {
    let mut heap = build_heap();
    let size = heap.len();

    heapify(&mut heap, &std::cmp::PartialOrd::ge, size);
    assert!(is_heap(&heap, &std::cmp::PartialOrd::ge));
    assert!(!is_heap(&heap, &std::cmp::PartialOrd::le));
  }

  #[test]
  fn min_mean() {
    let mut heap = build_heap();
    let size = heap.len();

    heapify(&mut heap, &std::cmp::PartialOrd::le, size);
    assert!(is_heap(&heap, &std::cmp::PartialOrd::le));
    assert!(!is_heap(&heap, &std::cmp::PartialOrd::ge));
  }

  #[test]
  fn max_heap_sort() {
    let mut heap = build_heap();

    heap_sort(&mut heap, &std::cmp::PartialOrd::ge);
    assert!(is_sorted(&mut heap, &std::cmp::PartialOrd::le));
  }

  #[test]
  fn min_heap_sort() {
    let mut heap = build_heap();

    heap_sort(&mut heap, &std::cmp::PartialOrd::le);
    assert!(is_sorted(&mut heap, &std::cmp::PartialOrd::ge));
  }
}
