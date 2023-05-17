use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
        // Combine both input vectors into a single, sorted vector
        let mut nums3 = nums1;
        nums3.append(&mut nums2);
        nums3.sort();
        
        // Find the length of the combined vector
        let len = nums3.len();
        
        if len % 2 == 0 {
            // If the length is even, return the average of the two middle elements
            let a = nums3[len / 2];
            let b = nums3[len / 2 - 1];
            (a + b) as f64 / 2.0
        } else {
            // If the length is odd, return the middle element
            nums3[len / 2] as f64
        }
    }
    
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        
        if n < 3 {
            return 0;
        }
        
        let mut max_height = vec![0; n];
        max_height[0] = height[0];
        
        for i in 1..n {
            max_height[i] = max_height[i - 1].max(height[i]);
        }
        
        let mut water = 0;
        let mut right_max = height[n - 1];
        
        for i in (0..n).rev() {
            right_max = right_max.max(height[i]);
            water += max_height[i].min(right_max) - height[i];
        }
        
        water
    }
    
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        
        for (i, num) in nums.iter().enumerate() {
            let diff = target - num;
            
            if let Some(j) = map.get(&diff) {
                return vec![*j as i32, i as i32];
            }
            
            map.entry(num).or_insert(i as i32);
        }
        
        // If we reach this point, no indices were found.
        vec![]
    }

    pub fn int_to_roman(num: i32) -> String {

        let mut num = num;
        let mut result = String::new();

        while num > 0 {
            if num >= 1000 {
                result.push('M');
                num -= 1000;
            } else if num >= 900 {
                result.push_str("CM");
                num -= 900;
            } else if num >= 500 {
                result.push('D');
                num -= 500;
            } else if num >= 400 {
                result.push_str("CD");
                num -= 400;
            } else if num >= 100 {
                result.push('C');
                num -= 100;
            } else if num >= 90 {
                result.push_str("XC");
                num -= 90;
            } else if num >= 50 {
                result.push('L');
                num -= 50;
            } else if num >= 40 {
                result.push_str("XL");
                num -= 40;
            } else if num >= 10 {
                result.push('X');
                num -= 10;
            } else if num >= 9 {
                result.push_str("IX");
                num -= 9;
            } else if num >= 5 {
                result.push('V');
                num -= 5;
            } else if num >= 4 {
                result.push_str("IV");
                num -= 4;
            } else {
                result.push('I');
                num -= 1;
            }
        }

        result
    }

    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut prev = 0;

        for c in s.chars() {
            let curr = match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                _ => 1_000,
            };

            result += curr;

            if curr > prev {
                result -= 2 * prev;
            }

            prev = curr;
        }

        result
    }

    pub fn is_palindrome(x: i32) -> bool {
        // Convert x to a string.
        let s = x.to_string();

        // Check if the string is equal to its reverse.
        s == s.chars().rev().collect::<String>()
    }
}

fn main() {}