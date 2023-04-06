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
}

fn main() {}