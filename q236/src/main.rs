use std::cell::RefCell;
use std::collections::VecDeque;
use std::io::stdin;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

type Node = Option<Rc<RefCell<TreeNode>>>;

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }

    pub fn add_left(&mut self, node: Node) {
        self.left = node;
    }

    pub fn add_right(&mut self, node: Node) {
        self.right = node;
    }
}

fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let root = match root {
        Some(a) => a,
        None => return None,
    };
    let v1 = p.unwrap().borrow().val;
    let v2 = q.unwrap().borrow().val;
    helper(root, v1, v2)
}

fn helper(root: Rc<RefCell<TreeNode>>, v1: i32, v2: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let root_v = root.borrow().val;
    if root_v == v1 || root_v == v2 {
        return Some(root);
    }
    let l = root
        .borrow()
        .left
        .as_ref()
        .and_then(|a| helper(a.clone(), v1, v2));
    let r = root
        .borrow()
        .right
        .as_ref()
        .and_then(|a| helper(a.clone(), v1, v2));
    match (l, r) {
        (Some(_), Some(_)) => Some(root),
        (None, None) => None,
        (Some(l), None) => Some(l),
        (None, Some(r)) => Some(r),
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

fn gen_tree(nums: Vec<&str>) -> Node {
    if nums.is_empty() || nums[0] == "null" {
        return None;
    }
    let root = Some(Rc::new(RefCell::new(TreeNode::new(
        nums[0].parse::<i32>().unwrap(),
    ))));

    //hack not sure best way
    let root_rc = Rc::clone(root.as_ref().unwrap());

    let mut q = VecDeque::<Node>::new();
    q.push_front(root);

    for (i, s) in nums.iter().copied().enumerate().skip(1) {
        push_node(&mut q, get_val(s), is_left(i));
    }

    return Some(root_rc);

    // if nums.is_empty(){
    //     return Some(*father.unwrap());
    // }
    // let t_node = Some(Rc::new(RefCell::new(TreeNode::new(nums[0]))));
    // let new_nums=Vec::from(&nums[1..]);
    // let father_ref = father.unwrap();
    // if father_ref.borrow().left == None{
    //     father_ref.borrow_mut().add_left(t_node);
    //     return gen_tree(new_nums, father);
    // }else if father_ref.borrow().left != None && father_ref.borrow().left != None{
    //     return Some(*father.unwrap());
    // }else{
    //     father_ref.borrow_mut().add_right(t_node);
    //     return gen_tree(new_nums, father.unwrap().borrow().left.as_ref());
    // }
}

fn find_node_rc(root: &Node, val: i32) -> Node {
    match root {
        None => None,
        Some(rt) => {
            if rt.borrow().val == val {
                let rt_rc = Rc::clone(rt);
                return Some(rt_rc);
            } else {
                let left_child_rc = find_node_rc(&rt.borrow().left, val);
                let right_child_rc = find_node_rc(&rt.borrow().right, val);
                match left_child_rc {
                    None => match right_child_rc {
                        None => None,
                        Some(rc) => Some(rc),
                    },
                    Some(lc) => Some(lc),
                }
            }
        }
    }
}

fn main() {
    let mut buf = String::new();
    let mut node1_val = String::new();
    let mut node2_val = String::new();
    stdin().read_line(&mut buf).expect("can't readline");
    stdin().read_line(&mut node1_val).unwrap();
    stdin().read_line(&mut node2_val).unwrap();

    let node1_val = node1_val.trim().parse::<i32>().unwrap();
    let node2_val = node2_val.trim().parse::<i32>().unwrap();

    let nums: Vec<&str> = buf.trim().split(",").collect();
    let tree: Node = gen_tree(nums);

    let node1 = find_node_rc(&tree, node1_val);
    let node2 = find_node_rc(&tree, node2_val);

    lowest_common_ancestor(tree, node1, node2);
}
