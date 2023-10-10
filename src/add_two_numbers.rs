/// # Add Two Numbers
///
/// You are given two non-empty linked lists representing two non-negative integers.
/// The digits are stored in reverse order, and each of their nodes contains a single digit.
/// Add the two numbers and return the sum as a linked list.
///
/// You may assume the two numbers do not contain any leading zero, except the number 0 itself.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new (val: i32) -> Self {
        Self {
            next: None,
            val,
        }
    }

}

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut l1_ptr = &l1;
        let mut l2_ptr = &l2;
        let mut carry = 0;
        let mut head: Option<Box<ListNode>> = None;
        let mut tail = head.as_mut();
        while l1_ptr.is_some() || l2_ptr.is_some() || carry != 0 {
            let l1_value = match l1_ptr {
                Some(n) => {
                    l1_ptr = &n.next;
                    n.val
                },
                None => {
                    0
                },
            };

            let l2_value = match l2_ptr {
                Some(n) => {
                    l2_ptr = &n.next;
                    n.val
                },
                None => {
                    0
                }
            };
            let sum = l1_value + l2_value + carry;
            let modulo = sum % 10;
            carry = sum / 10;

            if tail.is_none() {
                head = Some(Box::new(ListNode::new(modulo)));
                tail = head.as_mut();
            } else {
                tail.as_mut().unwrap().next = Some(Box::new(ListNode::new(modulo)));
                tail = tail.unwrap().next.as_mut();
            }
            
        };
        head
    }
}

#[cfg(test)]
mod add_two_numbers_test {
    use super::*;

    fn generate(values: Vec<i32>) -> Option<Box<ListNode>> {
        let mut rev_values_iter = values.iter().rev();
        let first = rev_values_iter.next()?;
        let mut ll = Box::new(ListNode::new(*first));

        for &val in rev_values_iter {
            ll = Box::new(ListNode {
                val,
                next: Some(ll.clone()),
            });
        }
        Some(ll)
    }

    #[test]
    fn test_a() {
        // Arrange
        let l1 = generate(vec![2, 4, 3]);
        let l2 = generate(vec![5, 6, 4]);
        let answer = generate(vec![7, 0, 8]);
        // Act
        let result = Solution::add_two_numbers(l1, l2);
        // Assert
        assert_eq!(result, answer);
        
    }

}
