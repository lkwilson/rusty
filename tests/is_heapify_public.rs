#[test]
fn is_heapify_from_visible() {
  let mut vec = vec![1, 2, 3, 4];
  let size = vec.len();
  // rusty::heap_lib::heap::heapify_from(&mut vec, &std::cmp::PartialOrd::ge, 1, size);
  // rusty::heap_lib::heap::heapify_from(&mut vec, &std::cmp::PartialOrd::ge, 0, size);
  rusty::heap_lib::heap::heapify(&mut vec, &std::cmp::PartialOrd::ge, size);
  assert_eq!(vec, vec![4, 2, 3, 1]);
}