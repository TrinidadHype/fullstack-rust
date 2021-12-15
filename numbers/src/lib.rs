pub fn print(limit: u8) {
    let nums = generate_seq(limit);
    output_sequence(&nums);
}

fn generate_seq(limit: u8) -> Vec<u8> {
    (1..=limit).collect()
}

fn output_sequence(nums: &[u8]) {
    for num in nums {
        println!("{}", num);
    }
}

#[test]
fn generate_sequence_should_work() {
    let result = generate_seq(3);
    assert_eq!(result, [1, 2, 3]);
}
