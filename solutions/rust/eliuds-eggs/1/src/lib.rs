pub fn egg_count(display_value: u32) -> usize {
    let bin_val = format!("{:b}", display_value);
    bin_val.chars().filter(|c| *c == '1').count()
}
