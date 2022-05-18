use std::collections::HashMap;
use std::ptr;

#[derive(Debug)]
struct LRUCache {
    capacity: usize,
    data: HashMap<i32, Box<Node<i32, i32>>>, // use box to keep same memory address (for rehash)
    cache: LinkedList<i32, i32>,
    count: usize,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        Self {
            capacity,
            data: HashMap::with_capacity(capacity), //new(),
            cache: LinkedList::new(),
            count: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.data.get_mut(&key) {
            self.cache.update_node(node.as_mut_ptr());
            node.val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.data.get_mut(&key) {
            self.cache.update_node(node.as_mut_ptr());
            node.val = value;
        } else {
            if self.count == self.capacity {
                self.data.remove(self.cache.remove_head());
            } else {
                self.count += 1;
            }
            let val = Box::new(Node::new(key, value));
            let p = val.as_mut_ptr();
            self.data.insert(key, val);
            self.cache.push_back(p);
        }
    }
}

#[derive(Debug)]
pub struct Node<K, V> {
    key: K, // for fast pop from hashmap
    val: V,
    prev: *mut Node<K, V>,
    next: *mut Node<K, V>,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, val: V) -> Self {
        Self {
            key,
            val,
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        }
    }

    pub fn as_mut_ptr(&self) -> *mut Self {
        self as *const Self as *mut Self
    }
}

/// no owned data
#[derive(Debug)]
pub struct LinkedList<K, V> {
    head: *mut Node<K, V>,
    tail: *mut Node<K, V>,
}

impl<K, V> LinkedList<K, V> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    pub fn remove_node(&mut self, p: *mut Node<K, V>) -> &K {
        let node = unsafe { &mut *p };

        if self.head == self.tail {
            self.head = ptr::null_mut();
            self.tail = ptr::null_mut();
        } else if p == self.head {
            self.head = node.next;
        } else if p == self.tail {
            self.tail = node.prev;
        } else {
            unsafe {
                (*node.prev).next = node.next;
                (*node.next).prev = node.prev;
            }
        }
        &node.key //.clone() //TODO: is it ok?
    }

    pub fn remove_head(&mut self) -> &K {
        self.remove_node(self.head)
    }

    pub fn push_back(&mut self, p: *mut Node<K, V>) {
        if self.head.is_null() {
            self.head = p;
        } else {
            unsafe {
                (*self.tail).next = p;
                (*p).prev = self.tail;
            }
        }
        self.tail = p;
    }

    pub fn update_node(&mut self, p: *mut Node<K, V>) {
        if p != self.tail {
            self.remove_node(p);
            self.push_back(p);
        }
    }
}

#[test]
fn test_146() {
    let mut lru = LRUCache::new(10);
    lru.put(10, 13);
    lru.put(3, 17);
    lru.put(6, 11);
    lru.put(10, 5);
    lru.put(9, 10);
    assert_eq!(-1, lru.get(13));
    lru.put(2, 19);
    assert_eq!(19, lru.get(2));
    assert_eq!(17, lru.get(3));
    lru.put(5, 25);
    assert_eq!(-1, lru.get(8));
    lru.put(9, 22);
    lru.put(5, 5);
    lru.put(1, 30);
    assert_eq!(-1, lru.get(11));
    lru.put(9, 12);
    assert_eq!(-1, lru.get(7));
    assert_eq!(5, lru.get(5));
    assert_eq!(-1, lru.get(8));
    assert_eq!(12, lru.get(9));
    lru.put(4, 30);
    lru.put(9, 3);
    assert_eq!(3, lru.get(9));
    assert_eq!(5, lru.get(10));
    assert_eq!(5, lru.get(10));
    lru.put(6, 14);
    lru.put(3, 1);
    assert_eq!(1, lru.get(3));
    lru.put(10, 11);
    assert_eq!(-1, lru.get(8));
    lru.put(2, 14);
    assert_eq!(30, lru.get(1));
    assert_eq!(5, lru.get(5));
    assert_eq!(30, lru.get(4));
    lru.put(11, 4);
    lru.put(12, 24);
    lru.put(5, 18);
    assert_eq!(-1, lru.get(13));
    lru.put(7, 23);
    assert_eq!(-1, lru.get(8));
    assert_eq!(24, lru.get(12));
    lru.put(3, 27);
    lru.put(2, 12);
    assert_eq!(18, lru.get(5));
    lru.put(2, 9);
    lru.put(13, 4);
    lru.put(8, 18);
    lru.put(1, 7);
    assert_eq!(-1, lru.get(6));
    lru.put(9, 29);
    lru.put(8, 21);
    assert_eq!(18, lru.get(5));
    lru.put(6, 30);
    lru.put(1, 12);
    assert_eq!(-1, lru.get(10));
    lru.put(4, 15);
    lru.put(7, 22);
    lru.put(11, 26);
    lru.put(8, 17);
    lru.put(9, 29);
    assert_eq!(18, lru.get(5));
    lru.put(3, 4);
    lru.put(11, 30);
    assert_eq!(-1, lru.get(12));
    lru.put(4, 29);
    assert_eq!(4, lru.get(3));
    assert_eq!(29, lru.get(9));
    assert_eq!(30, lru.get(6));
    lru.put(3, 4);
    assert_eq!(12, lru.get(1));
    assert_eq!(-1, lru.get(10));
    lru.put(3, 29);
    lru.put(10, 28);
    lru.put(1, 20);
    lru.put(11, 13);
    assert_eq!(29, lru.get(3));
    lru.put(3, 12);
    lru.put(3, 8);
    lru.put(10, 9);
    lru.put(3, 26);
    assert_eq!(17, lru.get(8));
    assert_eq!(22, lru.get(7));
    assert_eq!(18, lru.get(5));
    lru.put(13, 17);
    lru.put(2, 27);
    lru.put(11, 15);
    assert_eq!(-1, lru.get(12));
    lru.put(9, 19);
    lru.put(2, 15);
    lru.put(3, 16);
    assert_eq!(20, lru.get(1));
    lru.put(12, 17);
    lru.put(9, 1);
    lru.put(6, 19);
    assert_eq!(-1, lru.get(4));
    assert_eq!(18, lru.get(5));
    assert_eq!(18, lru.get(5));
    lru.put(8, 1);
    lru.put(11, 7);
    lru.put(5, 2);
    lru.put(9, 28);
    assert_eq!(20, lru.get(1));
    lru.put(2, 2);
    lru.put(7, 4);
    lru.put(4, 22);
    lru.put(7, 24);
    lru.put(9, 26);
    lru.put(13, 28);
    lru.put(11, 26);
}
