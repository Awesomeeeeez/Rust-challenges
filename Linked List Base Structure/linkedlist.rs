#![allow(dead_code, non_snake_case, unused_variables, unused_assignments, unused_mut, unused_parens, unused_imports)]

use crate::node::ListNode;

pub struct LinkedList {
    pub head: Option<Box<ListNode>>,
    pub count: i32,
}

pub fn print_list(head: &Option<Box<ListNode>>) {
    let mut rf = head;
    let mut result = Vec::new();

    while let Some(node) = rf {
        result.push(node.val);
        rf = &node.next;
    }

    println!("{:?}", result);
}

impl LinkedList {
    pub fn new() -> LinkedList {
        LinkedList { head: None, count: 0 }
    }

    pub fn addFirst(&mut self, value: i32) {
        let mut node = Box::new(ListNode::new(value));
        node.next = self.head.take();
        self.head = Some(node);
        self.count += 1;
    }

    pub fn addLast(&mut self, value: i32) {
        let new_node = Box::new(ListNode::new(value));
        if self.head.is_none() {
            self.head = Some(new_node);
        } else {
            let mut cur = self.head.as_mut().unwrap();
            while cur.next.is_some() {
                cur = cur.next.as_mut().unwrap();
            }
            cur.next = Some(new_node);
        }
        self.count += 1;
    }

    pub fn get(&self, index: i32) -> i32 {
        let mut cur = self.head.as_ref();
        let mut i = 0;
        while let Some(node) = cur {
            if i == index { return node.val; }
            cur = node.next.as_ref();
            i += 1;
        }
        -1
    }

    pub fn remove(&mut self, index: i32) {
        if self.head.is_none() || index < 0 { return; }
        if index == 0 {
            let mut old = self.head.take().unwrap();
            self.head = old.next.take();
            self.count -= 1;
            return;
        }
        let mut prev = self.head.as_mut().unwrap();
        let mut j = 0;
        while j < index - 1 {
            if prev.next.is_none() { return; }
            prev = prev.next.as_mut().unwrap();
            j += 1;
        }
        if let Some(mut to_remove) = prev.next.take() {
            prev.next = to_remove.next.take();
            self.count -= 1;
        }
    }

    pub fn size(&self) -> i32 {
        self.count
    }
}
