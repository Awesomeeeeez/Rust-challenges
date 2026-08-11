// https://leetcode.com/problems/time-based-key-value-store/description/?envType=problem-list-v2&envId=design
// Лучше чем 96% пользователей по Runtime
// Лучше чем 92% пользователей по Memory

use std::{collections::HashMap, println};

struct TimeMap {
    time_map: HashMap<String, Vec<(i32, String)>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {

    fn new() -> Self {
        Self {
            time_map: HashMap::new(),
        }
    }
    
    fn set(&mut self, key: String, value: String, timestamp: i32) {
        self.time_map.entry(key).or_insert(Vec::new()).push((timestamp, value));
    }
    
    fn get(&self, key: String, timestamp: i32) -> String {
        let pos = self.time_map.get(&key);
        if pos == None || timestamp < pos.unwrap()[0].0 {
            return "".to_string();
        } else if pos.unwrap().last().unwrap().0 < timestamp {
            return pos.unwrap().last().unwrap().1.clone();
        } else {
            let pair = pos.unwrap();
            for i in 0..pair.len() {
                if pair[i].0 == timestamp {
                    return pair[i].1.clone();
                } else {
                    if pair[i].0 > timestamp {
                        return pair[i-1].1.clone();
                    }
                }
            }
        }
        "".to_string()
    }
}

fn main() {
    let mut timeMap = TimeMap::new();
    // timeMap.set("foo".to_string(), "bar".to_string(), 1);  // сохраняем ключ "foo" и значение "bar" во временную метку timestamp = 1.
    // println!("{}", timeMap.get("foo".to_string(), 1));         // возвращаем "bar"
    // println!("{}", timeMap.get("foo".to_string(), 3));         // возвращаем "bar", так как нет значения по ключу foo во временной метке 3 и мы ищем самое свежее значение, то есть проверяем сначала во временной метке 2 - в ней тоже нет такого ключа, затем проверяем во временной метке 1 и находим значение "bar".
    // timeMap.set("foo".to_string(), "bar2".to_string(), 4); // сохраняем ключ "foo" и значение "bar2" во временную метку timestamp = 4.
    // println!("{}", timeMap.get("foo".to_string(), 4));         // возвращаем "bar2"
    // println!("{}", timeMap.get("foo".to_string(), 5));         // возвращаем "bar2"

    timeMap.set("foo".to_string(), "bar".to_string(), 1);    // в момент 1: foo = bar
    timeMap.set("foo".to_string(), "bar2".to_string(), 4);   // в момент 4: foo = bar2

    println!("{}", timeMap.get("foo".to_string(), 1));   //→ "bar"   // на момент 1: было именно это значение
    println!("{}", timeMap.get("foo".to_string(), 3));   //→ "bar"   // в момент 3 ещё не наступило время 4, поэтому берём последнее, что было ДО 3 — это "bar" из момента 1
    println!("{}", timeMap.get("foo".to_string(), 4));   //→ "bar2"  // ровно момент 4
    println!("{}", timeMap.get("foo".to_string(), 5));  //→ "bar2"  // после момента 4, самое свежее — bar2
    println!("{}", timeMap.get("foo".to_string(), 0));  //→ ""      // момент 0 раньше ЛЮБОЙ записи — ничего не найдено, вернуть пустую строку
}