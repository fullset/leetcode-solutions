// https://leetcode.com/contest/weekly-contest-447/problems/count-covered-buildings/

use std::{cmp::Ordering, collections::HashMap};

pub struct Solution;

impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let mut x_map = HashMap::with_capacity(n.try_into().unwrap());
        let mut y_map = HashMap::with_capacity(n.try_into().unwrap());

        // It is guaranteed that building contains exactly 2 elements
        for building in &buildings {
            x_map
                .entry(building[0])
                .and_modify(|v: &mut Vec<i32>| {
                    v.push(building[1]);
                })
                .or_insert({
                    let mut v = Vec::with_capacity((n / 64).try_into().unwrap());
                    v.push(building[1]);
                    v
                });

            y_map
                .entry(building[1])
                .and_modify(|v: &mut Vec<i32>| {
                    v.push(building[0]);
                })
                .or_insert({
                    let mut v = Vec::with_capacity((n / 64).try_into().unwrap());
                    v.push(building[0]);
                    v
                });
        }

        buildings
            .into_iter()
            .filter(|x| x[0] > 1 && x[0] < n && x[1] > 0 && x[1] < n)
            .fold(0, |acc, building| {
                let mut found_above = false;
                let mut found_left = false;
                let mut found_below = false;
                let mut found_right = false;

                // O(n*n). Should be optimized :(
                for other in &x_map[&building[0]] {
                    match (*other).cmp(&building[1]) {
                        Ordering::Less => found_below = true,
                        Ordering::Greater => found_above = true,
                        _ => {}
                    }
                    if found_below && found_above {
                        break;
                    }
                }

                for other in &y_map[&building[1]] {
                    match (*other).cmp(&building[0]) {
                        Ordering::Less => found_left = true,
                        Ordering::Greater => found_right = true,
                        _ => {}
                    }
                    if found_left && found_right {
                        break;
                    }
                }

                if found_above && found_below && found_left && found_right {
                    acc + 1
                } else {
                    acc
                }
            })
    }
}

fn main() {
    println!("Hello, world!");

    // Runtime
    // 1519 ms
    // Beats 6.67%
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::count_covered_buildings(
                3,
                vec![vec![1, 1], vec![2, 2], vec![1, 2], vec![2, 1]]
            ),
            0
        );
        assert_eq!(
            Solution::count_covered_buildings(
                3,
                vec![vec![1, 2], vec![2, 2], vec![3, 2], vec![2, 1], vec![2, 3]]
            ),
            1
        );
        assert_eq!(
            Solution::count_covered_buildings(
                5,
                vec![vec![1, 3], vec![3, 2], vec![3, 3], vec![3, 5], vec![5, 3]]
            ),
            1
        );

        assert_eq!(
            Solution::count_covered_buildings(
                4,
                vec![
                    vec![2, 4],
                    vec![1, 2],
                    vec![3, 1],
                    vec![1, 4],
                    vec![2, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![1, 3]
                ]
            ),
            1
        );
    }
}
