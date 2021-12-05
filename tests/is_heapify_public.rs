/// ```compile_fail
/// let mut vec = vec![1, 2, 3, 4];
/// let size = vec.len();
/// rusty::heap_lib::heap::heapify_from(&mut vec, &std::cmp::PartialOrd::ge, 1, size);
/// rusty::heap_lib::heap::heapify_from(&mut vec, &std::cmp::PartialOrd::ge, 0, size);
/// assert_eq!(vec, vec![4, 2, 3, 1]);
/// ```
fn is_heapify_from_visible() {
}

#[test]
fn run_heapify_from_visible() {
  is_heapify_from_visible();
}
