struct Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn cons(val: i32, node: ListNode) -> Self {
        Self {
            val,
            next: Some(Box::new(node)),
        }
    }
}

impl Solution {
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        if lists.len() == 0 {
            return None;
        }
        let mut res = ListNode::new(0); // dummy head
        let mut n = lists.len();
        while n > 1 {
            let delta = (n + 1) / 2;
            for i in 0..n / 2 {
                // merge pairs cost 5ms
                std::mem::swap(&mut res.next, &mut lists[i]);
                Solution::merge_2(&mut res.next, &mut lists[i + delta]);
            }
            std::mem::swap(&mut res.next, &mut lists[0]);
            n = delta;
        }

        // for mut lst in lists {
        //     Solution::merge_2(&mut res.next, &mut lst);
        // } // merge one by one cost 110ms

        lists[0].take()
    }

    #[inline]
    fn merge_2(mut l1: &mut Option<Box<ListNode>>, l2: &mut Option<Box<ListNode>>) {
        while let Some(v1) = Solution::get_val(l1) {
            if let Some(v2) = Solution::get_val(l2) {
                if v2 < v1 {
                    std::mem::swap(l1, l2);
                }
                l1 = &mut l1.as_mut().unwrap().next;
            } else {
                return; // finished
            }
        }
        if l2.is_some() {
            std::mem::swap(l1, l2);
        }
    }

    #[inline]
    fn get_val(node: &Option<Box<ListNode>>) -> Option<i32> {
        node.as_ref().map(|inner| inner.val)
    }
}

#[test]
fn test_0023() {
    let res = Some(Box::new(ListNode::cons(
        1,
        ListNode::cons(
            1,
            ListNode::cons(
                2,
                ListNode::cons(
                    3,
                    ListNode::cons(4, ListNode::cons(4, ListNode::cons(5, ListNode::new(6)))),
                ),
            ),
        ),
    )));
    assert_eq!(
        res,
        Solution::merge_k_lists(vec![
            Some(Box::new(ListNode::cons(
                1,
                ListNode::cons(4, ListNode::new(5)),
            ))),
            Some(Box::new(ListNode::cons(
                1,
                ListNode::cons(3, ListNode::new(4)),
            ))),
            Some(Box::new(ListNode::cons(2, ListNode::new(6)))),
        ])
    );

    assert_eq!(None, Solution::merge_k_lists(vec![]));
    assert_eq!(None, Solution::merge_k_lists(vec![None]));
}
