pub fn max_rev<const N: usize>(prices: &[i32; N]) {
  max_rev_slice(&prices[0..N]);
}

pub fn max_rev_slice(prices: &[i32]) {
}

#[cfg(test)]
mod tests {
  use super::*;


  #[test]
  fn name() {
    let mut arr = [0i32; 30];
    for (idx, val) in (1..30).enumerate() {
      arr[idx] = 50 + val;
    }

    max_rev_slice(&arr[2..5]);
    max_rev(&arr);
  }
}