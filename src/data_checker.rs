pub fn hamming(bits: &[i32; 16]) -> [i32; 2] {
    //! FUNCTION BREAKS IF THERE IS MORE THAN 2 ERRORED BITS
    //* RETURNS:
    //* [Location of errored bit (IS ONE OF 0..16), If master bit is wrong (IS EITHER 0 or 1)]

    // Remove all zero bits and pushes to stack
    let mut master_stack: Vec<i32> = Vec::new();
    for i in 0..16 {
        if bits[i] == 1 {
            master_stack.push(i as i32);
        }
    }

    // Pushes xor(prev, next) to prev
    let mut prev: i32 = master_stack[0];
    for i in 0..master_stack.len() {
        if i as i32 != 0 {
            prev = prev ^ master_stack[i];
        }
    }

    // If, master bit is present in the stack, then set to stack size minus 1
    // Else, set to stack size
    let stack_len: usize = if bits[0] == 1 {
        master_stack.len() - 1
    } else {
        master_stack.len()
    };

    // If, EVEN number of ones are present in stack (not including master bit, if present),
    // And, master bit indicates EVEN number of ones, Then set to 0
    // Else If, ODD number of ones are present in stack (not including master bit, if present),
    // And, master bit indicates ODD number of ones, Then set to 0
    // Else, set to 1
    let master_fault: i32 = if stack_len % 2 == 0 && bits[0] == 0 {
        0
    } else if stack_len % 2 != 0 && bits[0] == 1 {
        0
    } else {
        1
    };

    //* RETURNS:
    //* [Location of errored bit (IS ONE OF 0..16), If master bit is wrong (IS EITHER 0 or 1)]
    [prev, master_fault]
}
