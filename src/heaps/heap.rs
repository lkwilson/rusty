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

heapify_heap_after_replace_root

/**
 * Vec must be non empty, and index must be in range. Otherwise, undefined behavior.
 */
fn heapify_from<F:Fn(&T, &T)->bool, T:std::fmt::Debug>(vec: &mut Vec<T>, compare: &F, node_index: usize) {
  let left_child_index = get_left_child_index(node_index);
  if left_child_index < vec.len() {
    heapify_from(vec, compare, left_child_index);
  }

  let right_child_index = get_right_child_index(node_index);
  if right_child_index < vec.len() {
    heapify_from(vec, compare, right_child_index);
  }

  let swap_index: Option<usize>;
  let node_value = &vec[node_index];
  if left_child_index < vec.len() && right_child_index < vec.len() {
    // has both children
    let left_child_value = &vec[left_child_index];
    let right_child_value = &vec[right_child_index];
    if compare(left_child_value, node_value) {
      // left is higher than root
      if compare(left_child_value, right_child_value) {
        // left is higher than right
        // thus left is the highest
        swap_index = Some(left_child_index);
      } else {
        // right is higher than left
        // thus right is the highest
        swap_index = Some(right_child_index);
      }
    } else if compare(right_child_value, node_value) {
      // left is not higher than root
      // right is higher than root
      // thus right is the highest
      swap_index = Some(right_child_index);
    } else {
      swap_index = None;
    }
  } else if left_child_index < vec.len() {
    // only left child compare
    if compare(&vec[left_child_index], node_value) {
      swap_index = Some(left_child_index);
    } else {
      swap_index = None;
    }
  } else if right_child_index < vec.len() {
    // only rigth child compare
    if compare(&vec[right_child_index], node_value) {
      swap_index = Some(right_child_index);
    } else {
      swap_index = None;
    }
  } else {
    swap_index = None;
  }
  match swap_index {
    Some(swap_index) => {
      vec.swap(node_index, swap_index);
      heapify_heap_after_replace_root(vec, compare, swap_index);
    },
    None => {}
  }
}

pub fn heapify<F:Fn(&T, &T)->bool, T:std::fmt::Debug>(vec: &mut Vec<T>, compare: &F) {
  if !vec.is_empty() {
    heapify_from(vec, &compare, 0);
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
