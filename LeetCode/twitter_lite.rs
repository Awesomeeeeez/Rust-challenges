// LeetCode: https://leetcode.com/problems/design-twitter/description/?envType=problem-list-v2&envId=design


use std::{collections::{HashMap, HashSet, BTreeSet}, println};

struct Twitter {
    clock: u64,
    twits: HashMap<i32, BTreeSet<(u64, i32)>>,
    follows: HashMap<i32, HashSet<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Twitter {

    fn new() -> Self {
        Self {
            clock: 0,
            twits: HashMap::new(),
            follows: HashMap::new(),
        }
    }
    
    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        self.twits.entry(user_id).or_insert(BTreeSet::new()).insert((self.clock, tweet_id));
        self.follows.entry(user_id).or_insert(HashSet::new()).insert(user_id);
        self.clock += 1;
    }
    
    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        let mut list = BTreeSet::new();
        let mut result = Vec::new();
        if let Some(v) = self.follows.get(&user_id) {
            for follow in v {
                if let Some(posts) = self.twits.get(follow) {
                    for &i in posts {
                        list.insert(i);
                    }
                }
            }
        }
        for post in &list {
            result.push(post.1);
        }
        result.reverse();
        result[..10.min(result.len())].to_vec()
    }
    
    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        self.follows.entry(follower_id).or_insert(HashSet::new()).insert(followee_id);
    }
    
    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(v) = self.follows.get_mut(&follower_id) {
            (*v).remove(&followee_id);
        }
    }
}

fn main() {
    let mut twitter = Twitter::new();
    twitter.post_tweet(1, 5);
    println!("{:?}", twitter.get_news_feed(1));
    twitter.follow(1, 2);
    twitter.post_tweet(2, 6);
    println!("{:?}", twitter.get_news_feed(1));
    twitter.unfollow(1, 2);
    println!("{:?}", twitter.get_news_feed(1));

    println!("{:?}", twitter.twits);
    println!("{:?}", twitter.follows);
    twitter.get_news_feed(2);
}