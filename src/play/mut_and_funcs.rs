#[cfg(test)]
mod tests {
  fn mut_me(val: &mut i32) {
    *val = 2 as i32;
  }

  fn mut_vec(vec: &mut Vec<i32>) {
    (*vec).push(4 as i32);
    vec.push(5 as i32);
  }

  #[test]
  fn call_mut() {
    let mut a = 5 as i32;
    mut_me(&mut a);
    println!("Post: {}", a);

    let mut vec = vec![1, 2, 3];

    mut_vec(&mut vec);
    println!("Post: {:?}", vec);
  }
}
