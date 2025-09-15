pub fn abbreviate(phrase: &str) -> String {
    phrase.split(|c: char| c == ' ' || c == '-' || c == '_')
        .flat_map(spit_by_uppercase)
        .filter_map(|w| w.chars().next())
        .map(|c| c.to_ascii_uppercase())
        .collect()
}


pub fn spit_by_uppercase(s: &str) -> Vec<&str> {
    let mut vec = Vec::new();
    let mut start = 0;
    
    if s.chars().any(|c| !c.is_uppercase()) {
        for (i, c) in s.char_indices().skip(1) {
            if c.is_uppercase() {
                vec.push(&s[start..i]);
                start = i;
            }
        }   
    }
    
    vec.push(&s[start..]);
    vec
}