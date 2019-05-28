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

fn main() {
    println!("Add Two Numbers");
    let list_a_node_1 = ListNode::new(3);
    let mut list_a_node_2 = ListNode::new(4);
    list_a_node_2.next = Some(Box::new(list_a_node_1));
    let mut list_a_node_3 = ListNode::new(2);
    list_a_node_3.next = Some(Box::new(list_a_node_2));


    let list_b_node_1 = ListNode::new(4);
    let mut list_b_node_2 = ListNode::new(6);
    list_b_node_2.next = Some(Box::new(list_b_node_1));
    let mut list_b_node_3 = ListNode::new(5);
    list_b_node_3.next = Some(Box::new(list_b_node_2));

    let result = Solution::add_two_numbers(Some(Box::new(list_a_node_3)), Some(Box::new(list_b_node_3)));
    // println!("{:?}", result);
    assert_eq!("Some(ListNode { val: 7, next: Some(ListNode { val: 0, next: Some(ListNode { val: 8, next: None }) }) })", format!("{:?}", result));


    // called `Result::unwrap()` on an `Err` value: ParseIntError { kind: Overflow } (solution.rs)
    // [3,9,9,9,9,9,9,9,9,9]
    // [7]
}

struct Solution {}

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut node1 = l1.unwrap();
        let mut node2 = l2.unwrap();

        let mut first_number = String::from("");
        let mut second_number = String::from("");

        loop {
            let current_number = node1.val;
            first_number = format!("{}{}", first_number.as_str(), current_number.to_string());
            match node1.next {
                Some(next) => node1 = next,
                None => break,
            };
        }

        loop {
            let current_number = node2.val;
            second_number = format!("{}{}", second_number.as_str(), current_number.to_string());
            match node2.next {
                Some(next) => node2 = next,
                None => break,
            };
        }

        let result = first_number.parse::<i32>().unwrap() + second_number.parse::<i32>().unwrap();

        let mut vec = Vec::new();
        for c in result.to_string().chars() {
            let new_node = ListNode::new(c.to_string().parse::<i32>().unwrap());
            vec.push(new_node);
        }

        let mut current_node = ListNode::new(-1);
        let mut count = 0;
        for mut node in vec {
            if count == 0 {
                node.next = None;
            } else {
                node.next = Some(Box::new(current_node));
            }
            current_node = node;
            // println!("{:?}", current_node);
            count += 1;
        
        }

        return Some(Box::new(current_node));;
    }
}