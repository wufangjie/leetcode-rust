// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
	let mut stack1 = vec![];
	let mut stack2 = vec![];
	while let Some(mut node) = l1 {
	    l1 = node.next.take();
	    stack1.push(node);
	}
	while let Some(mut node) = l2 {
	    l2 = node.next.take();
	    stack2.push(node);
	}
	if stack1.len() < stack2.len() {
	    std::mem::swap(&mut stack1, &mut stack2);
	}

	let mut res = None;
	let mut carry = 0;
	while let Some(mut node1) = stack1.pop() {
	    if let Some(node2) = stack2.pop() {
		node1.val += node2.val + carry;
	    } else {
		node1.val += carry;
            }
	    carry = node1.val / 10;
	    node1.val %= 10;
	    node1.next = res;
	    res = Some(node1);
	}
	res
    }
}
