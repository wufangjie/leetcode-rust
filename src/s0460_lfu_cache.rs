use std::collections::HashMap;
use std::ptr;

/// NOTE: lfu (至少这道题的是这样) 会有固化的风险,
/// 比如有 n - 1 已经很高, 那么新的多条数据最近频繁访问,
/// 会因为相会争夺最后一个名额而频繁地相互覆盖, 导致没有数据真正写入 lfu cache

#[derive(Debug)]
struct LFUCache {
    data: HashMap<i32, Box<Node<i32, i32>>>,
    freq: HashMap<usize, Box<LinkedListFreq<i32, i32>>>,
    cache: LinkedList<i32, i32>,
    capacity: usize,
    count: usize,
}

impl LFUCache {
    fn new(capacity: i32) -> Self {
        let capacity = capacity as usize;
        let data = HashMap::with_capacity(capacity); //new();
        let freq = HashMap::with_capacity(capacity); //new();
        let cache = LinkedList::<i32, i32>::new();
        Self {
            data,
            freq,
            cache,
            capacity,
            count: 0,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.data.get(&key) {
            let p = node.as_mut_ptr();
            let freq = node.freq;
            let lp = self.freq.get_mut(&freq).unwrap().as_mut_ptr();
            match self.cache.update(p, lp) {
                Action::DoNothing => (),
                Action::Remove => {
                    self.freq.remove(&freq);
                }
                Action::Raise => {
                    let node = self.freq.remove(&freq).unwrap();
                    self.freq.insert(freq + 1, node);
                }
                Action::Insert(new) => {
                    self.freq.insert(freq + 1, new);
                }
            }
            node.val
        } else {
            -1
        }
    }

    // /// FIXME: 有没有可能把 get 和 update 中的相同的一段提取到这个 udpate 里?
    // fn update(&mut self, node: &Node<i32, i32>) -> i32 {
    // 	let freq = node.freq;
    // 	let p = node.as_mut_ptr();
    // 	let lp = self.freq.get_mut(&freq).unwrap().as_mut_ptr();
    // 	match self.cache.update(p, lp) {
    //         Action::DoNothing => (),
    //         Action::Remove => {
    // 		self.freq.remove(&freq);
    //         }
    //         Action::Raise => {
    // 		let old = self.freq.remove(&freq).unwrap();
    // 		self.freq.insert(freq + 1, old);
    //         }
    //         Action::Insert(new) => {
    // 		self.freq.insert(freq + 1, new);
    //         }
    // 	}
    // 	node.val
    // }

    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        if let Some(node) = self.data.get_mut(&key) {
            node.val = value; // NOTE: 唯一不同于 get 的操作
            let freq = node.freq;
            let p = node.as_mut_ptr();
            let lp = self.freq.get_mut(&freq).unwrap().as_mut_ptr();
            match self.cache.update(p, lp) {
                Action::DoNothing => (),
                Action::Remove => {
                    self.freq.remove(&freq);
                }
                Action::Raise => {
                    let node = self.freq.remove(&freq).unwrap();
                    self.freq.insert(freq + 1, node);
                }
                Action::Insert(new) => {
                    self.freq.insert(freq + 1, new);
                }
            }
        } else {
            if self.capacity == self.count {
                let (k, f) = self.cache.remove_lfu();
                self.data.remove(k);
                if f > 0 {
                    self.freq.remove(&f);
                }
            } else {
                self.count += 1;
            }

            let new = Box::new(Node::new(key, value));
            let p = new.as_mut_ptr();
            self.data.insert(key, new);

            if let Some(fnode) = self.freq.get_mut(&1) {
                fnode.push_back(p);
            } else {
                self.freq.insert(1, self.cache.add_freq_one(p));
            }
        }
    }
}

#[derive(Debug)]
pub struct Node<K, V> {
    key: K, // for fast pop from hashmap
    val: V,
    freq: usize,
    prev: *mut Node<K, V>,
    next: *mut Node<K, V>,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, val: V) -> Self {
        Self {
            key,
            val,
            freq: 1,
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }
    }

    pub fn as_mut_ptr(&self) -> *mut Self {
        self as *const Self as *mut Self
    }
}

pub enum Action<K, V> {
    Remove,
    Raise,
    Insert(Box<LinkedListFreq<K, V>>),
    DoNothing,
}

#[derive(Debug)]
pub struct LinkedList<K, V> {
    head: *mut LinkedListFreq<K, V>,
    tail: *mut LinkedListFreq<K, V>,
}

