// Med. Leetcode 227
// https://leetcode.com/problems/basic-calculator-ii/description/

use std::println;

fn main() {
    let s = "12-3*4".to_string();
    println!("{}", calculate(s));
}

pub fn calculate(s: String) -> i32 {
    let mut a = Parser::new(s);

    a.mul_and_div();
    a.result()
}

struct Parser {
    chars: Vec<char>,
    mut_and_div: Vec<(usize, usize, i32)>,
}

impl Parser {
    fn new(s: String) -> Self {
        Self {
            chars: s.chars().filter(|c| !c.is_whitespace()).collect(),
            mut_and_div: Vec::new(),
        }
    }
    // 2+2*2 0..2 4..6
    fn result(&self) -> i32 {
        let mut count = 0;
        let mut prijok = 0;
        let mut res = 0;
        let mut prev = '+';
        let mut s = String::new();

        for (i, ch) in self.chars.iter().enumerate() {
            if prijok != 0 {

                if i == self.chars.len() - 1 {
                    match prev {
                        '-' => res -= s.parse::<i32>().unwrap(),
                        '+' => res += s.parse::<i32>().unwrap(),
                        _ => panic!(),
                    }
                }

                prijok -= 1;
                continue;
            }
            if let Some(v) = self.mut_and_div.get(count) {
                if i == v.0 {
                    s = v.2.to_string();
                    
                    prijok = v.1 - v.0;
                    count += 1;
                    continue;
                }
            } 
            
            // блок обычного суммирования и вычитания
            if ch.is_digit(10) {
                s.push(*ch);
                
                if i == self.chars.len() - 1 {
                    match prev {
                        '-' => res -= s.parse::<i32>().unwrap(),
                        '+' => res += s.parse::<i32>().unwrap(),
                        _ => panic!(),
                    }
                }

            } else {
                match prev {
                    '-' => res -= s.parse::<i32>().unwrap(),
                    '+' => res += s.parse::<i32>().unwrap(),
                    _ => panic!(),
                }
                prev = *ch;
                s.clear();
            }
        }
        res
    }

    fn mul_and_div(&mut self) {
        if self.chars.contains(&'/') || self.chars.contains(&'*') {
            for (i, ch) in self.chars.iter().enumerate() {
                match ch {
                    '/' | '*' => {
                        if let Some(v) = self.mut_and_div.last() {
                            if i <= v.1 {
                                continue;
                            }
                        }
                        self.mut_and_div.push(self.mul_and_div_counter(i, *ch));
                    },
                    _ => continue,
                }
            }
        }
    }

    fn mul_and_div_counter(&self, i: usize, in_ch: char) -> (usize, usize, i32) {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut idx1 = 0;
        let mut idx2 = 0;

        for ch in (0..i).rev() {
            if self.chars[ch].is_digit(10) {
                num1.push(self.chars[ch]);
            } else {
                idx1 = ch + 1;
                break;
            }
        }

        let mut prev_ch = in_ch;
        let mut c_result = num1.chars().rev().collect::<String>().parse::<i32>().unwrap();
        // 3* 2*3
        for ch in i+1..self.chars.len() {
            if self.chars[ch] == '/' || self.chars[ch] == '*' {
                match prev_ch {
                    '*' => {c_result *= num2.parse::<i32>().unwrap(); prev_ch = self.chars[ch]},
                    '/' => {c_result /= num2.parse::<i32>().unwrap(); prev_ch = self.chars[ch]},
                    _ => panic!(),
                }

                num2.clear();
                continue;
            }
            if self.chars[ch].is_digit(10) {
                num2.push(self.chars[ch]);

                if ch == self.chars.len() - 1 { // 3*2 flag = false 2*3*4 flag = false
                    match prev_ch {
                        '*' => c_result *= num2.parse::<i32>().unwrap(),
                        '/' => c_result /= num2.parse::<i32>().unwrap(),
                        _ => panic!(),
                    }

                    idx2 = ch;
                }

            } else { // 3*2+2  3*2*4+5
                match prev_ch {
                    '*' => c_result *= num2.parse::<i32>().unwrap(),
                    '/' => c_result /= num2.parse::<i32>().unwrap(),
                    _ => panic!(),
                }

                idx2 = ch - 1;
                break;
            }
        }
        
        (idx1, idx2, c_result)
    }
}