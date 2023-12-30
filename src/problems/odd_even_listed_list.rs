// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val,
        }
    }
}

pub fn odd_even_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut flipflop = true;
    let mut odd = Box::new(ListNode::new(0));
    let mut even = Box::new(ListNode::new(0));

    let mut odd_tail = &mut odd;
    let mut even_tail = &mut even;

    while let Some(mut node) = head {
        head = node.next.take();

        if flipflop {
            odd_tail.next = Some(node);
            odd_tail = odd_tail.next.as_mut()?;
        } else {
            even_tail.next = Some(node);
            even_tail = even_tail.next.as_mut()?;
        }

        flipflop = !flipflop;
    }
    odd_tail.next = even.next;

    odd.next
}
