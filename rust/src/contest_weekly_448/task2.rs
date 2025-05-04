pub fn special_grid_with_initial(n: i32, initial: i32) -> Vec<Vec<i32>> {
    if n == 0 {
        return vec![vec![initial]];
    }

    let top_right = special_grid_with_initial(n - 1, initial);
    let bottom_right = special_grid_with_initial(n - 1, 4_i32.pow(n as u32 - 1) + initial);
    let bottom_left = special_grid_with_initial(n - 1, 4_i32.pow(n as u32 - 1) * 2 + initial);
    let top_left = special_grid_with_initial(n - 1, 4_i32.pow(n as u32 - 1) * 3 + initial);

    let mut result = Vec::with_capacity(2_i32.pow(n as u32) as usize);

    for i in 0..2_i32.pow(n as u32) / 2 {
        let row_i = top_left[i as usize]
            .iter()
            .cloned()
            .chain(top_right[i as usize].iter().cloned())
            .collect();
        result.push(row_i);
    }

    for i in 0..2_i32.pow(n as u32) / 2 {
        let row_i = bottom_left[i as usize]
            .iter()
            .cloned()
            .chain(bottom_right[i as usize].iter().cloned())
            .collect();
        result.push(row_i);
    }

    result
}

pub fn special_grid(n: i32) -> Vec<Vec<i32>> {
    if n == 0 {
        return special_grid_with_initial(0, 0);
    }

    let top_right = special_grid_with_initial(n - 1, 0);
    let bottom_right = special_grid_with_initial(n - 1, 4_i32.pow(n as u32 - 1));
    let bottom_left = special_grid_with_initial(n - 1, 4_i32.pow(n as u32 - 1) * 2);
    let top_left = special_grid_with_initial(n - 1, 4_i32.pow(n as u32 - 1) * 3);

    let mut result = Vec::with_capacity(2_i32.pow(n as u32) as usize);

    for i in 0..2_i32.pow(n as u32) / 2 {
        let row_i = top_left[i as usize]
            .iter()
            .cloned()
            .chain(top_right[i as usize].iter().cloned())
            .collect();
        result.push(row_i);
    }

    for i in 0..2_i32.pow(n as u32) / 2 {
        let row_i = bottom_left[i as usize]
            .iter()
            .cloned()
            .chain(bottom_right[i as usize].iter().cloned())
            .collect();
        result.push(row_i);
    }

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", special_grid(0));
        println!("{:?}", special_grid(1));
        println!("{:?}", special_grid(2));
    }
}
