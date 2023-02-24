struct MyCircularDeque {
    data: Vec<i32>,
    head: usize,
    tail: usize,
    cap: usize,
}

impl MyCircularDeque {

    fn new(k: i32) -> Self {
        let cap = k as usize;
        Self {
            data: vec![0; cap + 1],
            head: 0,
            tail: 0,
            cap
        }
    }

    fn insert_front(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.head = self.prev_index(self.head);
            self.data[self.head] = value;
            true
        }
    }

    fn insert_last(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.data[self.tail] = value;
            self.tail = self.next_index(self.tail);
            true
        }
    }

    fn delete_front(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.head = self.next_index(self.head);
            true
        }
    }

    fn delete_last(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.tail = self.prev_index(self.tail);
            true
        }

    }

    fn get_front(&self) -> i32 {
        if self.is_empty() { -1 } else { self.data[self.head] }
    }

    fn get_rear(&self) -> i32 {
        if self.is_empty() { -1 } else { self.data[self.prev_index(self.tail)] }
    }

    #[inline]
    fn is_full(&self) -> bool {
        self.next_index(self.tail) == self.head
    }

    #[inline]
    fn is_empty(&self) -> bool {
        self.head == self.tail
    }

    #[inline]
    fn next_index(&self, i: usize) -> usize {
        if i == self.cap { 0 } else { i + 1 }
    }

    #[inline]
    fn prev_index(&self, i: usize) -> usize {
        if i == 0 { self.cap } else { i - 1 }
    }
}
