pub fn collatz(n: u64) -> Option<u64> {
    match n {
        0 => None,
        1 => Some(0),
        _ => {
            let mut count: u64 = 0;
            let mut x = n;
        
            while x != 1 {
                if x % 2 == 0 { x /= 2 } else { x = x * 3 + 1} ;
                count += 1;
            }
            Some(count)    
        }
    }
}
