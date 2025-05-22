// https://leetcode.com/problems/kth-largest-element-in-an-array/



struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
        let mut heap: BinaryHeap<i32> = nums.into_iter().collect();
        for i in 1..k{
            heap.pop();
        }

        *heap.peek().unwrap()

        // let sorted = heap.into_sorted_vec();
        // *sorted.get(sorted.len() - (k as usize)).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::find_kth_largest(vec![3,2,1,5,6,4], 2), 5);
        assert_eq!(Solution::find_kth_largest(vec![3,2,3,1,2,4,5,5,6], 4), 4);
    }
}