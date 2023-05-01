// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
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
/// https://leetcode.com/problems/merge-two-sorted-lists/
pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        /// placeholder list. we will return its tail
        let mut new_list : ListNode = ListNode::new(0);
        let mut tail_ptr : &mut ListNode  = &mut new_list;

        loop {
            match (list1, list2) {
                (Some( l1), Some( l2) ) => {
                    let (mut small, big) = if l1.val > l2.val {
                        (l2, l1)
                    } else {
                        (l1, l2)
                    };

                    list1 = small.next.take();
                    list2 = Some(big);
                    tail_ptr.next = Some(small);
                    tail_ptr = tail_ptr.next.as_mut().unwrap();
                }

                (Some(l), None) | (None, Some(l)) => {
                    tail_ptr.next = Some(l);
                    break;
                }
                (None, None) => {
                    break;
                }
            }
        }

        new_list.next

}
