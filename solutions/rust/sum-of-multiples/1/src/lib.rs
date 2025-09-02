pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if factors.is_empty() { return 0; }
    
    let mut scores: Vec<u32> = Vec::new();
    
    for f in factors {
        if *f == 0 { continue; }
        let mut lmt = limit - 1;
        
        while lmt >= *f {
            if lmt % f == 0 {
                scores.push(lmt);
                lmt -= f;
            } else {
                lmt -= 1;
            }
        }
    }
    scores.sort();
    scores.dedup();
    let res: u32 = scores.into_iter().sum();
    res
}