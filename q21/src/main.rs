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
        if pt.is_some(){
            while pt.as_mut().unwrap().next.is_some(){
                pt=&mut pt.as_mut().unwrap().next;
            }
            pt.as_mut().unwrap().next=Some(Box::new(next));
        }else{
            self.next=Some(Box::new(next));
        }
    }
}

pub fn append_helper(list: &mut Option<Box<ListNode>>, next: ListNode){
    let mut pt = list;
    while pt.as_mut().unwrap().next.is_some(){
        pt=&mut pt.as_mut().unwrap().next;
    }
    pt.as_mut().unwrap().next=Some(Box::new(next));
}

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut pt1 = &list1;
    let mut pt2 = &list2;
    let mut new_list = Some(Box::new(ListNode::new(0)));

    while pt1.is_some() && pt2.is_some(){
        if pt1.as_ref().unwrap().val<pt2.as_ref().unwrap().val{
            let new_node = ListNode::new(pt1.as_ref().unwrap().val);
            append_helper(&mut new_list, new_node);
            pt1=&pt1.as_ref().unwrap().next;
        }else{
            let new_node = ListNode::new(pt2.as_ref().unwrap().val);
            append_helper(&mut new_list, new_node);
            pt2=&pt2.as_ref().unwrap().next;
        }
    }

    match(pt1,pt2){
        (Some(p1),None)=>{
            append_helper(&mut new_list, *p1.clone())
        },
        (None,Some(p2))=>{
            append_helper(&mut new_list, *p2.clone())
        },
        _=>{}
    };
    new_list.unwrap().next
}

fn main() {
    let mut buf1 = String::new();
    let mut buf2 = String::new();
    stdin().read_line(&mut buf1).expect("can't read line");
    stdin().read_line(&mut buf2).expect("can't read line");

    let vec1: Vec<i32> = buf1
        .trim()
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    let vec2: Vec<i32> = buf2
        .trim()
        .split(" ")
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut llst1 = ListNode::new(vec1[0]);
    let mut llst2 = ListNode::new(vec2[0]);

    for (i1,i2) in vec1.iter().skip(1).zip(vec2.iter().skip(1)){
        let tnode1 = ListNode::new(*i1);
        let tnode2 = ListNode::new(*i2);
        llst1.append(tnode1);
        llst2.append(tnode2);
    }

    merge_two_lists(Some(Box::new(llst1)), Some(Box::new(llst2)));
}
