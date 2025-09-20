pub fn find(array: &[i32], key: i32) -> Option<usize> {
    match array.len() {
        0 => None,
        1 => if array[0] == key { Some(0) } else { None },
        _ => {
            let mut start = 0;
            let mut end = array.len() - 1;

            while start <= end {
                let mid = (start + end) / 2;
                match array[mid] {
                    x if x == key => return Some(mid),
                    x if x > key && mid != 0 => end = mid - 1,
                    _ => start = mid + 1
                }
            }
            None
        }
    }
}
