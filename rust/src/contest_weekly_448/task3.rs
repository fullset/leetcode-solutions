use std::collections::HashSet;

pub fn get_time_i_merged(i: usize, mut position: Vec<i32>, mut time: Vec<i32>) -> i32 {
    time[i] = time[i] + time[i + 1];
    time.remove(i + 1);
    position.remove(i);

    println!("{:?}", position);
    println!("{:?}", time);

    let mut acc = 0;

    for i in 0..position.len() - 1 {
        acc += time[i] * (position[i + 1] - position[i]);
    }

    acc
}

pub fn min_travel_time(
    l: i32,
    mut n: i32,
    k: i32,
    mut position: Vec<i32>,
    mut time: Vec<i32>,
) -> i32 {
    let mut min = 0;
    for i in 0..position.len() - 1 {
        min += time[i] * (position[i + 1] - position[i]);
    }
    if k == 0 {
        return min;
    }

    min = i32::MAX;
    let mut i_mins = HashSet::new();
    let mut i_min = 0;

    for i in 1..n - 1 {
        let r = get_time_i_merged(i as usize, position.clone(), time.clone());
        if min >= r {
            i_mins.insert(i);
            min = r;
        }
        println!("{:?}:{:?}", i_min, min);
    }

    if i_mins.len() == 1{
        i_min = i_mins.clone().into_iter().next().unwrap();
    } else {
        for item in i_mins.iter() {
            let r = min_travel_time(l, n, k - 1, position.clone(), time.clone());
            if min > r {
                i_min = *item;
                min = r;
            }
        }
    }

    time[i_min as usize] = time[i_min as usize] + time[i_min as usize + 1];
    time.remove(i_min as usize + 1);
    position.remove(i_min as usize);
    n -= 1;

    if k == 1 {
        min
    } else {
        min_travel_time(l, n, k - 1, position, time)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            min_travel_time(10, 4, 1, vec![0, 3, 8, 10], vec![5, 8, 3, 6]),
            62
        );
        assert_eq!(
            min_travel_time(5, 5, 1, vec![0, 1, 2, 3, 5], vec![8, 3, 9, 3, 3]),
            34
        );
        assert_eq!(
            min_travel_time(5, 5, 2, vec![0, 1, 2, 3, 5], vec![8, 3, 9, 3, 3]),
            52
        );
        assert_eq!(
            min_travel_time(2, 2, 0, vec![0, 2], vec![1, 5]),
            2
        );
        // assert_eq!(
        //     min_travel_time(5, 5, 2, vec![0, 1, 2, 3, 5], vec![1, 2, 2, 1, 2]),
        //     9
        // );
    }
}
