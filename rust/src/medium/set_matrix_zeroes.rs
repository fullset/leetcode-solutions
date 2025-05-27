// https://leetcode.com/problems/set-matrix-zeroes/

use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();
        let m = matrix[0].len();
        let mut cols = HashSet::with_capacity(n);
        let mut rows = HashSet::with_capacity(m);

        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == 0 {
                    if !rows.contains(&i) {
                        for k in 0..j {
                            matrix[i][k] = 0;
                        }
                        rows.insert(i);
                    }
                    if !cols.contains(&j) {
                        for k in 0..i {
                            matrix[k][j] = 0;
                        }
                        cols.insert(j);
                    }
                } else {
                    if rows.contains(&i) || cols.contains(&j) {
                        matrix[i][j] = 0;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut matrix = vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]];
        Solution::set_zeroes(&mut matrix);
        assert_eq!(matrix, vec![vec![1,0,1],vec![0,0,0],vec![1,0,1]]);

        let mut matrix2 = vec![vec![0,1,2,0],vec![3,4,5,2],vec![1,3,1,5]];
        Solution::set_zeroes(&mut matrix2);
        assert_eq!(matrix2, vec![vec![0,0,0,0],vec![0,4,5,0],vec![0,3,1,0]]);

        let mut matrix3 = vec![vec![-4,-2147483648,6,-7,0],vec![-8,6,-8,-6,0],vec![2147483647,2,-9,-6,-10]];
        Solution::set_zeroes(&mut matrix3);
        assert_eq!(matrix3, vec![vec![0,0,0,0,0],vec![0,0,0,0,0],vec![2147483647,2,-9,-6,0]]);
    }
}
