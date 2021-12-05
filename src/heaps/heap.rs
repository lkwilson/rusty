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
 * Vec must be non empty, and index must be in range. Otherwise, undefined behavior.
 *
 * compare(parent, child) must return true, if it meets the heap property.
 */
fn heapify_from<T, F, const HEAPIFY_CHILDREN: bool>(vec: &mut Vec<T>, compare: &F, node_index: usize)
where F: Fn(&T, &T)->bool, T: std::fmt::Debug {
  let li = get_left_child_index(node_index);
  let ri = li + 1;
  let size = vec.len();
  if HEAPIFY_CHILDREN {
    if ri < size {
      heapify_from::<_, _, true>(vec, compare, li);
      heapify_from::<_, _, true>(vec, compare, ri);
    } else if li < size {
      heapify_from::<_, _, true>(vec, compare, li);
    }
  }

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
      heapify_from::<_, _, false>(vec, compare, swap_index);
    },
    None => {}
  }
}

fn heapify<F, T>(vec: &mut Vec<T>, compare: &F)
where F:Fn(&T, &T)->bool, T:std::fmt::Debug {
  if !vec.is_empty() {
    heapify_from::<_, _, true>(vec, compare, 0);
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
