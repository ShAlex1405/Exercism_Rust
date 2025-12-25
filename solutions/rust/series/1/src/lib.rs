pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut vc: Vec<String> = Vec::new();
    
    if len > digits.len() { return vc; }
    
    if len == digits.len() {
        vc.push(String::from(digits));
        return vc;
    }
    
    let mut start: usize = 0;
    let mut end: usize = len;
    
    while end <= digits.len() {
        vc.push(String::from(&digits[start..end]));
        start += 1;
        end += 1;
    }
    
    vc
}
