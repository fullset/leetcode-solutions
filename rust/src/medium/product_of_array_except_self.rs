// https://leetcode.com/problems/product-of-array-except-self/

struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut left_products = vec![1; nums.len()];
        let mut right_products = vec![1; nums.len()];
        let n = nums.len();

        for i in 1..n {
            left_products[i] = left_products[i - 1] * nums[i - 1];
            right_products[n - 1 - i] = right_products[n - 1 - (i - 1)] * nums[n - 1 - (i - 1)];
        }

        let result = (0..n)
            .map(|i| left_products[i] * right_products[i])
            .collect();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::product_except_self(vec![1, 2, 3, 4]),
            vec![24, 12, 8, 6]
        );
        assert_eq!(
            Solution::product_except_self(vec![-1, 1, 0, -3, 3]),
            vec![0, 0, 9, 0, 0]
        );
    }
}
