// https://leetcode.com/problems/maximum-subarray/

struct Solution;

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max_value = nums[0];
        let mut max_sum = nums[0];
        let mut found_first_positive = false;

        let mut positive_sum = 0;
        let mut negative_sum = 0;

        let mut next_positive_sum = 0;

        for num in nums {
            if num > max_value {
                max_value = num;
            }

            if !found_first_positive && num < 0 {
                continue;
            } else {
                found_first_positive = true;
                max_sum = max_sum.max(0);
            }

            if num > 0 {
                if negative_sum == 0 {
                    positive_sum += num;
                } else {
                    next_positive_sum += num;
                }
            } else {
                if next_positive_sum == 0 {
                    negative_sum += num;
                } else {
                    max_sum = max_sum.max(positive_sum);

                    if negative_sum.abs() > positive_sum {
                        positive_sum = next_positive_sum;
                        negative_sum = num;
                        next_positive_sum = 0;
                    } else {
                        positive_sum += negative_sum + next_positive_sum;
                        negative_sum = num;
                        next_positive_sum = 0;
                    }
                }
            };
        }

        if max_sum < 0 {
            max_value
        } else {
            max_sum = max_sum.max(positive_sum);
            max_sum = max_sum.max(next_positive_sum);
            max_sum = max_sum.max(positive_sum + next_positive_sum + negative_sum);
            max_sum
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Solution::max_sub_array(vec![1]), 1);
        assert_eq!(Solution::max_sub_array(vec![1, 2, 3, 4]), 10);

        assert_eq!(Solution::max_sub_array(vec![-1, 1, 2, 3, 4]), 10);
        assert_eq!(Solution::max_sub_array(vec![-1, 1, 2, 3, 4, -3]), 10);

        assert_eq!(Solution::max_sub_array(vec![-1, 1, 2, -10, 3, 4, -3]), 7);
        assert_eq!(Solution::max_sub_array(vec![-1, 1, 2, -4, 3, 4, -3]), 7);
        assert_eq!(Solution::max_sub_array(vec![-1, 1, 2, -1, 3, 4, -3]), 9);
        assert_eq!(Solution::max_sub_array(vec![-1, 3, 4, -1, 1, 2, -3]), 9);
        assert_eq!(Solution::max_sub_array(vec![-1, 3, 4, -10, 1, 2, -3]), 7);
        assert_eq!(Solution::max_sub_array(vec![-1, 3, 4, -4, 1, 2, -3]), 7);

        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![5, 4, -1, 7, 8]), 23);
        assert_eq!(Solution::max_sub_array(vec![3, 2, -3, -1, 1, -3, 1, -1]), 5);
        assert_eq!(
            Solution::max_sub_array(vec![-2, 3, -3, 1, 1, -1, 1, 1, 1]),
            4
        );

        assert_eq!(
            Solution::max_sub_array(vec![-9, -2, 1, 8, 7, -6, 4, 9, -9, -5, 0, 5, -2, 5, 9, 7]),
            33
        );
    }
}
