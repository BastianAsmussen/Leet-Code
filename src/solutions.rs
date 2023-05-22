use std::cmp::Reverse;
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

    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        const INF: i32 = 0x7FFFFFFF; // 2 ^ 31 - 1

        // Create an adjacency list to represent the network
        let mut graph = vec![vec![]; n as usize + 1];
        for time in times {
            let u = time[0] as usize;
            let v = time[1] as usize;
            let w = time[2];
            graph[u].push((v, w));
        }

        // Initialize the distance array with infinity values
        let mut dist = vec![INF; n as usize + 1];

        // Create a priority queue to store nodes and their distances
        let mut pq = BinaryHeap::new();

        // Set the distance of the source node to 0 and push it into the priority queue
        dist[k as usize] = 0;
        pq.push(Reverse((0, k)));

        while let Some(Reverse((d, u))) = pq.pop() {
            // If the current distance is greater than the stored distance, skip
            if d > dist[u as usize] {
                continue;
            }

            // Iterate over neighbors of the current node
            for &(v, w) in &graph[u as usize] {
                let new_dist = d + w;

                // If the new distance is smaller, update the distance and push the node into the priority queue
                if new_dist < dist[v] {
                    dist[v] = new_dist;
                    pq.push(Reverse((new_dist, v as i32)));
                }
            }
        }

        // Find the maximum distance in the distance array
        let max_dist = dist.iter().skip(1).cloned().max().unwrap();

        if max_dist == INF {
            // If there is a node that cannot receive the signal, return -1
            -1
        } else {
            // Otherwise, return the maximum distance
            max_dist
        }
    }

    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let mut left = 0;
        let mut right = letters.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if letters[mid] <= target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        letters[left % letters.len()]
    }

    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut dp = vec![0; cost.len() + 1];

        for i in 2..dp.len() {
            dp[i] = std::cmp::min(dp[i - 1] + cost[i - 1], dp[i - 2] + cost[i - 2]);
        }

        dp[cost.len()]
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];

        let nums = nums;
        let n = nums.len() as i32;

        let mut indices = vec![0; n as usize];

        for i in 0..n {
            indices[i as usize] = i;
        }

        result.push(nums.clone());

        loop {
            let mut i = n - 1;

            while i > 0 && indices[i as usize - 1] >= indices[i as usize] {
                i -= 1;
            }

            if i == 0 {
                break;
            }

            let mut j = n - 1;

            while indices[j as usize] <= indices[i as usize - 1] {
                j -= 1;
            }

            indices.swap((i - 1) as usize, j as usize);

            j = n - 1;

            while i < j {
                indices.swap(i as usize, j as usize);
                i += 1;
                j -= 1;
            }

            let mut permutation = vec![0; n as usize];

            for i in 0..n {
                permutation[i as usize] = nums[indices[i as usize] as usize];
            }

            result.push(permutation);
        }

        result
    }

    pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res = Vec::new();
        let mut nums = nums;
        nums.sort(); // Sort the input numbers to handle duplicates

        let mut used = vec![false; nums.len()];
        let mut current = Vec::new();

        Self::backtrack(&nums, &mut used, &mut current, &mut res);

        res
    }

    fn backtrack(nums: &[i32], used: &mut Vec<bool>, current: &mut Vec<i32>, res: &mut Vec<Vec<i32>>) {
        if current.len() == nums.len() {
            res.push(current.clone());
            return;
        }

        for i in 0..nums.len() {
            if used[i] || (i > 0 && nums[i] == nums[i - 1] && !used[i - 1]) {
                continue;
            }

            used[i] = true;
            current.push(nums[i]);

            Self::backtrack(nums, used, current, res);

            used[i] = false;
            current.pop();
        }
    }
}

fn main() {}
