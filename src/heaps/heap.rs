/*
These free functions are for heap management

note: index is 0 based
*/

pub fn get_parent_index(node_index: usize) -> usize {
  let parent_index = (node_index + 1) / 2 - 1;
  parent_index
}

pub fn get_left_child_index(node_index: usize) -> usize {
  let left_child_index = (node_index + 1) * 2 - 1;
  left_child_index
}

pub fn get_right_child_index(node_index: usize) -> usize {
  let right_child_index = (node_index + 1) * 2;
  right_child_index
}

/**
 * Vec must be non empty, and index must be in range. Otherwise, undefined behavior.
 */
fn heapify_from<T, F>(vec: &mut Vec<T>, compare: &F, node_index: usize, heapify_children: bool)
where F:Fn(&T, &T)->bool, T:std::fmt::Debug {
  let li = get_left_child_index(node_index);
  let ri = li + 1;
  let size = vec.len();
  if heapify_children {
    if ri < size {
      heapify_from(vec, compare, li, true);
      heapify_from(vec, compare, ri, true);
    } else if li < size {
      heapify_from(vec, compare, li, true);
    }
  }

  let mut swap_index = None;
  let nv = &vec[node_index];
  if ri < size {
    let lv = &vec[li];
    let rv = &vec[ri];
    if compare(rv, lv) { // rv >= lv
      if compare(rv, nv) { // lv <= rv >= nv
        swap_index = Some(ri);
      } // lv <= rv < nv
    } else if compare(lv, nv) { // rv < lv >= nv
      swap_index = Some(li);
    }
  } else if li < size && compare(&vec[li], nv) {
    swap_index = Some(li);
  }
  match swap_index {
    Some(swap_index) => {
      vec.swap(swap_index, node_index);
      heapify_from(vec, compare, swap_index, false);
    },
    None => {}
  }
}

fn heapify<F, T>(vec: &mut Vec<T>, compare: &F)
where F:Fn(&T, &T)->bool, T:std::fmt::Debug {
  if !vec.is_empty() {
    heapify_from(vec, &compare, 0, true);
  }
}

fn is_heap_from<F:Fn(&T, &T)->bool, T>(vec: &Vec<T>, compare: &F, node_index: usize) -> bool {
  let vec_length = vec.len();
  let node_value = &vec[node_index];
  let left_child_index = get_left_child_index(node_index);
  let right_child_index = get_right_child_index(node_index);
  let has_left_child = left_child_index < vec_length;
  let has_right_child = right_child_index < vec_length;

  (!has_left_child || compare(node_value, &vec[left_child_index])) &&
  (!has_right_child || compare(node_value, &vec[right_child_index])) &&
  (!has_left_child || is_heap_from(vec, compare, left_child_index)) &&
  (!has_right_child || is_heap_from(vec, compare, right_child_index))
}

pub fn is_heap<F:Fn(&T, &T)->bool, T>(vec: &Vec<T>, compare: &F) -> bool {
  vec.is_empty() || is_heap_from(vec, compare, 0)
}

pub fn main() -> u8 {
  let mut heap = vec![0, 1, 2, 3, 4, 5];
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
