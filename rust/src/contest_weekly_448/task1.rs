// https://leetcode.com/contest/weekly-contest-448/problems/maximum-product-of-two-digits/

pub fn max_product(mut n: i32) -> i32 {
    let mut digits = Vec::with_capacity(10);
    let mut max = 0;

    while n > 0 {
        digits.push(n % 10);
        n = n / 10;
    }

    for i in 0..digits.len() {
        for j in i + 1..digits.len() {
            let product = digits[i] * digits[j];
            println!("{:?}", product);
            max = max.max(product);
        }
    }

    max
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(max_product(13), 3);
        assert_eq!(max_product(22), 4);
        assert_eq!(max_product(124), 8);
        assert_eq!(max_product(412), 8);
        assert_eq!(max_product(412980), 72);
        assert_eq!(max_product(412999), 81);
    }
}
