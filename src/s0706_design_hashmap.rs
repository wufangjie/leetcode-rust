const M: u8 = 8;
const N: usize = (1000001 >> M) + 1;

struct MyHashMap {
    data: Vec<Vec<(i32, i32)>>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    fn new() -> Self {
        MyHashMap {
            data: vec![vec![]; N],
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        let lst = &mut self.data[key as usize >> M];
        for mut pair in lst.iter_mut() {
            if pair.0 == key {
                *pair = (key, value);
                return;
            }
        }
        lst.push((key, value));
    }

    fn get(&self, key: i32) -> i32 {
        let lst = &self.data[key as usize >> M];
        for &(k, v) in lst.iter() {
            if k == key {
                return v;
            }
        }
        -1
    }

    fn remove(&mut self, key: i32) {
        let lst = &mut self.data[key as usize >> M];
        let n = lst.len();
        let mut found = n;
        for (i, &(k, _)) in lst.iter().enumerate() {
            if k == key {
                found = i;
                break;
            }
        }
        if found == n - 1 {
            lst.pop();
        } else if found < n {
            lst[found] = lst.pop().unwrap();
        }
    }
}

/**
 * Your MyHashMap object will be instantiated and called as such:
 * let obj = MyHashMap::new();
 * obj.put(key, value);
 * let ret_2: i32 = obj.get(key);
 * obj.remove(key);
 */
