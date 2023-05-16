use data_validity_testing::data_checker;

fn main() {
    let bits: [i32; 16] = [
        0, 0, 1, 1,
        1, 0, 1, 0,
        0, 0, 0, 0,
        1, 0, 0, 1
    ];
    println!("{:?}", bits);
    println!("{}", data_checker::hamming(bits));
}
