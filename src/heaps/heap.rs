/*
These free functions are for heap management

note: index is 0 based
*/

pub fn get_parent_index(node_index: usize) -> usize {
  (node_index + 1) / 2 - 1
}

pub fn get_left_child_index(node_index: usize) -> usize {
  (node_index + 1) * 2 - 1
}

/**
 * This will heapify a vector from index node_index with heap property tester,
 * compare. compare(parent_value, child_value) will return true if the heap
 * property for those nodes is satisfied. This function assumes child nodes are
 * heaps, but the node at node_index may not be.
 */
fn heapify_from<T, F>(vec: &mut Vec<T>, compare: &F, node_index: usize) where
    F: Fn(&T, &T)->bool {
  let li = get_left_child_index(node_index);
  let ri = li + 1;
  let size = vec.len();

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
      heapify_from(vec, compare, swap_index);
    },
    None => {}
  }
}

/**
 * Vec must be non empty, and index must be in range. Otherwise, undefined behavior.
 *
 * compare(parent, child) must return true, if it meets the heap property.
 *
 * This function was inefficient for lots of reasons:
 * - we use recursion to get to the deepest leaves
 * - we heapify the leaf nodes when we know exactly where the leaf nodes are
 * - when ensuring children are heaps, we start from the root when we can jump
 *   straight to the parent of the last leaf node
 */
// fn heapify_from<T, F, const HEAPIFY_CHILDREN: bool>(vec: &mut Vec<T>, compare: &F, node_index: usize)
// where F: Fn(&T, &T)->bool, T: std::fmt::Debug {...}

fn heapify<F, T>(vec: &mut Vec<T>, compare: &F)
where F:Fn(&T, &T)->bool, T:std::fmt::Debug {
  let size = vec.len();
  if size > 1 {
    let index = size - 1;
    let index = get_parent_index(index);
    for index in (0..(index+1)).rev() {
      heapify_from(vec, compare, index);
    }
  }
}

pub fn is_heap<F, T>(vec: &Vec<T>, compare: &F) -> bool
where F: Fn(&T, &T)->bool, T: std::fmt::Debug {
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

pub fn main() -> u8 {
  let mut heap = vec![0, 1, 2, 3, 4, 5, 0, 1, 2, 3, 4, 5];
  println!("Pre heapify: {:?}", heap);

  heapify(&mut heap, &std::cmp::PartialOrd::ge);
  println!("Post max heapify: {:?}", heap);
  assert!(is_heap(&heap, &std::cmp::PartialOrd::ge));
  assert!(!is_heap(&heap, &std::cmp::PartialOrd::le));

  heapify(&mut heap, &std::cmp::PartialOrd::le);
  println!("Post min heapify: {:?}", heap);
  assert!(is_heap(&heap, &std::cmp::PartialOrd::le));
  assert!(!is_heap(&heap, &std::cmp::PartialOrd::ge));

  0
}
