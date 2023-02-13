use std::io::stdin;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn append(&mut self, next: ListNode) {
        let mut pt = &mut self.next;
        if pt.is_some() {
            while pt.as_mut().unwrap().next.is_some() {
                pt = &mut pt.as_mut().unwrap().next;
            }
            pt.as_mut().unwrap().next = Some(Box::new(next));
        } else {
            self.next = Some(Box::new(next));
        }
    }
}

pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    let mut dummy_head = ListNode::new(0);
    let mut pt = &mut dummy_head;
    while let Some(mut node) = head {
        head = std::mem::replace(&mut node.next, None);
        if node.val != val {
            pt.next = Some(node);
            pt = pt.next.as_mut().unwrap();
        }
    }
    dummy_head.next
}

fn main() {
    let mut buf1 = String::new();
    let mut buf2 = String::new();
    stdin().read_line(&mut buf1).expect("can't read line");
    stdin().read_line(&mut buf2).expect("can't read line");
    buf2 = buf2.trim().to_string();

    let vec1: Vec<i32> = buf1
        .trim()
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut llst1 = ListNode::new(vec1[0]);
    let val: i32 = buf2.parse().unwrap();

    for i1 in vec1.iter().skip(1) {
        let tnode1 = ListNode::new(*i1);
        llst1.append(tnode1);
    }

    remove_elements(Some(Box::new(llst1)), val);
}
