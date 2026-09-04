#![allow(dead_code, non_snake_case, unused_variables, unused_assignments, unused_mut, unused_parens, unused_imports)]
#[derive(Clone)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        Self { val, next: None }
    }

    pub fn getValue(&self) -> i32 {
        self.val
    }
}
