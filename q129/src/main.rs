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
                let new_node: Node = Some(Rc::new(RefCell::new(TreeNode::new(v))));
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

    pub fn find_node_rc(root: &Node, val: i32) -> Node {
        if root.is_none() {
            return None;
        }
        let rt = root.as_ref().unwrap();

        if rt.borrow().val == val {
            return Some(Rc::clone(rt));
        }

        let left_child_rc: Node = Self::find_node_rc(&rt.borrow().left, val);
        let right_child_rc: Node = Self::find_node_rc(&rt.borrow().right, val);
        match (left_child_rc, right_child_rc) {
            (None, None) => None,
            (None, Some(rrc)) => Some(rrc),
            (Some(lrc), _) => Some(lrc),
        }
    }
}

fn pre_order_sum(node: &Node) -> Vec<String> {
    if node.is_none() {
        return Vec::new();
    };
    let node_ref = node.as_ref().unwrap();
    let mut left_nums = pre_order_sum(&node_ref.borrow().left);
    let mut right_nums = pre_order_sum(&node_ref.borrow().right);
    left_nums.append(&mut right_nums);
    let mut nums = left_nums;

    if nums.is_empty() {
        nums.push(node_ref.borrow().val.to_string());
        return nums;
    }

    for s in &mut nums {
        s.insert_str(0, &node_ref.borrow().val.to_string());
    }
    return nums;
}

fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let nums = pre_order_sum(&root);
    //println!("{nums:#?}");

    let mut sum = 0;
    for i in &nums {
        sum = sum + i.parse::<i32>().unwrap();
    }
    return sum;
}

fn main() {
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("can't readline");

    let nums: Vec<&str> = buf.trim().split(",").collect();
    let tree: Node = TreeNode::from(nums);

    sum_numbers(tree);
}
