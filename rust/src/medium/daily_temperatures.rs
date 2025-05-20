pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut monotonic_stack = Vec::with_capacity(temperatures.len());
        let mut result = Vec::new();
        result.resize(temperatures.len(), 0);

        for i in (0..temperatures.len()).rev() {
            match monotonic_stack.last() {
                None => monotonic_stack.push(temperatures[i]),
                Some(value) => {
                    if *value > temperatures[i] {
                        result[i] = 1;
                        monotonic_stack.push(temperatures[i]);
                    } else {
                        let mut steps = 1;
                        while let Some(last) = monotonic_stack.last() {
                            if *last > temperatures[i] {
                                result[i] = steps;
                                monotonic_stack.push(temperatures[i]);
                                break;
                            } else {
                                steps += result[i + steps as usize];
                                monotonic_stack.pop().unwrap();
                            }
                        }

                        if monotonic_stack.is_empty() {
                            monotonic_stack.push(temperatures[i]);
                        }
                    }
                }
            };
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
            vec![1, 1, 4, 2, 1, 1, 0, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 40, 50, 60]),
            vec![1, 1, 1, 0]
        );
        assert_eq!(
            Solution::daily_temperatures(vec![30, 60, 90]),
            vec![1, 1, 0]
        );

        assert_eq!(
            Solution::daily_temperatures(vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70]),
            vec![8, 1, 5, 4, 3, 2, 1, 1, 0, 0]
        );
    }
}
