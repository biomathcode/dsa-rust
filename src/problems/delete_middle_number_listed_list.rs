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

pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    head.as_ref()?;
    let mut n = 1;
    let mut ptr = head.as_ref().unwrap().as_ref();
    while let Some(node) = ptr.next.as_ref() {
        ptr = node.as_ref();
        n += 1;
    }
    if n == 1 {
        return None;
    }
    let mut ptr = head.as_mut().unwrap().as_mut();
    for _ in 0..n / 2 - 1 {
        ptr = ptr.next.as_mut().unwrap().as_mut();
    }
    ptr.next = ptr.next.as_mut().unwrap().as_mut().next.take();
    head
}
