use std::collections::HashMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let mut res: Vec<&'static str> = Vec::new();
    
    let stds: HashMap<&str, usize> = [
        ("Alice", 0), ("Bob", 2), ("Charlie", 4), ("David", 6),
        ("Eve", 8), ("Fred", 10), ("Ginny", 12), ("Harriet", 14), 
        ("Ileana", 16), ("Joseph", 18), ("Kincaid", 20), ("Larry", 22),
    ].into_iter().collect();
    
    let flwrs: HashMap<char, &str> = [
        ('G', "grass"), ('C', "clover"), ('R', "radishes"), ('V', "violets"),
    ].into_iter().collect();
    
    let rows: Vec<&str> = diagram.split_whitespace().collect();
    let fst_row = rows[0].chars().collect::<Vec<char>>();
    let sec_row = rows[1].chars().collect::<Vec<char>>();
    
    if let Some(step) = stds.get(student) {
       let first = *step;
       let sec = first + 1;
       
       if let (Some(f_value_f_row), Some(s_value_f_row), 
            Some(f_value_s_row), Some(s_value_s_row)) = (flwrs.get(&fst_row[first]), 
                flwrs.get(&fst_row[sec]), flwrs.get(&sec_row[first]), flwrs.get(&sec_row[sec])) {
                
            res.push(f_value_f_row);
            res.push(s_value_f_row);
            res.push(f_value_s_row);
            res.push(s_value_s_row);
       }
    }
    
    res
}
