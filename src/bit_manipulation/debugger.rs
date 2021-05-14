// Solution to Exercise 5.5

/// The following code identifies whether a number is of the form 0..010..0 or is 0
/// i.e. is a power of 2
/// 
/// # Example:
/// ```
/// use ctci::bit_manipulation::debugger::debugger;
/// assert_eq!(true, debugger(1));  // 2^0
/// ```
pub fn debugger(n: i32) -> bool{
    n & (n-1) == 0
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_debugger(){
        assert_eq!(true, debugger(0));
        assert_eq!(true, debugger(1));  // 2^0
        assert_eq!(true, debugger(2));  // 2^1
        assert_eq!(true, debugger(4));  // 2^2
        assert_eq!(true, debugger(8));  // 2^3

        assert_eq!(false, debugger(3));
        assert_eq!(false, debugger(5));
        assert_eq!(false, debugger(6));
        assert_eq!(false, debugger(7));
    }
}