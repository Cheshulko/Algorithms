// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;

struct Solution {}

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        root.map(|root| {
            let root = root.borrow();
            let mut v = vec![];
            v.extend(Solution::inorder_traversal(root.left.clone()).into_iter());
            v.push(root.val);
            v.extend(Solution::inorder_traversal(root.right.clone()).into_iter());
            v
        })
        .unwrap_or(vec![])
    }
}
