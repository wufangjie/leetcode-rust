use std::collections::HashMap;

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut count = HashMap::with_capacity(order.len());
        for c in order.as_bytes() {
            count.insert(c, 0);
        }
        let mut res = String::new();
        for c in s.as_bytes() {
            if let Some(i) = count.get_mut(c) {
                *i += 1;
            } else {
                res.push(*c as char);
            }
        }
        for c in order.as_bytes() {
            for _ in 0..*count.get(c).unwrap() {
                res.push(*c as char);
            }
        }
        res
    }
}
