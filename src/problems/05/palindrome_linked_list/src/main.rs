// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

struct Solution {}

impl Solution {
  pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut stack = vec![];
    let mut node = head.unwrap();

    loop {
      stack.push(node.val);

      if node.next == None {
        break;
      }

      node = node.next.unwrap();
    }

    println!("{:?}", stack);
    return false;      
  }
}

fn main() {
  let list_a_node_1 = ListNode::new(1);
  let mut list_a_node_2 = ListNode::new(2);
  list_a_node_2.next = Some(Box::new(list_a_node_1));
  let mut list_a_node_3 = ListNode::new(1);
  list_a_node_3.next = Some(Box::new(list_a_node_2));

  let result = Solution::is_palindrome(Some(Box::new(list_a_node_3)));
  println!("{}", result);
}
