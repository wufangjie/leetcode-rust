struct Solution;

const M1: u8 = 5;
const M2: i32 = (1 << M1) - 1;
const N: usize = (1000001 >> M1) + 1;

struct MyHashSet {
    data: [u32; N],
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {

    fn new() -> Self {
        MyHashSet {
            data: [0; N],
        }
    }

    fn add(&mut self, key: i32) {
        self.data[key as usize >> M1] |= 1 << (key & M2);
    }

    fn remove(&mut self, key: i32) {
        self.data[key as usize >> M1] &= !(1 << (key & M2));
    }

    fn contains(&self, key: i32) -> bool {
        (self.data[key as usize >> M1] & (1 << (key & M2))) > 0
    }
}

/**
 * Your MyHashSet object will be instantiated and called as such:
 * let obj = MyHashSet::new();
 * obj.add(key);
 * obj.remove(key);
 * let ret_3: bool = obj.contains(key);
 */
