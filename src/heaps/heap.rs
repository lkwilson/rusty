// TODO: non hard coded type and capacity
pub struct Heap<const CAPACITY:usize> {
  nodes: [i32; CAPACITY],
  active: usize,
}

pub fn parent(idx: usize) -> usize {
  (idx + 1) / 2
}

pub fn left_child(idx: usize) -> usize {
  (idx + 1) * 2 - 1
}

pub fn right_child(idx: usize) -> usize {
  (idx + 1) * 2
}

impl<const CAPACITY:usize> Heap<CAPACITY> {
  pub fn new() -> Self {
    Heap {
      nodes: [0; CAPACITY],
      active: 0,
    }
  }

  pub fn capacity(&self) -> usize {
    self.nodes.len()
  }

  pub fn size(&self) -> usize {
    self.active
  }
}
