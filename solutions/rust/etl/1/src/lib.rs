use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();

    for (key, value) in h {
        let value_low = value.iter().map(|c| c.to_ascii_lowercase());

        for val in value_low {
            result.insert(val, *key);
        }
    }

    result
}