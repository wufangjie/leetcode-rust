use utils::SegmentTree;

#[derive(Debug)]
struct NumArray {
    tree: SegmentTree<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self {
            tree: SegmentTree::new(nums),
        }
    }

    fn update(&mut self, index: i32, val: i32) {
        self.tree.update(index as usize, val)
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.tree.query(left as usize, right as usize)
    }
}

#[test]
fn test_x() {
    let mut arr = NumArray::new(vec![7, 2, 7, 2, 0]);
    arr.update(4, 6);
    arr.update(0, 2);
    arr.update(0, 9);
    arr.update(3, 8);
    arr.update(4, 1);
}
