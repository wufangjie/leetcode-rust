struct MyCircularQueue {
    data: Vec<i32>,
    head: usize,
    tail: usize,
}

impl MyCircularQueue {

    fn new(k: i32) -> Self {
        Self {
            data: vec![0; k as usize + 1],
            head: 0,
            tail: 0,
        }
    }

    fn en_queue(&mut self, value: i32) -> bool {
        if self.is_full() {
            false
        } else {
            self.data[self.tail] = value;
            self.tail = self.next_index(self.tail);
            true
        }
    }

    fn de_queue(&mut self) -> bool {
        if self.is_empty() {
            false
        } else {
            self.head = self.next_index(self.head);
            true
        }
    }

    fn front(&self) -> i32 {
        if self.is_empty() { -1 } else { self.data[self.head] }
    }

    fn rear(&self) -> i32 {
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
        if i == self.data.len() - 1 { 0 } else { i + 1 }
    }

    #[inline]
    fn prev_index(&self, i: usize) -> usize {
        if i == 0 { self.data.len() - 1 } else { i - 1 }
    }
}