impl<K, V> LinkedList<K, V> {
    pub fn new() -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
        }
    }

    fn try_insert_into_next(
        &mut self,
        p: *mut Node<K, V>,
        lp: *mut LinkedListFreq<K, V>,
        curr: &mut LinkedListFreq<K, V>,
    ) -> bool {
        if lp == self.tail {
            false
        } else {
            let next = unsafe { &mut *curr.next };
            if next.freq == curr.freq + 1 {
                next.push_back(p);
                true
            } else {
                false
            }
        }
    }

    pub fn update(&mut self, p: *mut Node<K, V>, lp: *mut LinkedListFreq<K, V>) -> Action<K, V> {
        let mut curr = unsafe { &mut *lp };
        unsafe { (*p).freq += 1 };

        if curr.is_one_element() {
            if self.try_insert_into_next(p, lp, curr) {
                self.remove_freq(lp);
                Action::Remove
            } else {
                curr.freq += 1;
                Action::Raise
            }
        } else {
            curr.remove_node(p);
            if self.try_insert_into_next(p, lp, curr) {
                Action::DoNothing
            } else {
                let mut new = self.add_freq_after(lp);
                new.push_back(p);
                Action::Insert(new)
            }
        }
    }

    pub fn add_freq_one(&mut self, p: *mut Node<K, V>) -> Box<LinkedListFreq<K, V>> {
        // NOTE: need to store returned value
        // assert freq = 1 not exists
        let mut new = Box::new(LinkedListFreq::<K, V>::new(1));
        let lp = new.as_mut_ptr();
        if self.tail.is_null() {
            self.tail = lp; // here new.next == 0
        } else {
            new.next = self.head;
            unsafe {
                (*self.head).prev = lp;
            }
        }
        self.head = lp;
        new.push_back(p);
        new
    }

    pub fn add_freq_after(&mut self, lp: *mut LinkedListFreq<K, V>) -> Box<LinkedListFreq<K, V>> {
        // NOTE: need to store returned value
        let fnode = unsafe { &mut *lp };
        let mut new = Box::new(LinkedListFreq::<K, V>::new(fnode.freq + 1));
        let lp_new = new.as_mut_ptr();
        new.prev = lp;
        if lp == self.tail {
            self.tail = lp_new;
        } else {
            new.next = fnode.next;
            unsafe {
                (*fnode.next).prev = lp_new;
            }
        }
        fnode.next = lp_new;
        new
    }

    pub fn remove_freq(&mut self, lp: *mut LinkedListFreq<K, V>) {
        let fnode = unsafe { &mut *lp };

        if self.head == self.tail {
            // never happen
            self.head = ptr::null_mut();
            self.tail = ptr::null_mut();
        } else if lp == self.head {
            self.head = fnode.next;
        } else if lp == self.tail {
            // never happen
            self.tail = fnode.prev;
        } else {
            unsafe {
                (*fnode.prev).next = fnode.next;
                (*fnode.next).prev = fnode.prev;
            }
        }
    }

    pub fn remove_lfu(&mut self) -> (&K, usize) {
        let fnode = unsafe { &mut *self.head };
        let mut freq_to_pop = fnode.freq;
        if fnode.is_one_element() {
            self.remove_freq(self.head);
        } else {
            freq_to_pop = 0;
        }
        (fnode.remove_head(), freq_to_pop)
    }
}

/// (same frequent's) node-head (head / tail)
/// head / tail point to those nodes
/// prev / next point to diffenent frequent's node-head
/// 事实上, prev 是可以省的, 需要删除的情况把后一个节点的数据移到此节点, 删后一个
/// 这样就可以不用向前的指针, 重新串起链表 (但会加大复杂度)
#[derive(Debug)]
pub struct LinkedListFreq<K, V> {
    head: *mut Node<K, V>,
    tail: *mut Node<K, V>,
    prev: *mut Self,
    next: *mut Self,
    freq: usize,
    len: usize, // it's faster than pointer==
}

impl<K, V> LinkedListFreq<K, V> {
    pub fn new(freq: usize) -> Self {
        Self {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            freq,
            len: 0,
        }
    }

    #[inline]
    pub fn is_one_element(&self) -> bool {
        self.len == 1
    }

