// Лучше чем 89% всех пользователей по Runtimne
// Лучше чем 81% всех пользователей по Memory

// Идеальное решение задачи https://leetcode.com/problems/design-hashmap/submissions/2101753964/?envType=problem-list-v2&envId=design

use std::{vec, println};

struct MyHashMap {
    buckets: Vec<Vec<(i32, i32)>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); 1000],
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        for bucket in &mut self.buckets[hash(key)] {
            if bucket.0 == key {
                bucket.1 = value;
                return;
            }
        }
        self.buckets[hash(key)].push((key, value));
    }
    
    fn get(&self, key: i32) -> i32 {
        for bucket in &self.buckets[hash(key)] {
            if bucket.0 == key {
                return bucket.1;
            }
        }
        -1
    }
    
    fn remove(&mut self, key: i32) {
        let pair = &mut self.buckets[hash(key)];
        let mut remove_index: Option<usize> = None;
        for i in 0..pair.len() {
            if pair[i].0 == key {
                remove_index = Some(i);
                break;
            }
        }
        if let Some(i) = remove_index {
            pair.remove(i);
        }
    }
}

fn hash(key: i32) -> usize {
    (key % 1000) as usize
}

fn main() {
    let mut hash_table = MyHashMap::new();
    hash_table.remove(14);
    println!("{}", hash_table.get(2));
}