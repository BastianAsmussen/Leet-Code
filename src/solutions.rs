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
}

fn main() {}