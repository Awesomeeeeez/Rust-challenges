// https://leetcode.com/problems/insert-delete-getrandom-o1/description/?envType=problem-list-v2&envId=design

use std::{{collections::HashMap}, println};

use rand::Rng;

struct RandomizedSet {
    set: Vec<i32>,
    hash_map: HashMap<i32, usize>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {

    fn new() -> Self {
        Self {
            set: Vec::new(),
            hash_map: HashMap::new(),
        }
    }
    
    fn insert(&mut self, val: i32) -> bool {
        match self.hash_map.get(&val) {
            Some(_) => false,
            None => {
                self.set.push(val);
                self.hash_map.insert(val, self.set.len() - 1);
                true
            },
        }
    }
    
    fn remove(&mut self, val: i32) -> bool {
        let end_idx = self.set.len() - 1;

        if let Some(v) = self.hash_map.get(&val) {
            self.hash_map.insert(self.set[end_idx], *v);
        }

        match self.hash_map.get(&val) {
            Some(v) => {
                self.set.swap(*v, end_idx);
                self.set.pop();
                self.hash_map.remove(&val);
                true
            },
            None => false,
        }
    }
    
    fn get_random(&self) -> i32 {
        let idx = rand::thread_rng().gen_range(0..self.set.len());
        self.set[idx]
    }
}

fn main() {
    let mut set = RandomizedSet::new();
    my_print(&set);
    println!("{}", set.insert(1)); 
    my_print(&set);
    println!("{}", set.remove(2)); // false
    println!("{}", set.insert(2)); // true
    my_print(&set);
    println!("{}", set.get_random());
    println!("{}", set.remove(1));
    my_print(&set);
    println!("{}", set.insert(2));
    my_print(&set);
    println!("{}", set.get_random());
}

fn my_print(set: &RandomizedSet) {
    println!("{:?}", set.set);
    for (k, v) in &set.hash_map {
        println!("{k}: {v}");
    }
}
