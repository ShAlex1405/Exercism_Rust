/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let digits_only = code.chars().all(|c| c.is_ascii_digit() || c.is_whitespace());
    if !digits_only || code.trim().len() <= 1 {return false};

    let nums: Vec<u32> = code.chars()
                            .filter(|c| *c != ' ')
                            .filter_map(|c| c.to_digit(10))
                            .collect();

    let mut sum: u32 = 0;

    for num in nums.iter().rev().skip(1).step_by(2) {
        let mut val = num * 2;
        if val > 9 {val -= 9;}
        sum += val;
    }

    for num in nums.iter().rev().step_by(2) {
        sum += num;
    }

    sum % 10 == 0
}
