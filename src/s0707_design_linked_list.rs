struct MyLinkedList {
    head: Node,
    length: usize,
}

struct Node {
    val: i32,
    next: Option<Box<Node>>
}

impl Node {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    pub fn cons(val: i32, next: Option<Box<Node>>) -> Self {
        Self {val, next}
    }
}

impl MyLinkedList {

    fn new() -> Self {
        let head = Node::new(0); // dummy
        Self { head, length: 0 }
    }

    /// If the index is invalid, return -1
    fn get(&self, index: i32) -> i32 {
        if (index as usize) >= self.length {
            -1
        } else {
            let mut p = &self.head.next;
            for _ in 0..index {
                p = &p.as_ref().unwrap().next;
            }
            p.as_ref().unwrap().val
        }
    }

    fn add_at_head(&mut self, val: i32) {
        let old_head = self.head.next.take();
        self.head.next = Some(Box::new(Node::cons(val, old_head)));
        self.length += 1;
    }

    fn add_at_tail(&mut self, val: i32) {
        let mut p = &mut self.head.next;
        while p.is_some() {
            p = &mut p.as_mut().unwrap().next;
        }
        *p = Some(Box::new(Node::new(val)));
        self.length += 1;
    }

    /// If index is greater than the length, the node will not be inserted
    fn add_at_index(&mut self, index: i32, val: i32) {
        if (index as usize) <= self.length {
            let mut p = &mut self.head.next;
            for _ in 0..index {
                p = &mut p.as_mut().unwrap().next;
            }
            let mut node = Some(Box::new(Node::new(val)));
            std::mem::swap(p, &mut node);
            std::mem::swap(&mut p.as_mut().unwrap().next, &mut node);
            self.length += 1;
        }
    }

    /// if the index is valid
    fn delete_at_index(&mut self, index: i32) {
        if (index as usize) < self.length {
            let mut p = &mut self.head.next;
            for _ in 0..index {
                p = &mut p.as_mut().unwrap().next;
            }
            let mut node = p.as_mut().unwrap().next.take();
            std::mem::swap(p, &mut node);
            self.length -= 1;
        }
    }
}
