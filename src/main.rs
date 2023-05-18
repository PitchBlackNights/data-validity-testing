use data_validity_testing::data_checker;
use colored::*;

fn print_formatted_bits(bits: &[i32; 16]) {
    println!("bits = [");
    println!(
        "  {a}, {b}, {c}, {d},",
        a = bits[0],
        b = bits[1],
        c = bits[2],
        d = bits[3]
    );
    println!(
        "  {a}, {b}, {c}, {d},",
        a = bits[4],
        b = bits[5],
        c = bits[6],
        d = bits[7]
    );
    println!(
        "  {a}, {b}, {c}, {d},",
        a = bits[8],
        b = bits[9],
        c = bits[10],
        d = bits[11]
    );
    println!(
        "  {a}, {b}, {c}, {d}",
        a = bits[12],
        b = bits[13],
        c = bits[14],
        d = bits[15]
    );
    println!("]\n");
}

fn hamming_test() {
    println!("\n\n{}", "HAMMING'S CODE".green().underline());
    let bits: [i32; 16] = [0, 0, 1, 1, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1];
    let hamming_results = data_checker::hamming(&bits);
    print_formatted_bits(&bits);
    println!(
        "Position: {}\nMaster Fault: {}",
        hamming_results[0],
        hamming_results[1]
    );
}

fn main() {
    hamming_test();
}
