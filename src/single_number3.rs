// Constraints: O(n) time, O(1) space
pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
    let xor = nums.iter().fold(0, |acc, x| acc ^ *x); // = a ^ b
    let msb = (xor & (xor - 1)) ^ xor; // only a has most significant bit set
    let a = nums // xor all nums with MSB set, necessarilly will equal a
        .into_iter()
        .filter(|x| *x & msb != 0)
        .fold(0, |acc, x| acc ^ x);
    return vec![a, a ^ xor]; // a ^ b ^ a = b
}
