// Solution to Exercise 5.6

/// Given two integers, count the number of bits that would need to get flipped
///
/// # Examples:
/// ```
/// use ctci::bit_manipulation::conversion::conversion;
///
/// assert_eq!(4, conversion(10, 5));  // 1010 vs 0101
/// assert_eq!(3, conversion(11, 5));  // 1011 vs 0101
/// ```
pub fn conversion(i1: i32, i2: i32) -> u32 {
    let diff = i1 ^ i2;
    n_ones(diff)
}
fn n_ones(mut input: i32) -> u32 {
    let mut n_ones = 0;
    while input != 0 {
        n_ones += (input & 1) as u32;
        input = input >> 1;
    }
    n_ones
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_conversion() {
        let a = 10; // 1010
        let b = 5; //  0101
        assert_eq!(4, conversion(a, b));
        assert_eq!(1, conversion(0, 1));
        assert_eq!(2, conversion(1, 2));
        assert_eq!(1, conversion(3, 2));
        assert_eq!(0, conversion(0, 0));
        assert_eq!(2, conversion(29, 15));
    }

    #[test]
    fn test_bit(){
        let a = 0b001;
        let b = 0b010;
        assert_eq!(a, (b >> 1));
    }
}
