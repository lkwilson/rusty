mod heap;

pub fn main() -> u8 {
  let heap: heap::Heap<50> = heap::Heap::new();

  println!("Created heap!");
  println!("Capacity: {}", heap.capacity());
  println!("Size: {}", heap.size());

  0
}
