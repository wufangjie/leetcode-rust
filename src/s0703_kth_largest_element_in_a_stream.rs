use utils::Heap;

struct KthLargest {
    heap: Heap<i32>,
    size: usize
}


impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap = Heap::with_capacity(k as usize);
        let mut obj = Self { heap, size: k as usize };
        for v in nums.into_iter() {
            obj.try_add(v);
        }
        obj
    }

    fn add(&mut self, val: i32) -> i32 {
        self.try_add(val);
        *self.heap.peek().unwrap()
    }

    fn try_add(&mut self, val: i32) {
        if self.heap.len() == self.size {
            self.heap.pushpop(val);
        } else {
            self.heap.push(val);
        }
    }
}