    pub fn remove_node(&mut self, p: *mut Node<K, V>) -> &K {
        // NOTE: lfu will never remove the tail (mfu)
        self.len -= 1;
        let node = unsafe { &mut *p };

        if self.len == 0 {
            //self.head == self.tail {
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
        self.len += 1;
        if self.len == 1 {
            //self.head.is_null() {
            self.head = p;
        } else {
            unsafe {
                (*self.tail).next = p;
                (*p).prev = self.tail;
            }
        }
        self.tail = p;
    }

    pub fn as_mut_ptr(&self) -> *mut Self {
        self as *const Self as *mut Self
    }
}

#[test]
fn test_460() {
    let mut lfu = LFUCache::new(2);
    lfu.put(1, 1);
    lfu.put(2, 2);
    assert_eq!(1, lfu.get(1));
    lfu.put(3, 3);
    assert_eq!(-1, lfu.get(2));
    assert_eq!(3, lfu.get(3));
    lfu.put(4, 4);
    assert_eq!(-1, lfu.get(1));
    assert_eq!(3, lfu.get(3));
    assert_eq!(4, lfu.get(4));
}

#[test]
fn test_460_2() {
    let mut lfu = LFUCache::new(10);
    lfu.put(10, 13);

    lfu.put(3, 17);
    lfu.put(6, 11);
    lfu.put(10, 5);
    lfu.put(9, 10);
    assert_eq!(-1, lfu.get(13));
    lfu.put(2, 19);
    assert_eq!(19, lfu.get(2));
    assert_eq!(17, lfu.get(3));
    lfu.put(5, 25);
    assert_eq!(-1, lfu.get(8));
    lfu.put(9, 22);
    lfu.put(5, 5);
    lfu.put(1, 30);
    assert_eq!(-1, lfu.get(11));
    lfu.put(9, 12);
    assert_eq!(-1, lfu.get(7));
    assert_eq!(5, lfu.get(5));
    assert_eq!(-1, lfu.get(8));
    assert_eq!(12, lfu.get(9));

    lfu.put(4, 30);
    lfu.put(9, 3);
    assert_eq!(3, lfu.get(9));
    assert_eq!(5, lfu.get(10));
    assert_eq!(5, lfu.get(10));
    lfu.put(6, 14);
    lfu.put(3, 1);
    assert_eq!(1, lfu.get(3));
    lfu.put(10, 11);
    assert_eq!(-1, lfu.get(8));
    lfu.put(2, 14);
    assert_eq!(30, lfu.get(1));
    assert_eq!(5, lfu.get(5));
    assert_eq!(30, lfu.get(4));
    lfu.put(11, 4);
    lfu.put(12, 24);
    lfu.put(5, 18);

    assert_eq!(-1, lfu.get(13));
    lfu.put(7, 23);
    assert_eq!(-1, lfu.get(8));
    assert_eq!(24, lfu.get(12));
    lfu.put(3, 27);
    lfu.put(2, 12);
    assert_eq!(18, lfu.get(5));
    lfu.put(2, 9);
    lfu.put(13, 4);
    lfu.put(8, 18);
    lfu.put(1, 7);
    assert_eq!(14, lfu.get(6));
    lfu.put(9, 29);
    lfu.put(8, 21);
    assert_eq!(18, lfu.get(5));
    lfu.put(6, 30);
    lfu.put(1, 12);
    //dbg!(lfu.cache.len() == lfu.freq.len());

    assert_eq!(11, lfu.get(10));
    //dbg!(&lfu
    // .freq
    // .values()
    // .map(|f| (f.prev == f.next) as usize)
    // .sum::<usize>());

    // //dbg!(&lfu);
    lfu.put(4, 15);
    // //dbg!(&lfu);
    //dbg!(&lfu
    // .freq
    // .values()
    // .map(|f| (f.prev == f.next) as usize)
    // .sum::<usize>());

    lfu.put(7, 22);
    lfu.put(11, 26);

    lfu.put(8, 17);
    lfu.put(9, 29);

    assert_eq!(18, lfu.get(5));
    lfu.put(3, 4);
    lfu.put(11, 30);
    assert_eq!(-1, lfu.get(12));
    //dbg!(lfu.cache.len() == lfu.freq.len());
    ////dbg!(&lfu);
    lfu.put(4, 29);
    //dbg!(lfu.cache.len() == lfu.freq.len());
    ////dbg!(&lfu);

    assert_eq!(4, lfu.get(3));
    assert_eq!(29, lfu.get(9));

    assert_eq!(30, lfu.get(6));
    lfu.put(3, 4);
    assert_eq!(12, lfu.get(1));

    assert_eq!(11, lfu.get(10));

    lfu.put(3, 29);
    lfu.put(10, 28);
    lfu.put(1, 20);
    lfu.put(11, 13);

    assert_eq!(29, lfu.get(3));
    lfu.put(3, 12);
    lfu.put(3, 8);
    lfu.put(10, 9);
    lfu.put(3, 26);
    assert_eq!(17, lfu.get(8));
    assert_eq!(-1, lfu.get(7));
    assert_eq!(18, lfu.get(5));
    lfu.put(13, 17);
    lfu.put(2, 27);
    lfu.put(11, 15);
    assert_eq!(-1, lfu.get(12));
    lfu.put(9, 19);
    lfu.put(2, 15);
    lfu.put(3, 16);
    assert_eq!(20, lfu.get(1));
    lfu.put(12, 17);
    lfu.put(9, 1);
    lfu.put(6, 19);
    assert_eq!(29, lfu.get(4));
    assert_eq!(18, lfu.get(5));
    assert_eq!(18, lfu.get(5));
    lfu.put(8, 1);
    lfu.put(11, 7);
    lfu.put(5, 2);
    lfu.put(9, 28);
    assert_eq!(20, lfu.get(1));
    lfu.put(2, 2);
    lfu.put(7, 4);
    lfu.put(4, 22);
    lfu.put(7, 24);
    lfu.put(9, 26);
    lfu.put(13, 28);
    lfu.put(11, 26);
}
