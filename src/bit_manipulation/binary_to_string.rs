// Solution to Exercise 5.2

/// Convert a double digit real number to a binary number
/// 
/// # Example
/// ```
/// use ctci::bit_manipulation::binary_to_string::binary_to_string;
/// assert_eq!(".10000000000000000000000000000000", binary_to_string(0.5));
/// 
/// ```
pub fn binary_to_string(mut x: f64) -> String{
    assert!(x < 1.0);
    assert!(x >= 0.0);
    let mut result = String::with_capacity(33);
    result.push('.');
    for _ in 0..32 {
        x = x * 2.0;
        result.push(if x >= 1.0 { x = x - 1.0; '1' } else { '0' });
    }
    if x != 0.0 {
        String::from("ERROR")
    } else {
        result
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn binary_test(){
        assert_eq!(".00000000000000000000000000000000", binary_to_string(0.0));
        assert_eq!(".10000000000000000000000000000000", binary_to_string(0.5));
        assert_eq!(".01000000000000000000000000000000", binary_to_string(0.25));
        assert_eq!(".00100000000000000000000000000000", binary_to_string(0.125));
        assert_eq!(".00010000000000000000000000000000", binary_to_string(0.0625));
        assert_eq!(".00000000000000000000000000000001", binary_to_string(2.3283064365386963e-10));
        assert_eq!("ERROR", binary_to_string(2.3283064365387e-10));
        assert_eq!("ERROR", binary_to_string(0.72));
    }

    #[test]
    #[should_panic]
    fn invalid_ranges(){
        binary_to_string(1.0);
    }

    #[test]
    #[should_panic]
    fn invalid_ranges_low(){
        binary_to_string(-0.01);
    }
}