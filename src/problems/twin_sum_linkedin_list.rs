// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LinkedList {
    next: Option<Box<ListNode>>,
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

impl LinkedList {
    pub fn new() -> Self {
        LinkedList { next: None }
    }
    pub fn push(&mut self, val: i32) {
        let new_node = Box::new(ListNode {
            val,
            next: self.next.take(),
        });

        self.next = Some(new_node);
    }
    pub fn pair_sum(&self) -> i32 {
        let mut max_twin_sum: i32 = 0;
        let mut num_list: Vec<i32> = vec![];

        let mut temp_node: Option<Box<ListNode>> = self.next.clone();

        while temp_node.is_some() {
            // while let Some(node) = temp_node{
            num_list.push(temp_node.as_ref().unwrap().val);
            // num_list.push(node.val);
            temp_node = temp_node.unwrap().next;
            // temp_node = &node.next;
        }

        let mut left = 0;

        let mut right = num_list.len() - 1;

        while left < right {
            let twin_sum: i32 = num_list[left] + num_list[right];
            if max_twin_sum < twin_sum {
                max_twin_sum = twin_sum;
            }
            left += 1;
            right -= 1;
            println!("{:?}", num_list);
        }
        max_twin_sum
    }
}

#[cfg(test)]
mod test_super {
    use super::LinkedList;

    #[test]
    fn test_pair_sum() {
        let next = [4, 2, 2, 3];

        let mut lt = LinkedList::new();

        for i in next {
            lt.push(i);
        }

        println!("{:?}", lt);
        assert_eq!(lt.pair_sum(), 7)
    }
}
