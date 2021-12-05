/*
These free functions are for heap management

note: index is 0 based
*/

pub fn get_parent_index(node_index: usize) -> usize {
  let parent_index = (node_index + 1) / 2 - 1;
  // println!("Getting parent index: {} -> {}", node_index, parent_index);
  parent_index
}

pub fn get_left_child_index(node_index: usize) -> usize {
  let left_child_index = (node_index + 1) * 2 - 1;
  // println!("Getting left child index: {} -> {}", node_index, left_child_index);
  // assert_eq!(node_index, get_parent_index(left_child_index));
  left_child_index
}

pub fn get_right_child_index(node_index: usize) -> usize {
  let right_child_index = (node_index + 1) * 2;
  // println!("Getting right child index: {} -> {}", node_index, right_child_index);
  // assert_eq!(node_index, get_parent_index(right_child_index));
  right_child_index
}

/**
 * Vec must be non empty, and index must be in range. Otherwise, undefined behavior.
 */
pub fn heapify_from<F:Fn(&T, &T)->bool, T:std::fmt::Debug>(vec: &mut Vec<T>, compare: &F, node_index: usize) {
  println!("Heapifying {:?} from index {}", vec, node_index);
  let left_child_index = get_left_child_index(node_index);
  if left_child_index < vec.len() {
    println!("Heapifying left child found at index {}", left_child_index);
    heapify_from(vec, compare, left_child_index);
  }

  let right_child_index = get_right_child_index(node_index);
  if right_child_index < vec.len() {
    println!("Heapifying right child found at index {}", right_child_index);
    heapify_from(vec, compare, right_child_index);
  }

  let node_value = &vec[node_index];
  if left_child_index < vec.len() && right_child_index < vec.len() {
    // has both children
    let left_child_value = &vec[left_child_index];
    let right_child_value = &vec[right_child_index];
    println!("Left and right child found: root:{:?} left:{:?} right:{:?}", node_value, left_child_value, right_child_value);
    if compare(left_child_value, node_value) {
      // left is higher than root
      if compare(left_child_value, right_child_value) {
        // left is higher than right
        // thus left is the highest
        println!("left child was highest");
        vec.swap(node_index, left_child_index);
      } else {
        // right is higher than left
        // thus right is the highest
        println!("right child was highest");
        vec.swap(node_index, right_child_index);
      }
    } else if compare(right_child_value, node_value) {
      // left is not higher than root
      // right is higher than root
      // thus right is the highest
      println!("right child was highest");
      vec.swap(node_index, right_child_index);
    }
    println!("Post: root:{:?} left:{:?} right:{:?}", &vec[node_index], &vec[left_child_index], &vec[right_child_index]);
  } else if left_child_index < vec.len() {
    // only left child compare
    if compare(&vec[left_child_index], node_value) {
      println!("only child, left child, was higher");
      vec.swap(node_index, left_child_index);
    }
    println!("Post: root:{:?} left:{:?} right:-", &vec[node_index], &vec[left_child_index]);
  } else if right_child_index < vec.len() {
    // only rigth child compare
    if compare(&vec[right_child_index], node_value) {
      println!("only child, right child, was higher");
      vec.swap(node_index, right_child_index);
    }
    println!("Post: root:{:?} left:- right:{:?}", &vec[node_index], &vec[right_child_index]);
  }
  println!("Done heapifying from index {}", node_index);
}

pub fn heapify<F:Fn(&T, &T)->bool, T:std::fmt::Debug>(vec: &mut Vec<T>, compare: &F) {
  if !vec.is_empty() {
    heapify_from(vec, &compare, 0);
  }
}

pub fn is_heap_from<F:Fn(&T, &T)->bool, T>(vec: &Vec<T>, compare: &F, node_index: usize) -> bool {
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
