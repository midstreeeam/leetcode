use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::stdin;
use std::rc::Rc;

type Node = Option<Rc<RefCell<TreeNode>>>;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Node,
    pub right: Node,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    fn get_val(s: &str) -> Option<i32> {
        if s == "null" {
            return None;
        }
        return Some(s.parse::<i32>().unwrap());
    }

    fn is_left(i: usize) -> bool {
        if (i + 1) % 2 == 0 {
            return true;
        }
        return false;
    }

    fn push_node(q: &mut VecDeque<Node>, val: Option<i32>, is_left: bool) {
        match val {
            None => {
                if !is_left {
                    q.pop_back();
                }
            }
            Some(v) => {
                let new_node = Some(Rc::new(RefCell::new(TreeNode::new(v))));
                let new_node_rc = Rc::clone(new_node.as_ref().unwrap());
                if is_left {
                    q.back().unwrap().as_ref().unwrap().borrow_mut().left = new_node;
                } else {
                    q.back().unwrap().as_ref().unwrap().borrow_mut().right = new_node;
                    q.pop_back();
                }
                q.push_front(Some(new_node_rc));
            }
        }
    }

    pub fn from(nums: Vec<&str>) -> Node {
        if nums.is_empty() || nums[0] == "null" {
            return None;
        }

        let root = Some(Rc::new(RefCell::new(TreeNode::new(
            nums[0].parse::<i32>().unwrap(),
        ))));

        //hack, not sure best way
        let root_rc = Rc::clone(root.as_ref().unwrap());

        let mut q = VecDeque::<Node>::new();
        q.push_front(root);

        for (i, s) in nums.iter().copied().enumerate().skip(1) {
            Self::push_node(&mut q, Self::get_val(s), Self::is_left(i));
        }
        return Some(root_rc);
    }
}

pub struct Solution {}

impl Solution {
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>)
        -> Vec<Vec<i32>> {
        if root.is_none(){
            return Vec::new()
        };

        let mut left_flag = false;
        let mut v = Vec::<Vec<i32>>::new();
        let mut q = VecDeque::<Node>::new();
        q.push_front(root);

        // loop over each level
        while !q.is_empty(){
            let mut ans_piece = VecDeque::<i32>::new();

            for _ in 0..q.len(){
                let mut t:Node = q.pop_back().unwrap();
                if t.is_none(){continue;}
                if left_flag{
                    ans_piece.push_front(t.as_mut().unwrap().borrow_mut().val);
                }else{
                    ans_piece.push_back(t.as_mut().unwrap().borrow_mut().val);
                }
                

                // push next level
                if t.as_mut().unwrap().borrow_mut().left.is_some(){
                    q.push_front(t.as_mut().unwrap().borrow_mut().left.clone());
                }
                if t.as_mut().unwrap().borrow_mut().right.is_some(){
                    q.push_front(t.as_mut().unwrap().borrow_mut().right.clone());
                }
            }
            left_flag = !left_flag;
            v.push(Vec::from(ans_piece));
        };

        println!("{v:#?}");
        return v
    }
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("can't readline");

    let nums: Vec<&str> = buf.trim().split(",").collect();
    let tree: Node = TreeNode::from(nums);
    Solution::zigzag_level_order(tree);
}
