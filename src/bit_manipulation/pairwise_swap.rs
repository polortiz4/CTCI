// Solution to Exercise 5.7

/// Given a number, swap the odd and even bits.
/// 
/// # Examples:
/// 
/// ```
/// use ctci::bit_manipulation::pairwise_swap::pairwise_swap;
/// let a = 0b0101;
/// assert_eq!(0b1010, pairwise_swap(a));
/// ```
pub fn pairwise_swap(input: u32) -> u32{
    const ODD_MASK: u32 = 0b0101_0101_0101_0101_0101_0101_0101_0101;
    const EVEN_MASK: u32 = 0b1010_1010_1010_1010_1010_1010_1010_1010;
    let odds = ODD_MASK & input;
    let evens = input & EVEN_MASK;
    odds << 1 | evens >> 1
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_pairwise(){
        let a = 0b0101;
        assert_eq!(0b1010, pairwise_swap(a));
        assert_eq!(0b10, pairwise_swap(0b1));
        assert_eq!(0, pairwise_swap(0));
        assert_eq!(0b0110_1001, pairwise_swap(0b1001_0110));
    }
}