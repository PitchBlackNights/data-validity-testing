pub fn hamming(bits: [i32; 16]) -> i32 {
  let mut prev: i32 = 0;
  let mut stack: Vec<i32> = Vec::new();

  // Remove all zero bits and pushes to stack
  for i in 0..16 {
      if bits[i] == 1 {
          stack.push(i as i32);
      }
  }

  // Pushes xor(prev, next) to prev
  for i in 0..stack.len() {
      if i as i32 != 0 {
          prev = prev ^ stack[i];
      } else {
          prev = stack[0];
      }
  }

  prev
}
